use bevy_reflect::TypePath;
use froglight_core::common::ConnectionIntent;
use froglight_protocol::{
    states::{Handshaking, Status},
    traits::Version,
    versions::v1_20_0::{
        handshaking::HandshakeC2SPacket,
        status::{
            QueryPingC2SPacket, QueryPongS2CPacket, QueryRequestC2SPacket, StatusClientboundPackets,
        },
        V1_20_0,
    },
};

use super::Queryable;
use crate::{
    status::{PingRequest, PingResponse, StatusRequest, StatusResponse},
    Connection, ConnectionError,
};

impl Queryable for V1_20_0 {
    async fn handshake(
        url: &str,
        port: u16,
        connection: &mut Connection<Self, Handshaking>,
    ) -> Result<(), ConnectionError> {
        let intent = HandshakeC2SPacket {
            protocol_version: V1_20_0::PROTOCOL_VERSION,
            hostname: url.into(),
            port,
            intention: ConnectionIntent::Status,
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
                    StatusResponse { entity: event.entity, url: event.url, status: status.0 };

                Ok(response)
            }
            StatusClientboundPackets::QueryPong(_) => {
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
            StatusClientboundPackets::QueryPong(pong) => {
                let response = PingResponse { entity: event.entity, url: event.url, time: pong.0 };

                Ok(response)
            }
            StatusClientboundPackets::QueryResponse(_) => {
                Err(ConnectionError::UnexpectedPacket(QueryPongS2CPacket::type_path()))
            }
        }
    }
}
