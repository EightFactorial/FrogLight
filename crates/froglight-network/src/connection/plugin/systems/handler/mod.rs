use std::future::Future;

use bevy_app::{App, PostUpdate, PreUpdate};
use bevy_ecs::{
    prelude::any_with_component,
    schedule::{common_conditions::on_event, IntoSystemConfigs, IntoSystemSetConfigs},
};
use bevy_tasks::IoTaskPool;
use compact_str::CompactString;
use froglight_core::systemsets::{NetworkPostUpdateSet, NetworkPreUpdateSet};
use froglight_protocol::{
    common::{ConnectionIntent, GameProfile},
    packet::ServerStatus,
    states::{Handshaking, Login, Play, Status},
    traits::{State, Version},
};

use crate::{
    connection::{
        plugin::{
            channel::{task::ConnectionTask, traits::ChannelType},
            systems::misc::{ConnectionBundle, ConnectionPostUpdateSet, ConnectionPreUpdateSet},
        },
        server_conn::PendingConnectionTask,
        server_status::PendingRequestTask,
        Connection, ConnectionError, NetworkDirection, RequestConnectionEvent, RequestStatusEvent,
        Serverbound,
    },
    resolver::Resolver,
};

mod generic;

mod v1_20_0;

pub trait ConnectionHandler: Version
where
    Serverbound: NetworkDirection<Self, Handshaking>
        + NetworkDirection<Self, Status>
        + NetworkDirection<Self, Login>
        + NetworkDirection<Self, Play>,
    Handshaking: State<Self>,
    Status: State<Self>,
    Login: State<Self>,
    Play: State<Self>,
{
    type Channel: ChannelType;

    fn build(app: &mut App) {
        // Add ConnectionSets
        app.configure_sets(
            PreUpdate,
            ConnectionPreUpdateSet::<Self>::default().in_set(NetworkPreUpdateSet),
        );
        app.configure_sets(
            PostUpdate,
            ConnectionPostUpdateSet::<Self>::default().in_set(NetworkPostUpdateSet),
        );

        // Handle status request events
        app.add_systems(
            PostUpdate,
            PendingRequestTask::listen_for_requeststatus_event::<Self>
                .run_if(on_event::<RequestStatusEvent>())
                .in_set(ConnectionPostUpdateSet::<Self>::default()),
        );

        // Handle connection request events
        app.add_systems(
            PreUpdate,
            PendingConnectionTask::<Self>::poll_conn_tasks
                .ambiguous_with_all()
                .run_if(any_with_component::<PendingConnectionTask<Self>>)
                .in_set(ConnectionPreUpdateSet::<Self>::default()),
        );
        app.add_systems(
            PostUpdate,
            PendingConnectionTask::<Self>::listen_for_requestconn_event
                .run_if(on_event::<RequestConnectionEvent>())
                .in_set(ConnectionPostUpdateSet::<Self>::default()),
        );

        // Add other events and systems
        Self::build_version(app);
    }

    /// Build the version-specific events and systems.
    fn build_version(app: &mut App);

    /// Connect to the server and retrieve the server status.
    fn connect_status(
        address: CompactString,
        resolver: Resolver,
    ) -> impl Future<Output = Result<ServerStatus, ConnectionError>> + Send {
        async move {
            let mut conn = Connection::connect_to(&address, &resolver).await?;
            Self::perform_handshake(&mut conn, ConnectionIntent::Status).await?;

            let mut conn = conn.status();
            Self::perform_status(&mut conn).await
        }
    }

    /// Connect to the server and log in.
    fn connect_game(
        address: CompactString,
        resolver: Resolver,
    ) -> impl Future<Output = Result<ConnectionBundle<Self>, ConnectionError>> + Send {
        async move {
            let mut conn = Connection::connect_to(&address, &resolver).await?;
            Self::perform_handshake(&mut conn, ConnectionIntent::Login).await?;

            let mut conn = conn.login();
            let profile = Self::perform_login(&mut conn).await?;

            let (channel, task) = Self::create_task(conn);
            Ok(ConnectionBundle::new(profile, channel, task))
        }
    }

    /// Performs the handshake process with the server.
    fn perform_handshake(
        conn: &mut Connection<Self, Handshaking>,
        intent: ConnectionIntent,
    ) -> impl Future<Output = Result<(), ConnectionError>> + Send + Sync;

    /// Performs the status request process with the server.
    fn perform_status(
        conn: &mut Connection<Self, Status>,
    ) -> impl Future<Output = Result<ServerStatus, ConnectionError>> + Send + Sync;

    /// Performs the login process with the server.
    fn perform_login(
        conn: &mut Connection<Self, Login>,
    ) -> impl Future<Output = Result<GameProfile, ConnectionError>> + Send + Sync;

    /// Creates a new task and channel for the connection.
    ///
    /// The connection will be passed to the task function,
    /// and send/receive information via the channel.
    fn create_task(conn: Connection<Self, Login>) -> (Self::Channel, ConnectionTask) {
        let (channel, task_half) = Self::Channel::new_pair();

        let future = Self::task_function(conn, task_half);
        let task = IoTaskPool::get().spawn(future);

        (channel, ConnectionTask::new(task))
    }

    fn task_function(
        conn: Connection<Self, Login>,
        channel: <Self::Channel as ChannelType>::TaskHalf,
    ) -> impl Future<Output = ()> + Send;
}
