use froglight_protocol::{
    packet::ServerStatus,
    states::Status,
    versions::v1_20_3::{
        status::{QueryPingC2SPacket, QueryRequestC2SPacket, StatusClientboundPackets},
        V1_20_3,
    },
};

use crate::connection::{Connection, ConnectionError};

impl super::StatusHandler for V1_20_3 {
    async fn version_status_request(
        conn: &mut Connection<Self, Status>,
    ) -> Result<ServerStatus, ConnectionError> {
        // Send the query request packet
        conn.send(QueryRequestC2SPacket).await?;

        // Wait for the response
        match conn.recv().await? {
            StatusClientboundPackets::QueryResponse(packet) => Ok(packet.status),
            StatusClientboundPackets::PingResult(pong) => {
                Err(ConnectionError::UnexpectedPacket(std::any::type_name_of_val(&pong)))
            }
        }
    }

    async fn version_ping_request(
        conn: &mut Connection<Self, Status>,
        payload: u64,
    ) -> Result<u64, ConnectionError> {
        // Send the ping request packet
        conn.send(QueryPingC2SPacket { time: payload }).await?;

        // Wait for the response
        match conn.recv().await? {
            StatusClientboundPackets::PingResult(packet) => Ok(packet.time),
            StatusClientboundPackets::QueryResponse(response) => {
                Err(ConnectionError::UnexpectedPacket(std::any::type_name_of_val(&response)))
            }
        }
    }
}
