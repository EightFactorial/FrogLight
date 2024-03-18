use bevy_reflect::TypePath;
use compact_str::CompactString;
use froglight_core::common::ConnectionIntent;
use froglight_protocol::{
    states::{Handshaking, Status},
    traits::Version,
    versions::v1_20_2::{
        handshaking::HandshakeC2SPacket,
        status::{
            PingResultS2CPacket, QueryPingC2SPacket, QueryRequestC2SPacket,
            StatusClientboundPackets,
        },
        V1_20_2,
    },
};

use super::Queryable;
use crate::{
    status::{PingRequest, PingResponse, StatusRequest, StatusResponse},
    Connection, ConnectionError,
};

impl Queryable for V1_20_2 {
    async fn handshake(
        connection: &mut Connection<Self, Handshaking>,
    ) -> Result<(), ConnectionError> {
        let (hostname, port) = match connection.info.as_ref() {
            Some(info) => (info.address.clone().unwrap_or_default(), info.socket.port()),
            None => (CompactString::default(), 25565),
        };

        let intent = HandshakeC2SPacket {
            protocol_version: V1_20_2::PROTOCOL_VERSION,
            intention: ConnectionIntent::Status,
            hostname,
            port,
        };
        connection.send(intent).await?;

        Ok(())
    }

    async fn status(
        event: StatusRequest<Self>,
        connection: &mut Connection<Self, Status>,
    ) -> Result<StatusResponse, ConnectionError> {
        connection.send(QueryRequestC2SPacket).await?;

        match connection.recv().await? {
            StatusClientboundPackets::QueryResponse(status) => {
                let response =
                    StatusResponse { entity: event.entity, url: event.address, status: status.0 };

                Ok(response)
            }
            StatusClientboundPackets::PingResult(_) => {
                Err(ConnectionError::UnexpectedPacket(QueryRequestC2SPacket::type_path()))
            }
        }
    }

    async fn ping(
        event: PingRequest<Self>,
        connection: &mut Connection<Self, Status>,
    ) -> Result<PingResponse, ConnectionError> {
        connection.send(QueryPingC2SPacket::unix_epoch()).await?;

        match connection.recv().await? {
            StatusClientboundPackets::PingResult(pong) => {
                let response =
                    PingResponse { entity: event.entity, url: event.address, time: pong.0 };

                Ok(response)
            }
            StatusClientboundPackets::QueryResponse(_) => {
                Err(ConnectionError::UnexpectedPacket(PingResultS2CPacket::type_path()))
            }
        }
    }
}
