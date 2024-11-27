use std::time::Duration;

use froglight_protocol::{
    common::ConnectionIntent,
    packet::ServerStatus,
    states::{Handshake, Status},
    traits::Version,
    versions::v1_21_0::{
        handshake::HandshakePacket,
        play::QueryPingPacket,
        status::{QueryRequestPacket, StatusClientboundPackets},
        V1_21_0,
    },
};

use super::PerformStatusRequest;
use crate::connection::{Connection, ConnectionError, Serverbound};

impl PerformStatusRequest for V1_21_0 {
    async fn status_request(
        mut conn: Connection<Self, Handshake, Serverbound>,
    ) -> Result<(Connection<Self, Status, Serverbound>, ServerStatus, Duration), ConnectionError>
    {
        // Send a Handshake to the server.
        conn.send(HandshakePacket {
            protocol: V1_21_0::ID,
            address: conn.info().get_address(),
            port: conn.info().get_port(),
            intent: ConnectionIntent::Status,
        })
        .await?;

        // Enter the Status state.
        let mut conn = conn.status();

        // Query the server for the status.
        conn.send(QueryRequestPacket).await?;
        let StatusClientboundPackets::QueryResponse(response) = conn.recv().await? else {
            return Err(ConnectionError::UnexpectedPacket("QueryResponse"));
        };

        // Time how long it takes to get a ping response.
        let instant = std::time::Instant::now();
        conn.send(QueryPingPacket::unix_epoch()).await?;
        let StatusClientboundPackets::PingResult(_pong) = conn.recv().await? else {
            return Err(ConnectionError::UnexpectedPacket("PingResult"));
        };

        Ok((conn, response.status, instant.elapsed()))
    }
}
