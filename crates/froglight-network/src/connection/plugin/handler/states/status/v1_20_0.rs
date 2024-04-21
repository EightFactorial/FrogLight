use std::time::{Duration, Instant};

use froglight_protocol::{
    packet::ServerStatus,
    states::Status,
    versions::v1_20_0::{
        status::{QueryPingC2SPacket, QueryRequestC2SPacket, StatusClientboundPackets},
        V1_20_0,
    },
};

use crate::connection::{Connection, ConnectionError};

impl super::StatusHandler for V1_20_0 {
    async fn perform_status_request(
        conn: &mut Connection<Self, Status>,
    ) -> Result<ServerStatus, ConnectionError> {
        // Send the query request packet
        conn.send(QueryRequestC2SPacket).await?;

        // Wait for the response
        match conn.recv().await? {
            StatusClientboundPackets::QueryResponse(packet) => Ok(packet.status),
            StatusClientboundPackets::QueryPong(other) => {
                Err(ConnectionError::UnexpectedPacket(std::any::type_name_of_val(&other)))
            }
        }
    }

    async fn perform_ping_request(
        conn: &mut Connection<Self, Status>,
        payload: u64,
    ) -> Result<Duration, ConnectionError> {
        // Send the ping request packet
        conn.send(QueryPingC2SPacket { time: payload }).await?;

        // Get the current time
        let now = Instant::now();

        // Wait for the response
        match conn.recv().await? {
            StatusClientboundPackets::QueryPong(_) => Ok(now.elapsed()),
            StatusClientboundPackets::QueryResponse(other) => {
                Err(ConnectionError::UnexpectedPacket(std::any::type_name_of_val(&other)))
            }
        }
    }
}
