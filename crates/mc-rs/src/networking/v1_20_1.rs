use async_trait::async_trait;
use flume::Receiver;
use mc_rs_proto::{
    types::enums::ConnectionIntent,
    versions::{state::*, v1_20_1::V1_20_1},
    Connection, ConnectionError,
};

use super::{
    handle::{ConnectionData, NetworkHandle},
    network::Network,
    request::{PingResponse, StatusResponse},
};

impl Network for V1_20_1 {}

#[async_trait]
impl NetworkHandle for V1_20_1 {
    async fn handshake_handle(
        _con: Connection<Self, Handshake>,
        _intent: ConnectionIntent,
    ) -> Result<Connection<Self, Handshake>, ConnectionError> {
        todo!()
    }

    async fn status_handle(
        _con: Connection<Self, Status>,
    ) -> Result<(StatusResponse, PingResponse), ConnectionError> {
        todo!()
    }

    async fn login_handle(
        _con: Connection<Self, Login>,
    ) -> Result<Connection<Self, Login>, ConnectionError> {
        todo!()
    }

    async fn configuration_handle(
        _con: Connection<Self, Configuration>,
    ) -> Result<Connection<Self, Configuration>, ConnectionError> {
        todo!()
    }

    async fn play_handle(_con: Connection<Self, Play>) -> Receiver<ConnectionData<Self>> { todo!() }
}
