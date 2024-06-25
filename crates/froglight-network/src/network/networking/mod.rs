use std::{
    net::SocketAddr,
    sync::atomic::{AtomicU8, Ordering},
    time::Duration,
};

use bevy_log::error;
use bevy_tasks::IoTaskPool;
use compact_str::ToCompactString;
use configuration::ConfigurationState;
use froglight_protocol::{
    common::ConnectionIntent,
    packet::ServerStatus,
    states::{Configuration, Handshake, Login, Play, Status},
    traits::{State, Version},
};
use handshake::HandshakeState;
use login::LoginState;
use play::PlayState;
use status::StatusState;

use super::{
    channel::{ConnectionTaskChannel, PacketTaskChannel},
    ConnectionChannel, ConnectionTask, StatusTask,
};
use crate::connection::{
    Connection, ConnectionError, NetworkDirection, ReadConnection, Serverbound, WriteConnection,
};

mod configuration;
mod handshake;
mod login;
mod play;
mod status;

mod select;
use select::PacketFn;

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
    /// provided address.
    #[must_use]
    #[cfg(feature = "resolver")]
    fn connect(
        address: &str,
        resolver: &crate::resolver::Resolver,
    ) -> (ConnectionTask, ConnectionChannel<Self>);

    /// Request the status of a server.
    ///
    /// Requires a [`Resolver`](crate::resolver::Resolver) to resolve the
    /// provided address.
    #[must_use]
    #[cfg(feature = "resolver")]
    fn status(address: &str, resolver: &crate::resolver::Resolver) -> StatusTask;

    /// Connect and login to a server.
    ///
    /// If no address is provided, the connection will use the provided socket
    /// as the address.
    #[must_use]
    fn connect_to(
        socket: SocketAddr,
        address: Option<&str>,
    ) -> (ConnectionTask, ConnectionChannel<Self>);

    /// Request the status of a server.
    ///
    /// If no address is provided, the connection will use the provided socket
    /// as the address.
    #[must_use]
    fn status_of(socket: SocketAddr, address: Option<&str>) -> StatusTask;
}

impl<V> ConnectionTrait for V
where
    V: Version + HandshakeState + StatusState + LoginState + ConfigurationState + PlayState,
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
            conn.info_mut().address = address;
            perform_server_connection(conn, task_channel).await
        });

        (ConnectionTask::new::<Self>(task), channel)
    }

    fn status_of(socket: SocketAddr, address: Option<&str>) -> StatusTask {
        let address = address.map(|addr| addr.to_compact_string());

        let task = IoTaskPool::get().spawn(async move {
            let mut conn = Connection::<Self, Handshake>::connect(socket).await?;
            conn.info_mut().address = address;
            perform_status_request(conn).await
        });

        StatusTask::new::<Self>(task)
    }
}

/// Use a [`Connection`] to perform a status request.
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

/// An enum representing the different states of a [`Connection`].
#[derive(Debug)]
enum ConnEnum<V: Version>
where
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
    Serverbound:
        NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
{
    Login(ReadConnection<V, Login>, WriteConnection<V, Login>),
    Configuration(ReadConnection<V, Configuration>, WriteConnection<V, Configuration>),
    Play(ReadConnection<V, Play>, WriteConnection<V, Play>),
}

/// Use a [`Connection`] to connect to a server.
///
/// Relay packets between the channel and the connection.
async fn perform_server_connection<V>(
    conn: Connection<V, Handshake>,
    task_channel: ConnectionTaskChannel<V, Serverbound>,
) -> Result<(), ConnectionError>
where
    V: Version + HandshakeState + LoginState + ConfigurationState + PlayState,
    Handshake: State<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
    Serverbound: NetworkDirection<V, Handshake>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
{
    let conn = V::perform_handshake(conn, ConnectionIntent::Login).await?;
    let conn = V::perform_login(conn.login()).await?;

    let (read_conn, write_conn) = conn.into_split();
    let mut conn = ConnEnum::Login(read_conn, write_conn);

    // Main loop
    loop {
        let atomic_state = AtomicU8::new(0u8);
        match conn {
            // Login state
            ConnEnum::Login(mut read_conn, mut write_conn) => {
                let (read_res, write_res) = futures_lite::future::zip(
                    server_to_bevy(
                        &mut read_conn,
                        write_conn.clone(),
                        &task_channel.login,
                        &atomic_state,
                    ),
                    bevy_to_server(
                        &mut write_conn,
                        &task_channel.login,
                        V::login_acknowledged,
                        &atomic_state,
                    ),
                )
                .await;

                read_res?;
                write_res?;

                conn = ConnEnum::Configuration(read_conn.set_state(), write_conn.set_state());
            }

            // Configuration state
            ConnEnum::Configuration(mut read_conn, mut write_conn) => {
                let (read_res, write_res) = futures_lite::future::zip(
                    server_to_bevy(
                        &mut read_conn,
                        write_conn.clone(),
                        &task_channel.config,
                        &atomic_state,
                    ),
                    bevy_to_server(
                        &mut write_conn,
                        &task_channel.config,
                        V::config_acknowledged,
                        &atomic_state,
                    ),
                )
                .await;

                read_res?;
                write_res?;

                conn = ConnEnum::Play(read_conn.set_state(), write_conn.set_state());
            }
            // Play state
            ConnEnum::Play(mut read_conn, mut write_conn) => {
                let (read_res, write_res) = futures_lite::future::zip(
                    server_to_bevy(
                        &mut read_conn,
                        write_conn.clone(),
                        &task_channel.play,
                        &atomic_state,
                    ),
                    bevy_to_server(
                        &mut write_conn,
                        &task_channel.play,
                        V::play_acknowledged,
                        &atomic_state,
                    ),
                )
                .await;

                read_res?;
                write_res?;

                conn = ConnEnum::Configuration(read_conn.set_state(), write_conn.set_state());
            }
        }
    }
}

/// Relay packets from the connected server to Bevy.
async fn server_to_bevy<V, S>(
    read_conn: &mut ReadConnection<V, S, Serverbound>,
    write_conn: WriteConnection<V, S, Serverbound>,
    task_channel: &PacketTaskChannel<V, S, Serverbound>,
    atomic_state: &AtomicU8,
) -> Result<(), ConnectionError>
where
    V: Version + LoginState + ConfigurationState + PlayState,
    S: State<V> + PacketFn<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
    Serverbound: NetworkDirection<V, S>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
{
    let error;
    loop {
        // Read packets from the server
        let packet = match read_conn.recv().await {
            Ok(packet) => packet,
            Err(err) => {
                error = err;
                break;
            }
        };

        // Send the packet to Bevy and check if the state has changed
        let end_state = S::packet_fn(&packet, &write_conn).await?;
        if task_channel.send(packet).await.is_err() {
            error!("Failed to send packet to Bevy!");
            error = ConnectionError::ConnectionClosed;
            break;
        }

        // If the state has changed, return
        if end_state {
            #[cfg(debug_assertions)]
            bevy_log::debug!("Waiting for the state change to be acknowledged...");

            // Notify the writer that the state has changed
            atomic_state.store(1u8, Ordering::Relaxed);
            return Ok(());
        }
    }

    // Notify the writer and return the error
    atomic_state.store(2u8, Ordering::Relaxed);
    Err(error)
}

/// Relay packets from Bevy to the connected server.
async fn bevy_to_server<V, S>(
    write_conn: &mut WriteConnection<V, S, Serverbound>,
    task_channel: &PacketTaskChannel<V, S, Serverbound>,
    acknowledge_fn: fn(&<Serverbound as NetworkDirection<V, S>>::Send) -> bool,
    atomic_state: &AtomicU8,
) -> Result<(), ConnectionError>
where
    V: Version + LoginState + ConfigurationState + PlayState,
    S: State<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
    Serverbound: NetworkDirection<V, S>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
{
    loop {
        // Forward packets from Bevy to the server
        let packet = task_channel.recv().await.map_err(|_| {
            error!("Failed to receive packet from Bevy!");
            ConnectionError::ConnectionClosed
        })?;
        write_conn.send_packet(&packet).await?;

        match atomic_state.load(Ordering::Relaxed) {
            // If the state has changed, check if the change has been acknowledged
            1 => {
                if acknowledge_fn(&packet) {
                    #[cfg(debug_assertions)]
                    bevy_log::debug!("State change acknowledged!");

                    return Ok(());
                }
            }
            // If the connection has been closed, return
            2 => return Ok(()),
            _ => {}
        }
    }
}
