use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use bevy::{ecs::system::SystemState, prelude::*, tasks::IoTaskPool};
use futures_lite::future::{block_on, poll_once};
use mc_rs_proto::{
    types::enums::ConnectionIntent,
    versions::state::{Configuration, Handshake, Login, Play, Status},
    Connection, State, Version,
};

use crate::networking::{
    handle::ConnectionEnum,
    task::{
        ConnectionHandshakeTask, ConnectionLoginTask, ConnectionPlayTask, ConnectionStatusTask,
    },
};

use super::{
    handle::{ConnectionData, ConnectionState, NetworkHandle},
    request::{PingResponse, StatusRequest, StatusResponse},
    task::{ConnectionConfigurationTask, ConnectionTask},
};

/// An event that is sent to create a new connection
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub struct ConnectionEvent<V: Version> {
    pub addr: String,
    pub intent: ConnectionIntent,
    _version: PhantomData<V>,
}

impl<V: Version> ConnectionEvent<V> {
    pub fn new(addr: impl Into<String>, intent: ConnectionIntent) -> Self {
        Self {
            addr: addr.into(),
            intent,
            _version: PhantomData,
        }
    }
}

/// A trait that defines how to handle a network version
///
/// A version must also have the [NetworkHandle] trait implemented
pub trait Network: NetworkHandle + Version + Send + Sync + 'static
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
        app.configure_set(
            Update,
            ConnectionSystemSet::<Self>::default()
                .run_if(any_with_component::<ConnectionMarker<Self>>()),
        );

        // Add systems to the set
        app.add_systems(
            Update,
            (
                (Self::status_request, Self::connection_request).chain(),
                (
                    Self::connection_query.run_if(any_with_component::<ConnectionTask<Self>>()),
                    Self::handshake_query
                        .run_if(any_with_component::<ConnectionHandshakeTask<Self>>()),
                    Self::status_query.run_if(any_with_component::<ConnectionStatusTask<Self>>()),
                    Self::login_query.run_if(any_with_component::<ConnectionLoginTask<Self>>()),
                    Self::configuration_query.run_if(
                        Self::has_configuration_state
                            .and_then(any_with_component::<ConnectionConfigurationTask<Self>>()),
                    ),
                    Self::packet_query.run_if(any_with_component::<ConnectionPlayTask<Self>>()),
                )
                    .in_set(ConnectionSystemSet::<Self>::default()),
            ),
        );
    }

    /// Send status request
    fn status_request(
        mut reader: EventReader<StatusRequest<Self>>,
        mut writer: EventWriter<ConnectionEvent<Self>>,
    ) {
        for request in reader.iter() {
            writer.send(ConnectionEvent::new(
                request.host.clone(),
                ConnectionIntent::Status,
            ));
        }
    }

    /// Create connections from connection events
    fn connection_request(mut events: EventReader<ConnectionEvent<Self>>, mut commands: Commands) {
        for event in events.iter() {
            let addr = event.addr.clone();
            let task = IoTaskPool::get().spawn(Connection::new(Self::default(), addr));

            match event.intent {
                ConnectionIntent::Status | ConnectionIntent::Login => {
                    commands.spawn((
                        ConnectionMarker::<Self>::default(),
                        ConnectionTask::new_with(task, event.intent),
                    ));
                }
                _ => {
                    warn!("Skipping making connection with invalid connection intent!");
                }
            }
        }
    }

    /// Wait for connections to be established and start the handshake
    fn connection_query(
        mut query: Query<(Entity, &mut ConnectionTask<Self>)>,
        mut commands: Commands,
    ) {
        for (entity, mut task) in query.iter_mut() {
            if let Some(result) = block_on(poll_once(task.task_mut())) {
                match result {
                    Ok(con) => {
                        info!(
                            "Connected to {}",
                            con.peer_addr().expect("Unable to get peer address")
                        );

                        let new_task =
                            IoTaskPool::get().spawn(Self::handshake_handle(con, task.intent));

                        commands
                            .entity(entity)
                            .insert(ConnectionHandshakeTask::new(new_task, task.intent));
                    }
                    Err(err) => {
                        error!("Failed to connect: {}", err);
                    }
                }

                commands.entity(entity).remove::<ConnectionTask<Self>>();
            }
        }
    }

    /// Wait for the handshake to finish and start the next state
    fn handshake_query(
        mut query: Query<(Entity, &mut ConnectionHandshakeTask<Self>)>,
        mut commands: Commands,
    ) {
        for (entity, mut task) in query.iter_mut() {
            if let Some(result) = block_on(poll_once(task.task_mut())) {
                match result {
                    Ok(con) => {
                        info!(
                            "Handshake finished with {}",
                            con.peer_addr().expect("Unable to get peer address")
                        );

                        match task.intent {
                            ConnectionIntent::Status => {
                                let new_task =
                                    IoTaskPool::get().spawn(Self::status_handle(con.into()));

                                commands
                                    .entity(entity)
                                    .insert(ConnectionStatusTask::new(new_task));
                            }
                            ConnectionIntent::Login => {
                                let new_task =
                                    IoTaskPool::get().spawn(Self::login_handle(con.into()));

                                commands
                                    .entity(entity)
                                    .insert(ConnectionLoginTask::new(new_task));
                            }
                            _ => {
                                unreachable!("Invalid connection intent!")
                            }
                        }
                    }
                    Err(err) => {
                        error!("Failed to handshake: {}", err);
                    }
                }

                commands
                    .entity(entity)
                    .remove::<ConnectionHandshakeTask<Self>>();
            }
        }
    }

    /// Wait for the status to finish and broadcast the results
    fn status_query(
        mut query: Query<(Entity, &mut ConnectionStatusTask<Self>)>,
        mut status_events: EventWriter<StatusResponse>,
        mut ping_events: EventWriter<PingResponse>,
        mut commands: Commands,
    ) {
        for (entity, mut task) in query.iter_mut() {
            if let Some(result) = block_on(poll_once(task.task_mut())) {
                match result {
                    Ok((status, ping)) => {
                        info!("Status finished with {:?}", status);
                        status_events.send(status);

                        info!("Ping finished with {:?}", ping);
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
    const HAS_CONFIGURATION_STATE: bool;

    fn has_configuration_state() -> bool { Self::HAS_CONFIGURATION_STATE }

    /// Wait for the login to finish and start the next state
    fn login_query(
        mut query: Query<(Entity, &mut ConnectionLoginTask<Self>)>,
        mut commands: Commands,
    ) {
        for (entity, mut task) in query.iter_mut() {
            if let Some(result) = block_on(poll_once(task.task_mut())) {
                match result {
                    Ok((con, profile)) => {
                        info!(
                            "Login finished with {}",
                            con.peer_addr().expect("Unable to get peer address")
                        );

                        if Self::HAS_CONFIGURATION_STATE {
                            // Go to the configuration state
                            let new_task =
                                IoTaskPool::get().spawn(Self::configuration_handle(con.into()));

                            commands
                                .entity(entity)
                                .insert(profile)
                                .insert(ConnectionConfigurationTask::new(new_task));
                        } else {
                            // Go to the play state
                            let (tx1, rx1) = flume::unbounded();
                            let (tx2, rx2) = flume::unbounded();

                            let con: Connection<Self, Play> = con.into();
                            let new_task = IoTaskPool::get().spawn(Self::play_handle(
                                ConnectionEnum::Play(con),
                                tx1,
                                rx2,
                            ));

                            commands
                                .entity(entity)
                                .insert(profile)
                                .insert(ConnectionPlayTask::new(rx1, tx2, new_task));
                        }
                    }
                    Err(err) => {
                        error!("Failed to login: {}", err);
                    }
                }

                commands
                    .entity(entity)
                    .remove::<ConnectionLoginTask<Self>>();
            }
        }
    }

    /// Wait for the configuration to finish and start the next state
    fn configuration_query(
        mut query: Query<(Entity, &mut ConnectionConfigurationTask<Self>)>,
        mut commands: Commands,
    ) {
        for (entity, mut task) in query.iter_mut() {
            if let Some(result) = block_on(poll_once(task.task_mut())) {
                match result {
                    Ok(con) => {
                        info!(
                            "Configuration finished with {}",
                            con.peer_addr().expect("Unable to get peer address")
                        );

                        let (tx1, rx1) = flume::unbounded();
                        let (tx2, rx2) = flume::unbounded();

                        let con: Connection<Self, Play> = con.into();
                        let new_task = IoTaskPool::get().spawn(Self::play_handle(
                            ConnectionEnum::Play(con),
                            tx1,
                            rx2,
                        ));

                        commands
                            .entity(entity)
                            .insert(ConnectionPlayTask::new(rx1, tx2, new_task));
                    }
                    Err(err) => {
                        error!("Failed to configure client: {}", err);
                    }
                }

                commands
                    .entity(entity)
                    .remove::<ConnectionConfigurationTask<Self>>();
            }
        }
    }

    /// Query the task for any packets
    fn packet_query(world: &mut World) {
        let mut channel_state: ConnectionState;
        let mut channel_data = Vec::new();

        // Get the channel state and data
        {
            let mut state = SystemState::<Query<&ConnectionPlayTask<Self>>>::new(world);
            let query = state.get(world);
            let task = query.single();

            channel_state = *task.state();
            while let Ok(result) = task.recv() {
                match result {
                    Ok(data) => {
                        channel_data.push(data);
                    }
                    Err(err) => {
                        error!("Failed to receive packet: {}", err);
                    }
                }
            }
        }

        for data in channel_data {
            match data {
                ConnectionData::Configuration(packet) => {
                    if Self::HAS_CONFIGURATION_STATE {
                        if channel_state != ConnectionState::Configuration {
                            error!("Received configuration packet in play state!");
                        }

                        Self::config_packet(world, packet)
                    } else {
                        unreachable!("Configuration packet when connection doesn't have Configuration state!")
                    }
                }
                ConnectionData::Play(packet) => {
                    if channel_state != ConnectionState::Play {
                        error!("Received play packet in configuration state!");
                    }

                    Self::play_packet(world, packet)
                }
                ConnectionData::NewState(state) => {
                    if Self::HAS_CONFIGURATION_STATE {
                        info!("Connection state changed to {:?}", state);
                        channel_state = state;
                    } else {
                        unreachable!(
                            "State changed when connection doesn't have Configuration state!"
                        )
                    }
                }
                ConnectionData::Closed => todo!("Handle closed connection!"),
            }
        }

        // Update the channel state
        {
            let mut state = SystemState::<Query<&ConnectionPlayTask<Self>>>::new(world);
            let query = state.get(world);
            let task = query.single();

            *task
                .state
                .write()
                .expect("Unable to write to channel state") = channel_state;
        }
    }

    fn config_packet(world: &mut World, packet: <Configuration as State<Self>>::Clientbound);

    fn play_packet(world: &mut World, packet: <Play as State<Self>>::Clientbound);
}

/// A system set that contains all the systems needed for a connection
#[derive(Debug, Default, PartialEq, Eq, Hash, SystemSet)]
pub struct ConnectionSystemSet<V: Version>(PhantomData<V>);

/// I don't know why I have to do this myself
impl<V: Version> Clone for ConnectionSystemSet<V> {
    fn clone(&self) -> Self { Self(self.0) }
}

/// A marker component for entities that represent a connection
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct ConnectionMarker<V: Version>(PhantomData<V>);
