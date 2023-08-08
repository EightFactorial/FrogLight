use async_trait::async_trait;
use flume::Receiver;
use mc_rs_proto::{
    types::enums::ConnectionIntent,
    versions::state::{Configuration, Handshake, Login, Play, Status},
    Connection, ConnectionError, State, Version,
};

use super::request::{PingResponse, StatusResponse};

#[async_trait]
pub trait NetworkHandle: Version + Send + Sync + 'static
where
    Handshake: State<Self>,
    Status: State<Self>,
    Login: State<Self>,
    Configuration: State<Self>,
    Play: State<Self>,
{
    /// Handle connections in the handshake state
    async fn handshake_handle(
        con: Connection<Self, Handshake>,
        intent: ConnectionIntent,
    ) -> Result<Connection<Self, Handshake>, ConnectionError>;

    /// Handle connections in the status state
    async fn status_handle(
        con: Connection<Self, Status>,
    ) -> Result<(StatusResponse, PingResponse), ConnectionError>;

    /// Handle connections in the login state
    async fn login_handle(
        con: Connection<Self, Login>,
    ) -> Result<Connection<Self, Login>, ConnectionError>;

    /// Handle connections in the configuration state
    async fn configuration_handle(
        con: Connection<Self, Configuration>,
    ) -> Result<Connection<Self, Configuration>, ConnectionError>;

    /// Handle connections in the play state
    async fn play_handle(con: Connection<Self, Play>) -> Receiver<ConnectionData<Self>>;
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ConnectionData<V: Version>
where
    Play: State<V>,
{
    Packet(<Play as State<V>>::Clientbound),
    Error(ConnectionError),
}
