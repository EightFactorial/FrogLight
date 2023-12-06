use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use bevy::{ecs::system::SystemState, prelude::*, tasks::IoTaskPool};
use futures_lite::future::{block_on, poll_once};
use mc_rs_core::{
    components::player::CreateControlledPlayerEvent,
    events::{ConnectionEvent, PingResponse, StatusRequest, StatusResponse},
    resources::player::username::Username,
    schedule::state::ApplicationState,
};
use mc_rs_protocol::{
    types::enums::ConnectionIntent,
    versions::state::{Configuration, Handshake, Login, Play, Status},
    Connection, ConnectionError, State, Version,
};

use crate::{
    handle::ConnectionEnum,
    task::{ConnectionChannel, ConnectionHandshakeTask, ConnectionLoginTask, ConnectionStatusTask},
    NetworkingSet,
};

use super::{
    handle::{ConnectionData, ConnectionState, NetworkHandle},
    task::ConnectionTask,
};

/// A trait that defines how to handle a network version
///
/// A version must also have the [`NetworkHandle`] trait implemented
pub(super) trait Network: NetworkHandle + Version + Send + Sync + 'static
where
    Handshake: State<Self>,
    Status: State<Self>,
    Login: State<Self>,
    Configuration: State<Self>,
    Play: State<Self>,
{
    /// Register the version's networking systems to the app
    fn register(app: &mut App) {
        // Add events
        app.add_event::<StatusRequest<Self>>();
        app.add_event::<ConnectionEvent<Self>>();

        // Configure the system set
        app.configure_sets(
            PreUpdate,
            ConnectionSystemSet::<Self>::default()
                .run_if(any_with_component::<ConnectionMarker<Self>>())
                .ambiguous_with(NetworkingSet)
                .in_set(NetworkingSet),
        );

        // Add systems to the set
        app.add_systems(
            PreUpdate,
            (
                (Self::status_request, Self::connection_request).chain(),
                (
                    Self::connection_query.run_if(any_with_component::<ConnectionTask<Self>>()),
                    Self::handshake_query
                        .run_if(any_with_component::<ConnectionHandshakeTask<Self>>()),
                    Self::status_query.run_if(any_with_component::<ConnectionStatusTask<Self>>()),
                    Self::login_query.run_if(any_with_component::<ConnectionLoginTask<Self>>()),
                    Self::packet_query.run_if(resource_exists::<ConnectionChannel<Self>>()),
                )
                    .in_set(ConnectionSystemSet::<Self>::default()),
            ),
        );
    }

    /// Create a new connection to get the server status
    fn status_request(
        mut reader: EventReader<StatusRequest<Self>>,
        mut writer: EventWriter<ConnectionEvent<Self>>,
    ) {
        for request in reader.read() {
            writer.send(ConnectionEvent::new_with(
                request.entity,
                request.hostname.clone(),
                ConnectionIntent::Status,
            ));
        }
    }

    /// Create a new connection to the server
    fn connection_request(mut events: EventReader<ConnectionEvent<Self>>, mut commands: Commands) {
        for event in events.read() {
            let addr = event.hostname.clone();
            let task = IoTaskPool::get().spawn(Connection::new(Self::default(), addr.clone()));

            match event.intent {
                ConnectionIntent::Status | ConnectionIntent::Login => {
                    let mut entity = commands.spawn((
                        ConnectionMarker::<Self>::default(),
                        ConnectionTask::new_with(task, addr, event.intent),
                    ));

                    if matches!(
                        (event.intent, event.entity),
                        (ConnectionIntent::Status, Some(_))
                    ) {
                        entity.insert(ConnectionReplyMarker(event.entity.unwrap()));
                    }
                }
                _ => {
                    warn!("Skipping connection creation with invalid connection intent!");
                }
            }
        }
    }

    /// Wait for connections to be established and start the handshake
    fn connection_query(
        mut query: Query<(Entity, &mut ConnectionTask<Self>)>,
        mut commands: Commands,
    ) {
        for (entity, mut task) in &mut query {
            if let Some(result) = block_on(poll_once(task.task_mut())) {
                match result {
                    Ok(con) => {
                        debug!(
                            "Connected to {} ({})",
                            con.hostname,
                            con.peer_addr().expect("Unable to get peer address")
                        );

                        let handshake_task =
                            IoTaskPool::get().spawn(Self::handshake_handle(con, task.intent));

                        commands
                            .entity(entity)
                            .insert(ConnectionHandshakeTask::new(handshake_task, task.intent))
                            .remove::<ConnectionTask<Self>>();
                    }
                    Err(err) => {
                        error!("Failed to connect to {}, {}", task.hostname, err);
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }

    /// Wait for the handshake to finish and start the next state
    fn handshake_query(
        username: Res<Username>,
        mut query: Query<(Entity, &mut ConnectionHandshakeTask<Self>)>,
        mut commands: Commands,
    ) {
        for (entity, mut task) in &mut query {
            if let Some(result) = block_on(poll_once(task.task_mut())) {
                let mut entity_commands = commands.entity(entity);

                match result {
                    Ok(con) => {
                        debug!(
                            "Handshake finished with {} ({})",
                            con.hostname,
                            con.peer_addr().expect("Unable to get peer address")
                        );

                        entity_commands.remove::<ConnectionHandshakeTask<Self>>();

                        match task.intent {
                            ConnectionIntent::Status => {
                                let status_task =
                                    IoTaskPool::get().spawn(Self::status_handle(con.into()));

                                entity_commands.insert(ConnectionStatusTask::new(status_task));
                            }
                            ConnectionIntent::Login => {
                                let login_task = IoTaskPool::get()
                                    .spawn(Self::login_handle(username.clone(), con.into()));

                                entity_commands.insert(ConnectionLoginTask::new(login_task));
                            }
                            _ => {
                                error!("Invalid connection task intent!");

                                entity_commands.despawn_recursive();
                            }
                        }
                    }
                    Err(err) => {
                        error!("Failed to handshake: {}", err);
                        entity_commands.despawn_recursive();
                    }
                }
            }
        }
    }

    /// Wait for the status to finish and broadcast the results
    fn status_query(
        mut query: Query<(
            Entity,
            &mut ConnectionStatusTask<Self>,
            Option<&ConnectionReplyMarker>,
        )>,
        mut status_events: EventWriter<StatusResponse>,
        mut ping_events: EventWriter<PingResponse>,
        mut commands: Commands,
    ) {
        for (entity, mut task, reply) in &mut query {
            if let Some(result) = block_on(poll_once(task.task_mut())) {
                match result {
                    Ok((mut status, mut ping)) => {
                        if let Some(entity) = reply {
                            status.entity = Some(entity.0);
                            ping.entity = Some(entity.0);
                        }

                        #[cfg(any(debug_assertions, feature = "debug"))]
                        trace!("`{}` responded with {:?}", status.hostname, status);
                        status_events.send(status);

                        #[cfg(any(debug_assertions, feature = "debug"))]
                        debug!("`{}` responded with {:?}", ping.hostname, ping);
                        ping_events.send(ping);
                    }
                    Err(err) => {
                        error!("Failed to get status: {}", err);
                    }
                }

                // Despawn the entity, we're done with it
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    /// Whether or not the version has the configuration state
    ///
    /// All versions starting with 1.20.2 and after have the configuration state
    const HAS_CONFIGURATION_STATE: bool = true;

    /// Whether or not the version has the configuration state
    ///
    /// Used for bevy system [`run_if`](bevy::ecs::schedule::config::IntoSystemConfigs::run_if)s
    fn has_configuration_state() -> bool { Self::HAS_CONFIGURATION_STATE }

    /// Wait for the login to finish and start the next state
    fn login_query(
        mut query: Query<(Entity, &mut ConnectionLoginTask<Self>)>,
        mut state: ResMut<NextState<ApplicationState>>,
        mut events: EventWriter<CreateControlledPlayerEvent>,
        mut commands: Commands,
    ) {
        for (entity, mut task) in &mut query {
            if let Some(result) = block_on(poll_once(task.task_mut())) {
                match result {
                    Ok((conn, profile)) => {
                        debug!(
                            "Login finished with {}",
                            conn.peer_addr().expect("Unable to get peer address")
                        );

                        events.send(CreateControlledPlayerEvent(entity));

                        commands
                            .entity(entity)
                            .insert(profile)
                            .remove::<ConnectionLoginTask<Self>>();

                        let (tx1, rx1) = flume::unbounded();
                        let (tx2, rx2) = flume::unbounded();

                        if Self::HAS_CONFIGURATION_STATE {
                            let task = IoTaskPool::get().spawn(Self::packet_handle(
                                ConnectionEnum::Configuration(conn.into()),
                                tx1,
                                rx2,
                            ));

                            commands.insert_resource(ConnectionChannel::new_config(rx1, tx2, task));
                        } else {
                            let task = IoTaskPool::get().spawn(Self::packet_handle(
                                ConnectionEnum::Play(conn.into()),
                                tx1,
                                rx2,
                            ));

                            commands.insert_resource(ConnectionChannel::new_play(rx1, tx2, task));
                            state.set(ApplicationState::InGame);
                        }
                    }
                    Err(err) => {
                        error!("Failed to login: {}", err);
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }

    /// Query the task for any packets and handle them
    fn packet_query(world: &mut World) {
        let mut channel_state: ConnectionState;
        let mut channel_data = Vec::new();

        // Get the channel data and state
        {
            let mut state = SystemState::<Res<ConnectionChannel<Self>>>::new(world);
            let task = state.get(world);

            channel_state = task.state;

            if task.is_disconnected() {
                error!("Channel disconnected");
                world.remove_resource::<ConnectionChannel<Self>>();
                return;
            }

            for result in task.try_iter() {
                match result {
                    Ok(data) => {
                        channel_data.push(data);
                    }
                    Err(err) => {
                        match err {
                            ConnectionError::Closed => {
                                error!("Connection Closed");
                                // TODO: Handle this
                            }
                            ConnectionError::Disconnected(reason) => {
                                warn!("Disconnected: {}", reason.to_string());
                                // TODO: Handle this
                            }
                            ConnectionError::ParsePort(_)
                            | ConnectionError::NoAddressFound
                            | ConnectionError::UnexpectedPacket => {
                                unreachable!("Does not occur in configuration/play state")
                            }
                            _ => {
                                error!("{err}");
                            }
                        }

                        world.remove_resource::<ConnectionChannel<Self>>();
                        return;
                    }
                }
            }
        }

        for data in channel_data {
            match data {
                ConnectionData::Configuration(packet) => {
                    if Self::HAS_CONFIGURATION_STATE {
                        if channel_state != ConnectionState::Configuration {
                            warn!("Received configuration packet in play state!");
                        }

                        // Process the configuration packet
                        Self::config_packet(world, packet);
                    } else {
                        unreachable!("Configuration packet when connection doesn't have Configuration state!")
                    }
                }
                ConnectionData::Play(packet) => {
                    if Self::HAS_CONFIGURATION_STATE && channel_state != ConnectionState::Play {
                        warn!("Received play packet in configuration state!");
                    }

                    // Process the play packet
                    Self::play_packet(world, packet);
                }
                ConnectionData::NewState(state) => {
                    if Self::HAS_CONFIGURATION_STATE {
                        debug!("Connection state changed to {:?}", state);

                        // Update the channel state
                        channel_state = state;
                    } else {
                        unreachable!(
                            "State changed when connection doesn't have Configuration state!"
                        )
                    }
                }
                ConnectionData::Closed => todo!("Handle closed connection"),
            }
        }

        // Update the channel state
        {
            SystemState::<ResMut<ConnectionChannel<Self>>>::new(world)
                .get_mut(world)
                .state = channel_state;
        }
    }

    /// Handle configuration packets, implemented individually for each version
    ///
    /// Only called if the version has the configuration state
    fn config_packet(world: &mut World, packet: <Configuration as State<Self>>::Clientbound);

    /// Handle play packets, implemented individually for each version
    fn play_packet(world: &mut World, packet: <Play as State<Self>>::Clientbound);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(super) struct ConnectionReplyMarker(Entity);

/// A system set that contains all the systems needed for a connection
#[derive(Debug, Default, PartialEq, Eq, Hash, SystemSet)]
pub(super) struct ConnectionSystemSet<V: Version>(PhantomData<V>);

impl<V: Version> Clone for ConnectionSystemSet<V> {
    fn clone(&self) -> Self { Self(self.0) }
}

/// A marker component for entities that contain a connection
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(super) struct ConnectionMarker<V: Version>(PhantomData<V>);
