use std::{marker::PhantomData, time::Duration};

use bevy_app::{App, PostUpdate};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::EventWriter,
    schedule::{common_conditions::any_with_component, IntoSystemConfigs},
    system::{Commands, Query},
};
use bevy_log::error;
use bevy_tasks::{block_on, poll_once, Task};
use froglight_core::systemsets::NetworkPostUpdateSet;
use froglight_protocol::{packet::ServerStatus, traits::Version};

use super::events::{ConnectionDisconnect, StatusResponse};
use crate::connection::ConnectionError;

/// A marker [`Component`](bevy_ecs::prelude::Component) used to identify
/// [`Entities`](bevy_ecs::prelude::Entity) that have a
/// [`Connection`](crate::connection::Connection).
#[derive(Clone, Default, Copy, PartialEq, Eq, Hash, Component)]
pub struct ConnectionMarker<V: Version>(PhantomData<V>);

impl<V: Version> std::fmt::Debug for ConnectionMarker<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionMarker<{:?}>", V::default())
    }
}

/// A simple [`Component`] that holds a
/// [`Task`] used to manage a connection.
///
/// Removing this component will drop the connection.
#[derive(Debug, Component)]
pub struct ConnectionTask {
    /// The task handling the connection.
    task: Task<ConnectionError>,
    /// The [`Version::ID`] of the connection.
    pub version_id: i32,
}

impl ConnectionTask {
    /// Create a new [`ConnectionTask`] with the given [`Task`].
    pub(super) fn new<V: Version>(task: Task<ConnectionError>) -> Self {
        Self { task, version_id: V::ID }
    }

    pub(super) fn build(app: &mut App) {
        app.add_systems(
            PostUpdate,
            Self::poll_connection_tasks
                .run_if(any_with_component::<Self>)
                .in_set(NetworkPostUpdateSet),
        );
    }

    /// Poll all connection tasks.
    fn poll_connection_tasks(
        mut query: Query<(Entity, &mut ConnectionTask)>,
        mut events: EventWriter<ConnectionDisconnect>,
        mut commands: Commands,
    ) {
        for (entity, mut task) in &mut query.iter_mut() {
            if let Some(err) = block_on(poll_once(&mut task.task)) {
                events.send(ConnectionDisconnect { entity, reason: err.to_string(), error: err });

                // Remove the task from the entity
                commands.entity(entity).remove::<ConnectionTask>();
            }
        }
    }
}

/// A simple [`Component`] that holds a
/// [`Task`] used to manage a connection.
///
/// Removing this component will drop the connection.
#[derive(Debug, Component)]
pub struct StatusTask {
    /// The task handling the connection.
    task: Task<Result<(ServerStatus, Duration), ConnectionError>>,
    /// The [`Version::ID`] of the connection.
    pub version_id: i32,
}

impl StatusTask {
    /// Create a new [`StatusTask`] with the given [`Task`].
    pub(super) fn new<V: Version>(
        task: Task<Result<(ServerStatus, Duration), ConnectionError>>,
    ) -> Self {
        Self { task, version_id: V::ID }
    }

    pub(super) fn build(app: &mut App) {
        app.add_systems(
            PostUpdate,
            Self::poll_status_tasks.run_if(any_with_component::<Self>).in_set(NetworkPostUpdateSet),
        );
    }

    /// Poll all status tasks.
    fn poll_status_tasks(
        mut query: Query<(Entity, &mut StatusTask)>,
        mut events: EventWriter<StatusResponse>,
        mut commands: Commands,
    ) {
        for (entity, mut task) in &mut query {
            if let Some(result) = block_on(poll_once(&mut task.task)) {
                match result {
                    Err(err) => error!("Error getting ServerStatus: \"{err:?}\""),
                    Ok((status, ping)) => {
                        events.send(StatusResponse::new(status, ping));
                    }
                }

                // Remove the task from the entity
                commands.entity(entity).remove::<StatusTask>();
            }
        }
    }
}
