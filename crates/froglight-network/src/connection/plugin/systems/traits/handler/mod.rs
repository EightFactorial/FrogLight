use std::future::Future;

use bevy_app::{App, PreUpdate};
use bevy_ecs::{bundle::Bundle, prelude::any_with_component, schedule::IntoSystemSetConfigs};
use bevy_tasks::IoTaskPool;
use froglight_core::systemsets::NetworkPreUpdateSet;
use froglight_protocol::{
    common::{ConnectionIntent, GameProfile},
    packet::ServerStatus,
    states::{Handshaking, Login, Play, Status},
    traits::{State, Version},
};

use crate::connection::{
    plugin::{
        channel::{task::ConnectionTask, traits::ChannelType},
        systems::misc::{ConnectionMarker, ConnectionSet},
    },
    Connection, ConnectionError, NetworkDirection, RecvPacketEvent, SendPacketEvent, Serverbound,
};

mod generic;

mod v1_20_0;

pub trait HandleConnection: Version
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
        // Add packet events
        app.add_event::<SendPacketEvent<Self>>().add_event::<RecvPacketEvent<Self>>();

        // Add ConnectionSet<V>
        app.configure_sets(
            PreUpdate,
            ConnectionSet::<Self>::default()
                .run_if(any_with_component::<ConnectionMarker<Self>>)
                .in_set(NetworkPreUpdateSet),
        );

        // Add systems
        Self::add_systems(app);
    }

    fn add_systems(app: &mut App);

    /// Connect to the server and retrieve the server status.
    fn connect_status(
        mut conn: Connection<Self, Handshaking>,
    ) -> impl Future<Output = Result<ServerStatus, ConnectionError>> + Send + Sync {
        async move {
            Self::perform_handshake(&mut conn, ConnectionIntent::Status).await?;

            let mut conn = conn.status();
            Self::perform_status(&mut conn).await
        }
    }

    /// Connect to the server and log in.
    fn connect_game(
        mut conn: Connection<Self, Handshaking>,
    ) -> impl Future<Output = Result<(GameProfile, Self::Channel, ConnectionTask), ConnectionError>>
           + Send
           + Sync {
        async move {
            Self::perform_handshake(&mut conn, ConnectionIntent::Login).await?;

            let mut conn = conn.login();
            let profile = Self::perform_login(&mut conn).await?;

            let (channel, task) = Self::create_task(conn);
            Ok((profile, channel, task))
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

/// A bundle containing [`Connection`] components.
#[derive(Debug, Bundle)]
pub struct ConnectionBundle<V: Version + HandleConnection>
where
    Serverbound: NetworkDirection<V, Handshaking>
        + NetworkDirection<V, Status>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Play>,
    Handshaking: State<V>,
    Status: State<V>,
    Login: State<V>,
    Play: State<V>,
{
    /// The player's game profile.
    pub profile: GameProfile,
    /// The connection channel.
    pub channel: V::Channel,
    /// The connection task.
    pub task: ConnectionTask,
}
