use std::{fmt::Debug, marker::PhantomData};

use bevy::{prelude::*, tasks::IoTaskPool};
use futures_lite::future::{block_on, poll_once};
use mc_rs_proto::{
    types::enums::ConnectionIntent,
    versions::state::{Configuration, Handshake, Login, Play, Status},
    Connection, State, Version,
};

use crate::networking::task::{ConnectionHandshakeTask, ConnectionLoginTask, ConnectionStatusTask};

use super::{
    handle::NetworkHandle,
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
        app.add_event::<StatusRequest<Self>>();
        app.add_event::<ConnectionEvent<Self>>();

        app.add_systems(
            Update,
            (
                (
                    Self::status_request,
                    Self::connection_request,
                    Self::connection_query,
                )
                    .chain(),
                Self::handshake_query,
                Self::status_query,
                Self::login_query,
                Self::configuration_query,
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
                    commands.spawn(ConnectionTask::new_with(task, event.intent));
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
                        info!("Connected to {}", con.peer_addr().unwrap());

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
                        info!("Handshake finished with {}", con.peer_addr().unwrap());

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

    /// Wait for the login to finish and start the next state
    fn login_query(
        mut _query: Query<(Entity, &mut ConnectionLoginTask<Self>)>,
        mut _commands: Commands,
    ) {
        todo!();
    }

    /// Wait for the configuration to finish and start the next state
    fn configuration_query(
        mut _query: Query<(Entity, &mut ConnectionConfigurationTask<Self>)>,
        mut _commands: Commands,
    ) {
        todo!();
    }
}
