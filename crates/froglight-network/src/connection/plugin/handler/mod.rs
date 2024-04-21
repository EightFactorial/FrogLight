//! A trait that defines and provides the systems and components required to
//! handle a connection to a server.

use std::{
    net::SocketAddr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use bevy_app::{App, PostUpdate, PreUpdate};
use bevy_ecs::{
    component::Component,
    schedule::{IntoSystemConfigs, IntoSystemSetConfigs},
};
use bevy_log::{debug, error};
use bevy_tasks::IoTaskPool;
use froglight_core::systemsets::{NetworkPostUpdateSet, NetworkPreUpdateSet};
use froglight_protocol::{
    common::ConnectionIntent,
    packet::ServerStatus,
    states::{Handshaking, Login, Status},
    traits::{State, Version},
};

use self::impls::listen_connection_request;
use super::{
    channels::traits::{PacketChannelTrait, TaskChannelTrait},
    systemsets::{ConnectionPostUpdateSet, ConnectionPreUpdateSet},
    ConnectionTask, LoginPlugins, StatusTask,
};
use crate::connection::{Connection, ConnectionError, NetworkDirection, Serverbound};

mod impls;
use impls::listen_status_request;

mod states;
pub use states::*;

/// A trait that defines and provides the systems and components required to
/// handle a connection to a server.
pub trait ConnectionHandler
where
    Self: Version + HandshakeHandler + LoginHandler + StatusHandler,
    Serverbound: NetworkDirection<Self, Handshaking>
        + NetworkDirection<Self, Status>
        + NetworkDirection<Self, Login>,
    Handshaking: State<Self>,
    Status: State<Self>,
    Login: State<Self>,
{
    /// The packet channels inserted into bevy's ECS.
    type PacketChannels: PacketChannelTrait<Self> + Component;

    /// Add [`Version`]-specific systems to the app.
    fn build(app: &mut App) {
        // Configure sets
        app.configure_sets(
            PreUpdate,
            ConnectionPreUpdateSet::<Self>::default().in_set(NetworkPreUpdateSet),
        );
        app.configure_sets(
            PostUpdate,
            ConnectionPostUpdateSet::<Self>::default().in_set(NetworkPostUpdateSet),
        );

        // Listen for status and connection requests
        app.add_systems(
            PostUpdate,
            listen_status_request::<Self>.in_set(ConnectionPostUpdateSet::<Self>::default()),
        );
        app.add_systems(
            PostUpdate,
            listen_connection_request::<Self>.in_set(ConnectionPostUpdateSet::<Self>::default()),
        );

        // Add implementation-specific systems to the app.
        Self::version_build(app);
    }

    /// Add implementation-specific systems to the app.
    fn version_build(app: &mut App);

    /// Get the status of a server using it's address.
    #[cfg(feature = "resolver")]
    #[must_use]
    fn status_of(address: &str, resolver: &crate::resolver::Resolver) -> StatusTask {
        use compact_str::ToCompactString;

        let address = address.to_compact_string();
        let resolver = resolver.clone();

        let task = IoTaskPool::get().spawn(async move {
            match Connection::connect_to(&address, &resolver).await {
                Ok(conn) => Self::status_with(conn).await,
                Err(err) => Err(err),
            }
        });

        StatusTask::new::<Self>(task)
    }

    /// Get the status of a server using it's ip and port.
    #[must_use]
    fn status_of_socket(socket: SocketAddr) -> StatusTask {
        let task = IoTaskPool::get().spawn(async move {
            match Connection::connect(socket).await {
                Ok(conn) => Self::status_with(conn).await,
                Err(err) => Err(err),
            }
        });

        StatusTask::new::<Self>(task)
    }

    /// Get the status and ping of a server with a pre-existing connection.
    #[must_use]
    fn status_with(
        mut conn: Connection<Self, Handshaking>,
    ) -> impl std::future::Future<Output = Result<(ServerStatus, Duration), ConnectionError>> + Send + Sync
    {
        async move {
            // Perform the handshake.
            if let Err(err) = Self::perform_handshake(&mut conn, ConnectionIntent::Status).await {
                error!("Failed to perform handshake: {err:?}");
                return Err(err);
            }

            // Perform the status request.
            debug!("Performing status request");
            let mut conn = conn.status();
            let status = match Self::perform_status_request(&mut conn).await {
                Ok(status) => status,
                Err(err) => {
                    error!("Failed to perform status request: {err:?}");
                    return Err(err);
                }
            };

            // Perform the ping request.
            debug!("Performing ping request");
            #[allow(clippy::cast_possible_truncation)]
            let payload = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
            let ping = match Self::perform_ping_request(&mut conn, payload).await {
                Ok(ping) => ping,
                Err(err) => {
                    error!("Failed to perform ping request: {err:?}");
                    return Err(err);
                }
            };

            Ok((status, ping))
        }
    }

    /// Connect to a server using it's address.
    #[cfg(feature = "resolver")]
    #[must_use]
    fn connect_to(
        address: &str,
        channels: <Self::PacketChannels as PacketChannelTrait<Self>>::TaskHalf,
        resolver: &crate::resolver::Resolver,
        plugins: &LoginPlugins,
    ) -> ConnectionTask {
        use compact_str::ToCompactString;

        let address = address.to_compact_string();
        let resolver = resolver.clone();
        let plugins = plugins.clone();

        let task = IoTaskPool::get().spawn(async move {
            match Connection::connect_to(&address, &resolver).await {
                Ok(conn) => Self::connect_with(conn, channels, plugins).await,
                Err(err) => err,
            }
        });

        ConnectionTask::new::<Self>(task)
    }

    /// Connect to a server using it's ip and port.
    #[must_use]
    fn connect_to_socket(
        socket: SocketAddr,
        channels: <Self::PacketChannels as PacketChannelTrait<Self>>::TaskHalf,
        plugins: &LoginPlugins,
    ) -> ConnectionTask {
        let plugins = plugins.clone();

        let task = IoTaskPool::get().spawn(async move {
            match Connection::connect(socket).await {
                Ok(conn) => Self::connect_with(conn, channels, plugins).await,
                Err(err) => err,
            }
        });

        ConnectionTask::new::<Self>(task)
    }

    /// Connect to a server using a pre-existing connection.
    #[must_use]
    fn connect_with(
        mut conn: Connection<Self, Handshaking>,
        channels: <Self::PacketChannels as PacketChannelTrait<Self>>::TaskHalf,
        plugins: LoginPlugins,
    ) -> impl std::future::Future<Output = ConnectionError> + Send + Sync {
        async move {
            // Perform the handshake.
            if let Err(err) = Self::perform_handshake(&mut conn, ConnectionIntent::Login).await {
                error!("Failed to perform handshake: {err:?}");
                return err;
            };

            // Perform the login.
            let mut conn = conn.login();
            if let Err(err) = Self::perform_login(&mut conn, channels.login(), &plugins).await {
                error!("Failed to perform login: {err:?}");
                return err;
            };

            // Handle packets.
            Self::handle_packets(conn, channels).await
        }
    }

    /// Handle passing back and forth packets.
    #[must_use]
    fn handle_packets(
        conn: Connection<Self, Login>,
        channels: <Self::PacketChannels as PacketChannelTrait<Self>>::TaskHalf,
    ) -> impl std::future::Future<Output = ConnectionError> + Send + Sync;
}
