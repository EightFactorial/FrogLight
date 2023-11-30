use bevy::{log::error, prelude::debug, utils::HashMap};
use mc_rs_core::{resources::player::username::Username, PingResponse, StatusResponse};
use mc_rs_protocol::{
    types::{enums::ConnectionIntent, GameProfile},
    versions::{
        state::{Handshake, Login, Status},
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
        // Send the handshake packet
        conn.send_packet(ServerboundHandshakePacket {
            protocol_version: Self::ID,
            hostname: conn.hostname.clone(),
            port: conn.port,
            intention,
        })
        .await?;

        // Return the connection
        Ok(conn)
    }

    async fn status_handle(
        mut conn: Connection<Self, Status>,
    ) -> Result<(StatusResponse, PingResponse), ConnectionError> {
        // Send the status request
        conn.send_packet(ServerboundQueryRequestPacket).await?;

        // Receive the status response
        let ClientboundStatusPackets::QueryResponse(status_packet) = conn.receive_packet().await?
        else {
            error!("Expected status response, got something else");
            return Err(ConnectionError::UnexpectedPacket);
        };

        // Convert the player list into a hashmap
        let mut sample_players = HashMap::with_capacity(status_packet.players.sample.len());
        for player in status_packet.players.sample {
            sample_players.insert(player.name, player.uuid);
        }

        // Create the status response event
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

        // Send the ping request
        conn.send_packet(ServerboundQueryPingPacket::unix_epoch())
            .await?;

        // Receive the ping response
        let ClientboundStatusPackets::QueryPong(ping_packet) = conn.receive_packet().await? else {
            error!("Expected ping response, got something else");
            return Err(ConnectionError::UnexpectedPacket);
        };

        // Create the ping response event
        let ping = PingResponse {
            hostname: conn.hostname.clone(),
            time: *ping_packet,
        };

        // Return the status and ping response events
        Ok((status, ping))
    }

    async fn login_handle(
        username: Username,
        mut conn: Connection<Self, Login>,
    ) -> Result<(Connection<Self, Login>, GameProfile), ConnectionError> {
        // Send the login hello packet
        // TODO: Get the player's uuid
        conn.send_packet(ServerboundLoginHelloPacket {
            username: username.clone().into(),
            uuid: None,
        })
        .await?;

        // Complete the login process
        let profile = loop {
            match conn.receive_packet().await? {
                // TODO: Handle encryption
                ClientboundLoginPackets::LoginHello(p) => {
                    debug!("Received login encryption packet: {p:?}");
                }
                // Return the game profile
                ClientboundLoginPackets::LoginSuccess(p) => {
                    break p.into();
                }
                // Set the compression threshold
                ClientboundLoginPackets::LoginCompression(p) => {
                    conn.compression = Some(*p);
                }
                // TODO: Handle login queries
                ClientboundLoginPackets::LoginQueryRequest(p) => {
                    debug!("Received login query: {p:?}");
                }
                // Disconnect if the server sends a disconnect packet
                ClientboundLoginPackets::LoginDisconnect(p) => {
                    return Err(ConnectionError::Disconnected(p.reason));
                }
            }
        };

        // Return the connection and game profile
        Ok((conn, profile))
    }
}
