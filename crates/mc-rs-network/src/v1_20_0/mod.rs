use bevy::utils::HashMap;
use log::{debug, error};
use mc_rs_core::{PingResponse, StatusResponse};
use mc_rs_protocol::{
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

use super::handle::NetworkHandle;

mod network;

impl NetworkHandle for V1_20_0 {
    async fn handshake_handle(
        mut conn: Connection<Self, Handshake>,
        intention: ConnectionIntent,
    ) -> Result<Connection<Self, Handshake>, ConnectionError> {
        conn.send_packet(ServerboundHandshakePacket {
            protocol_version: Self::ID,
            hostname: conn.hostname.clone(),
            port: conn.port,
            intention,
        })
        .await?;

        Ok(conn)
    }

    async fn status_handle(
        mut conn: Connection<Self, Status>,
    ) -> Result<(StatusResponse, PingResponse), ConnectionError> {
        // Get the status
        conn.send_packet(ServerboundQueryRequestPacket {}).await?;
        let ClientboundStatusPackets::QueryResponse(status_packet) = conn.receive_packet().await?
        else {
            error!("Expected status response, got something else");
            return Err(ConnectionError::UnexpectedPacket);
        };

        let mut sample_players = HashMap::with_capacity(status_packet.players.sample.len());
        for player in status_packet.players.sample {
            sample_players.insert(player.name, player.uuid);
        }

        let status = StatusResponse {
            hostname: conn.hostname.clone(),
            description: status_packet.description,
            favicon: status_packet.favicon,
            player_max: status_packet.players.max,
            player_online: status_packet.players.online,
            sample_players,
            version: status_packet.version.name,
            protocol: status_packet.version.protocol,
        };

        // Get the ping
        conn.send_packet(ServerboundQueryPingPacket::unix_epoch())
            .await?;
        let ClientboundStatusPackets::QueryPong(ping_packet) = conn.receive_packet().await? else {
            error!("Expected ping response, got something else");
            return Err(ConnectionError::UnexpectedPacket);
        };

        let ping = PingResponse {
            hostname: conn.hostname.clone(),
            time: *ping_packet,
        };

        Ok((status, ping))
    }

    async fn login_handle(
        mut conn: Connection<Self, Login>,
    ) -> Result<(Connection<Self, Login>, GameProfile), ConnectionError> {
        conn.send_packet(ServerboundLoginHelloPacket {
            username: "MC-RS".to_string(),
            uuid: None,
        })
        .await?;

        let profile = loop {
            match conn.receive_packet().await? {
                ClientboundLoginPackets::LoginHello(p) => {
                    debug!("Received login encryption packet: {p:?}");
                }
                ClientboundLoginPackets::LoginSuccess(p) => {
                    break p.into();
                }
                ClientboundLoginPackets::LoginCompression(p) => {
                    conn.compression = Some(*p);
                }
                ClientboundLoginPackets::LoginQueryRequest(p) => {
                    debug!("Received login query: {p:?}");
                }
                ClientboundLoginPackets::LoginDisconnect(p) => {
                    return Err(ConnectionError::Disconnected(p.reason));
                }
            }
        };

        Ok((conn, profile))
    }
}
