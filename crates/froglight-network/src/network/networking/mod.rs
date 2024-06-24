use std::{net::SocketAddr, time::Duration};

use bevy_tasks::IoTaskPool;
use compact_str::ToCompactString;
use froglight_protocol::{
    common::ConnectionIntent,
    packet::ServerStatus,
    states::{Configuration, Handshake, Login, Play, Status},
    traits::{State, Version},
};
use handshake::HandshakeState;
use status::StatusState;

use super::{channel::ConnectionTaskChannel, ConnectionChannel, ConnectionTask, StatusTask};
use crate::connection::{Connection, ConnectionError, NetworkDirection, Serverbound};

mod configuration;
mod handshake;
mod login;
mod play;
mod status;

/// A trait for creating connections to servers.
pub trait ConnectionTrait
where
    Self: Version,
    Handshake: State<Self>,
    Status: State<Self>,
    Login: State<Self>,
    Configuration: State<Self>,
    Play: State<Self>,
    Serverbound: NetworkDirection<Self, Handshake>
        + NetworkDirection<Self, Status>
        + NetworkDirection<Self, Login>
        + NetworkDirection<Self, Configuration>
        + NetworkDirection<Self, Play>,
{
    /// Connect and login to a server.
    ///
    /// Requires a [`Resolver`](crate::resolver::Resolver) to resolve the
    /// address.
    #[must_use]
    #[cfg(feature = "resolver")]
    fn connect(
        address: &str,
        resolver: &crate::resolver::Resolver,
    ) -> (ConnectionTask, ConnectionChannel<Self>);

    /// Request the status of a server.
    ///
    /// Requires a [`Resolver`](crate::resolver::Resolver) to resolve the
    /// address.
    #[must_use]
    #[cfg(feature = "resolver")]
    fn status(address: &str, resolver: &crate::resolver::Resolver) -> StatusTask;

    /// Connect and login to a server.
    #[must_use]
    fn connect_to(
        socket: SocketAddr,
        address: Option<&str>,
    ) -> (ConnectionTask, ConnectionChannel<Self>);

    /// Request the status of a server.
    #[must_use]
    fn status_of(socket: SocketAddr, address: Option<&str>) -> StatusTask;
}

impl<V> ConnectionTrait for V
where
    V: Version + HandshakeState + StatusState,
    Handshake: State<V>,
    Status: State<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
    Serverbound: NetworkDirection<V, Handshake>
        + NetworkDirection<V, Status>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
{
    #[cfg(feature = "resolver")]
    fn connect(
        address: &str,
        resolver: &crate::resolver::Resolver,
    ) -> (ConnectionTask, ConnectionChannel<Self>) {
        let (channel, task_channel) = ConnectionChannel::<Self>::new();

        let address = address.to_compact_string();
        let resolver = resolver.clone();

        let task = IoTaskPool::get().spawn(async move {
            let conn = Connection::<Self, Handshake>::connect_to(&address, &resolver).await?;
            perform_server_connection(conn, task_channel).await
        });

        (ConnectionTask::new::<Self>(task), channel)
    }

    #[cfg(feature = "resolver")]
    fn status(address: &str, resolver: &crate::resolver::Resolver) -> StatusTask {
        let address = address.to_compact_string();
        let resolver = resolver.clone();

        let task = IoTaskPool::get().spawn(async move {
            let conn = Connection::<Self, Handshake>::connect_to(&address, &resolver).await?;
            perform_status_request(conn).await
        });

        StatusTask::new::<Self>(task)
    }

    fn connect_to(
        socket: SocketAddr,
        address: Option<&str>,
    ) -> (ConnectionTask, ConnectionChannel<Self>) {
        let (channel, task_channel) = ConnectionChannel::<Self>::new();

        let address = address.map(|addr| addr.to_compact_string());

        let task = IoTaskPool::get().spawn(async move {
            let mut conn = Connection::<Self, Handshake>::connect(socket).await?;
            conn.info.address = address;
            perform_server_connection(conn, task_channel).await
        });

        (ConnectionTask::new::<Self>(task), channel)
    }

    fn status_of(socket: SocketAddr, address: Option<&str>) -> StatusTask {
        let address = address.map(|addr| addr.to_compact_string());

        let task = IoTaskPool::get().spawn(async move {
            let mut conn = Connection::<Self, Handshake>::connect(socket).await?;
            conn.info.address = address;
            perform_status_request(conn).await
        });

        StatusTask::new::<Self>(task)
    }
}

async fn perform_status_request<V>(
    mut conn: Connection<V, Handshake>,
) -> Result<(ServerStatus, Duration), ConnectionError>
where
    V: Version + HandshakeState + StatusState,
    Handshake: State<V>,
    Status: State<V>,
    Serverbound: NetworkDirection<V, Handshake> + NetworkDirection<V, Status>,
{
    conn = V::perform_handshake(conn, ConnectionIntent::Status).await?;
    V::perform_status_request(conn.status()).await
}

async fn perform_server_connection<V>(
    conn: Connection<V, Handshake>,
    _task_channel: ConnectionTaskChannel<V, Serverbound>,
) -> Result<(), ConnectionError>
where
    V: Version + HandshakeState,
    Handshake: State<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
    Serverbound: NetworkDirection<V, Handshake>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
{
    let _conn = V::perform_handshake(conn, ConnectionIntent::Login).await?;
    todo!();
}
