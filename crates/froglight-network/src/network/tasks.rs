use std::time::Duration;

use bevy_app::{App, PostUpdate};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    schedule::{common_conditions::any_with_component, IntoSystemConfigs},
    system::{ParallelCommands, Query},
};
use bevy_tasks::Task;
use froglight_protocol::{packet::ServerStatus, traits::Version};

use super::NetworkPostUpdateSet;
use crate::connection::ConnectionError;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.add_systems(
        PostUpdate,
        ConnectionTask::poll_connection_tasks
            .run_if(any_with_component::<ConnectionTask>)
            .in_set(NetworkPostUpdateSet),
    );
    app.add_systems(
        PostUpdate,
        StatusTask::poll_status_tasks
            .run_if(any_with_component::<StatusTask>)
            .in_set(NetworkPostUpdateSet),
    );
}

/// A [`Task`] for a connection.
#[derive(Debug, Component)]
pub struct ConnectionTask {
    task: Task<ConnectionTaskResult>,
    version: i32,
}

/// The result of a [`ConnectionTask`].
pub type ConnectionTaskResult = Result<(), ConnectionError>;

impl ConnectionTask {
    /// Create a new [`ConnectionTask`] with the given task.
    #[must_use]
    pub const fn new<V: Version>(task: Task<ConnectionTaskResult>) -> Self {
        Self::new_with_id(task, V::ID)
    }
    /// Create a new [`ConnectionTask`] with the given task and version id.
    ///
    /// If you know the [`Version`], use [`ConnectionTask::new`] instead.
    #[must_use]
    pub const fn new_with_id(task: Task<ConnectionTaskResult>, version: i32) -> Self {
        Self { task, version }
    }

    /// Check if the [`ConnectionTask`] is for the given [`Version`].
    #[must_use]
    pub const fn is_version<V: Version>(&self) -> bool { self.is_version_id(V::ID) }
    /// Check if the [`ConnectionTask`] is for the given version id.
    ///
    /// If you know the [`Version`], use [`ConnectionTask::is_version`] instead.
    #[must_use]
    pub const fn is_version_id(&self, version: i32) -> bool { self.version == version }

    /// Poll the task once.
    ///
    /// # Note
    /// If the task returns `Some`, it should be immediately dropped.
    ///
    /// Polling the task again will cause a panic.
    pub fn poll_once(&mut self) -> Option<ConnectionTaskResult> {
        bevy_tasks::block_on(bevy_tasks::poll_once(&mut self.task))
    }

    /// A bevy [`System`](bevy_ecs::system::System) that
    /// polls all [`ConnectionTask`]s in parallel.
    fn poll_connection_tasks(mut query: Query<(Entity, &mut Self)>, commands: ParallelCommands) {
        query.par_iter_mut().for_each(|(entity, mut task)| match task.poll_once() {
            // The task errored, log and remove it.
            Some(Err(err)) => {
                #[cfg(debug_assertions)]
                bevy_log::error!("Connection Error: {entity:?}: {err}");

                commands.command_scope(|mut commands| {
                    commands.entity(entity).remove::<Self>();
                });
            }
            // The task is done, remove it.
            Some(Ok(())) => {
                #[cfg(debug_assertions)]
                bevy_log::debug!("Connection Closed: {entity:?}");

                commands.command_scope(|mut commands| {
                    commands.entity(entity).remove::<Self>();
                });
            }
            // Do nothing, the task is still running.
            None => {}
        });
    }
}

/// A [`Task`] for a status request.
#[derive(Debug, Component)]
pub struct StatusTask {
    task: Task<StatusTaskResult>,
    version: i32,
}

/// The result of a [`StatusTask`].
pub type StatusTaskResult = Result<(ServerStatus, Duration), ConnectionError>;

impl StatusTask {
    /// Create a new [`StatusTask`] with the given task.
    #[must_use]
    pub const fn new<V: Version>(task: Task<StatusTaskResult>) -> Self {
        Self::new_with_id(task, V::ID)
    }
    /// Create a new [`StatusTask`] with the given task and version id.
    ///
    /// If you know the [`Version`], use [`StatusTask::new`] instead.
    #[must_use]
    pub const fn new_with_id(task: Task<StatusTaskResult>, version: i32) -> Self {
        Self { task, version }
    }

    /// Check if the [`StatusTask`] is for the given [`Version`].
    #[must_use]
    pub const fn is_version<V: Version>(&self) -> bool { self.is_version_id(V::ID) }
    /// Check if the [`StatusTask`] is for the given version id.
    ///
    /// If you know the [`Version`], use [`StatusTask::is_version`] instead.
    #[must_use]
    pub const fn is_version_id(&self, version: i32) -> bool { self.version == version }

    /// Poll the task once.
    ///
    /// # Note
    /// If the task returns `Some`, it should be immediately dropped.
    ///
    /// Polling the task again will cause a panic.
    pub fn poll_once(&mut self) -> Option<StatusTaskResult> {
        bevy_tasks::block_on(bevy_tasks::poll_once(&mut self.task))
    }

    /// A bevy [`System`](bevy_ecs::system::System) that
    /// polls all [`StatusTask`]s in parallel.
    fn poll_status_tasks(mut query: Query<(Entity, &mut Self)>, commands: ParallelCommands) {
        query.par_iter_mut().for_each(|(entity, mut task)| match task.poll_once() {
            // The task errored, log and remove it.
            Some(Err(err)) => {
                #[cfg(debug_assertions)]
                bevy_log::error!("Status Error: {entity:?}: {err}");

                commands.command_scope(|mut commands| {
                    commands.entity(entity).remove::<Self>();
                });
            }
            // The task is done, remove it.
            Some(Ok((_status, _ping))) => {
                #[cfg(debug_assertions)]
                bevy_log::debug!("Status Received: {entity:?}");

                commands.command_scope(|mut commands| {
                    commands.entity(entity).remove::<Self>();
                });
            }
            // Do nothing, the task is still running.
            None => {}
        });
    }
}
