use std::{future::join, sync::Arc};

use async_trait::async_trait;
use bevy::utils::HashMap;
use flume::{Receiver, Sender};
use futures_locks::Mutex;
use log::{debug, error};
use mc_rs_proto::{
    types::{enums::ConnectionIntent, GameProfile},
    versions::{
        state::*,
        v1_20_0::{
            handshake::serverboundhandshakepacket::ServerboundHandshakePacket,
            login::{
                serverboundloginhellopacket::ServerboundLoginHelloPacket, ClientboundLoginPackets,
            },
            status::{
                serverboundquerypingpacket::ServerboundQueryPingPacket,
                serverboundqueryrequestpacket::ServerboundQueryRequestPacket,
                ClientboundStatusPackets,
            },
            V1_20_0,
        },
    },
    Connection, ConnectionError, Version,
};

use super::{
    handle::{ConnectionData, ConnectionEnum, ConnectionSend, NetworkHandle},
    request::{PingResponse, StatusResponse},
};

mod network;
mod palette;

#[async_trait]
impl NetworkHandle for V1_20_0 {
    async fn handshake_handle(
        mut con: Connection<Self, Handshake>,
        intention: ConnectionIntent,
    ) -> Result<Connection<Self, Handshake>, ConnectionError> {
        con.send_packet(ServerboundHandshakePacket {
            protocol_version: V1_20_0::ID,
            hostname: con.hostname.clone(),
            port: con.port,
            intention,
        })
        .await?;

        Ok(con)
    }

    async fn status_handle(
        mut con: Connection<Self, Status>,
    ) -> Result<(StatusResponse, PingResponse), ConnectionError> {
        // Get the status
        con.send_packet(ServerboundQueryRequestPacket {}).await?;
        let ClientboundStatusPackets::QueryResponse(status_packet) = con.receive_packet().await?
        else {
            error!("Expected status response, got something else");
            return Err(ConnectionError::UnexpectedPacket);
        };

        let mut sample_players = HashMap::with_capacity(status_packet.players.sample.len());
        for player in status_packet.players.sample {
            sample_players.insert(player.name, player.uuid);
        }

        let status = StatusResponse {
            hostname: con.hostname.clone(),
            description: status_packet.description,
            favicon: status_packet.favicon,
            player_max: status_packet.players.max,
            player_online: status_packet.players.online,
            sample_players,
            version: status_packet.version.name,
            protocol: status_packet.version.protocol,
        };

        // Get the ping
        con.send_packet(ServerboundQueryPingPacket::default())
            .await?;
        let ClientboundStatusPackets::QueryPong(ping_packet) = con.receive_packet().await? else {
            error!("Expected ping response, got something else");
            return Err(ConnectionError::UnexpectedPacket);
        };

        let ping = PingResponse {
            hostname: con.hostname.clone(),
            time: ping_packet.time,
        };

        Ok((status, ping))
    }

    async fn login_handle(
        mut con: Connection<Self, Login>,
    ) -> Result<(Connection<Self, Login>, GameProfile), ConnectionError> {
        con.send_packet(ServerboundLoginHelloPacket {
            username: "MC-RS".to_string(),
            uuid: None,
        })
        .await?;

        let profile = loop {
            match con.receive_packet().await? {
                ClientboundLoginPackets::LoginHello(p) => {
                    debug!("Received login encryption packet: {p:?}");
                }
                ClientboundLoginPackets::LoginSuccess(p) => {
                    break p.profile;
                }
                ClientboundLoginPackets::LoginCompression(p) => {
                    con.compression = Some(p.threshold);
                }
                ClientboundLoginPackets::LoginQueryRequest(p) => {
                    debug!("Received login query: {p:?}");
                }
                ClientboundLoginPackets::LoginDisconnect(p) => {
                    return Err(ConnectionError::Disconnected(p.reason));
                }
            }
        };

        Ok((con, profile))
    }

    async fn configuration_handle(
        _con: Connection<Self, Configuration>,
    ) -> Result<Connection<Self, Configuration>, ConnectionError> {
        unreachable!("This version does not have a configuration state")
    }

    async fn play_handle(
        con: ConnectionEnum<Self>,
        tx: Sender<Result<ConnectionData<Self>, ConnectionError>>,
        rx: Receiver<ConnectionSend<Self>>,
    ) {
        let con = Arc::new(Mutex::new(con));
        join!(con_read(con.clone(), tx), con_write(con, rx)).await;
    }
}

/// Reads packets from the connection and sends them to the channel
async fn con_read(
    con: Arc<Mutex<ConnectionEnum<V1_20_0>>>,
    tx: Sender<Result<ConnectionData<V1_20_0>, ConnectionError>>,
) {
    loop {
        let mut con = con.lock().await;
        if tx.send_async(con.receive_packet().await).await.is_err() {
            error!("Failed to send packet to channel");
            return;
        }
    }
}

/// Writes packets from the channel to the connection
async fn con_write(
    con: Arc<Mutex<ConnectionEnum<V1_20_0>>>,
    rx: Receiver<ConnectionSend<V1_20_0>>,
) {
    loop {
        if let Ok(data) = rx.recv_async().await {
            let mut con = con.lock().await;
            if let Err(e) = con.send_packet(data).await {
                error!("Failed to send packet: {:?}", e);
                return;
            }
        } else {
            error!("Failed to receive packet from channel");
            return;
        }
    }
}
