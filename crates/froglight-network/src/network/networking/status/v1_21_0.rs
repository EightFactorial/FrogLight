use std::time::{Duration, Instant};

use froglight_protocol::{
    packet::ServerStatus,
    states::Status,
    versions::v1_21_0::{
        status::{QueryPingPacket, QueryRequestPacket, StatusClientboundPackets},
        V1_21_0,
    },
};

use super::StatusState;
use crate::connection::{Connection, ConnectionError};

impl StatusState for V1_21_0 {
    async fn perform_status_request(
        mut conn: Connection<Self, Status>,
    ) -> Result<(ServerStatus, Duration), ConnectionError> {
        // Send a query request and read the response.
        conn.send(QueryRequestPacket).await?;
        let StatusClientboundPackets::QueryResponse(query_response) = conn.recv().await? else {
            return Err(ConnectionError::UnexpectedPacket("QueryResponse"));
        };

        // Send a ping request and read the response.
        let instant = Instant::now();
        conn.send(QueryPingPacket::unix_epoch()).await?;
        let StatusClientboundPackets::PingResult(_ping_result) = conn.recv().await? else {
            return Err(ConnectionError::UnexpectedPacket("PingResult"));
        };

        Ok((query_response.status, instant.elapsed()))
    }
}
