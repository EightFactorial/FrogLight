use std::net::SocketAddr;

use bevy_tasks::IoTaskPool;
use compact_str::ToCompactString;
use froglight_protocol::{
    states::{Configuration, Handshake, Login, Play, Status},
    traits::{State, Version},
};

use super::{BevyConnectionChannel, ConnectionTask, StatusTask};
use crate::connection::{Connection, NetworkDirection, Serverbound};

mod play;
use play::{perform_server_connection, PerformServerConnection};

mod status;
use status::{perform_status_request, PerformStatusRequest};

/// A trait for performing connections and status requests to servers.
pub trait ConnectTrait
where
    Self: Version,
    Serverbound: NetworkDirection<Self, Handshake>
        + NetworkDirection<Self, Status>
        + NetworkDirection<Self, Login>
        + NetworkDirection<Self, Configuration>
        + NetworkDirection<Self, Play>,
    Handshake: State<Self>,
    Status: State<Self>,
    Login: State<Self>,
    Configuration: State<Self>,
    Play: State<Self>,
{
    /// Connect and login to a server.
    ///
    /// Requires a [`Resolver`](crate::resolver::Resolver) to resolve the
    /// provided address.
    #[must_use]
    #[cfg(feature = "resolver")]
    fn connect(
        address: &str,
        resolver: &crate::resolver::Resolver,
    ) -> (BevyConnectionChannel<Self, Serverbound>, ConnectionTask);

    /// Connect and login to a server.
    ///
    /// If no address is provided, the connection will use the provided socket
    /// as the address.
    #[must_use]
    fn connect_to(
        socket: SocketAddr,
        address: Option<&str>,
    ) -> (BevyConnectionChannel<Self, Serverbound>, ConnectionTask);

    /// Request the status of a server.
    ///
    /// Requires a [`Resolver`](crate::resolver::Resolver) to resolve the
    /// provided address.
    #[must_use]
    #[cfg(feature = "resolver")]
    fn status(address: &str, resolver: &crate::resolver::Resolver) -> StatusTask;

    /// Request the status of a server.
    ///
    /// If no address is provided, the connection will use the provided socket
    /// as the address.
    #[must_use]
    fn status_of(socket: SocketAddr, address: Option<&str>) -> StatusTask;
}

impl<V> ConnectTrait for V
where
    V: Version + PerformStatusRequest + PerformServerConnection,
    Serverbound: NetworkDirection<V, Handshake>
        + NetworkDirection<V, Status>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
    Handshake: State<V>,
    Status: State<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    #[cfg(feature = "resolver")]
    fn connect(
        address: &str,
        resolver: &crate::resolver::Resolver,
    ) -> (BevyConnectionChannel<Self, Serverbound>, ConnectionTask) {
        let task_address = address.to_compact_string();
        let resolver = resolver.clone();

        let (bevy, task) = super::channel::channel();

        let task = IoTaskPool::get().spawn(async move {
            match Connection::<Self, Handshake>::connect_to(&task_address, &resolver).await {
                Ok(conn) => {
                    if let Err(err) = perform_server_connection(conn, &task).await {
                        let _ = task.send_error(err).await;
                    }
                }
                Err(err) => {
                    let _ = task.send_error(err).await;
                }
            }
        });

        (bevy, ConnectionTask::new::<Self>(address, task))
    }

    fn connect_to(
        socket: SocketAddr,
        address: Option<&str>,
    ) -> (BevyConnectionChannel<Self, Serverbound>, ConnectionTask) {
        let task_address = address.map(|addr| addr.to_compact_string());
        let address = task_address.clone().unwrap_or(socket.to_compact_string());

        let (bevy, task) = super::channel::channel();

        let task = IoTaskPool::get().spawn(async move {
            match Connection::<Self, Handshake, Serverbound>::connect(socket).await {
                Ok(mut conn) => {
                    conn.info_mut().address = task_address;
                    if let Err(err) = perform_server_connection(conn, &task).await {
                        let _ = task.send_error(err).await;
                    }
                }
                Err(err) => {
                    let _ = task.send_error(err).await;
                }
            }
        });

        (bevy, ConnectionTask::new::<Self>(address, task))
    }

    #[cfg(feature = "resolver")]
    fn status(address: &str, resolver: &crate::resolver::Resolver) -> StatusTask {
        let task_address = address.to_compact_string();
        let resolver = resolver.clone();

        let task = IoTaskPool::get().spawn(async move {
            match Connection::<Self, Handshake>::connect_to(&task_address, &resolver).await {
                Ok(conn) => perform_status_request(conn).await,
                Err(err) => Err(err),
            }
        });

        StatusTask::new::<Self>(address, task)
    }

    fn status_of(socket: SocketAddr, address: Option<&str>) -> StatusTask {
        let task_address = address.map(|addr| addr.to_compact_string());
        let address = task_address.clone().unwrap_or(socket.to_compact_string());

        let task = IoTaskPool::get().spawn(async move {
            match Connection::<Self, Handshake, Serverbound>::connect(socket).await {
                Ok(conn) => perform_status_request(conn).await,
                Err(err) => Err(err),
            }
        });

        StatusTask::new::<Self>(address, task)
    }
}
