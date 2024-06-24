use std::time::Duration;

use bevy_app::{App, PostUpdate};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::EventWriter,
    query::With,
    reflect::ReflectComponent,
    schedule::{common_conditions::any_with_component, IntoSystemConfigs},
    system::{Local, ParallelCommands, Query},
};
use bevy_hierarchy::DespawnRecursiveExt;
use bevy_reflect::{std_traits::ReflectDefault, Reflect};
use bevy_tasks::Task;
use froglight_protocol::{packet::ServerStatus, traits::Version};
use parking_lot::Mutex;

use super::{NetworkErrorEvent, NetworkPostUpdateSet, ServerStatusResponse};
use crate::connection::ConnectionError;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<PolledTask>();

    app.add_systems(
        PostUpdate,
        PolledTask::poll_status_tasks
            .ambiguous_with(PolledTask::poll_connection_tasks)
            .run_if(any_with_component::<StatusTask>)
            .run_if(any_with_component::<PolledTask>)
            .in_set(NetworkPostUpdateSet),
    );
    app.add_systems(
        PostUpdate,
        PolledTask::poll_connection_tasks
            .ambiguous_with(PolledTask::poll_status_tasks)
            .run_if(any_with_component::<ConnectionTask>)
            .run_if(any_with_component::<PolledTask>)
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
}

/// A marker [`Component`] for [`Tasks`](Task) that will automatically be
/// polled.
///
/// Add this to a [`ConnectionTask`] or [`StatusTask`] to have it polled,
/// the result sent as an [`Event`](bevy_ecs::event::Event), and then despawned.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Default, Component)]
pub struct PolledTask;

impl PolledTask {
    /// A bevy [`System`](bevy_ecs::system::System) that
    /// polls all [`ConnectionTask`]s in parallel.
    pub(super) fn poll_connection_tasks(
        mut query: Query<(Entity, &mut ConnectionTask), With<PolledTask>>,
        commands: ParallelCommands,

        mut events: EventWriter<NetworkErrorEvent>,
        buffer: Local<Mutex<Vec<NetworkErrorEvent>>>,
    ) {
        query.par_iter_mut().for_each(|(entity, mut task)| match task.poll_once() {
            // Do nothing, the task is still running.
            None => {}
            // The task is done, despawn it.
            Some(Ok(())) => {
                #[cfg(debug_assertions)]
                bevy_log::debug!("Connection Closed: {entity:?}");
                commands.command_scope(|mut commands| {
                    commands.entity(entity).despawn_recursive();
                });
            }
            // The task errored, push the error and despawn it.
            Some(Err(error)) => {
                buffer.lock().push(NetworkErrorEvent { entity, error });
                commands.command_scope(|mut commands| {
                    commands.entity(entity).despawn_recursive();
                });
            }
        });

        // Send all errors.
        let mut error_buffer = buffer.lock();
        if !error_buffer.is_empty() {
            events.send_batch(std::mem::take(&mut *error_buffer));
        }
    }

    /// A bevy [`System`](bevy_ecs::system::System) that
    /// polls all [`StatusTask`]s in parallel.
    pub(super) fn poll_status_tasks(
        mut query: Query<(Entity, &mut StatusTask), With<PolledTask>>,
        commands: ParallelCommands,

        mut errors: EventWriter<NetworkErrorEvent>,
        error_buffer: Local<Mutex<Vec<NetworkErrorEvent>>>,
        mut responses: EventWriter<ServerStatusResponse>,
        response_buffer: Local<Mutex<Vec<ServerStatusResponse>>>,
    ) {
        query.par_iter_mut().for_each(|(entity, mut task)| match task.poll_once() {
            // Do nothing, the task is still running.
            None => {}
            // The task is done, despawn it.
            Some(Ok((status, ping))) => {
                response_buffer.lock().push(ServerStatusResponse { entity, status, ping });
                commands.command_scope(|mut commands| {
                    commands.entity(entity).despawn_recursive();
                });
            }
            // The task errored, push the error and despawn it.
            Some(Err(error)) => {
                error_buffer.lock().push(NetworkErrorEvent { entity, error });
                commands.command_scope(|mut commands| {
                    commands.entity(entity).despawn_recursive();
                });
            }
        });

        // Send all errors.
        let mut error_buffer = error_buffer.lock();
        if !error_buffer.is_empty() {
            errors.send_batch(std::mem::take(&mut *error_buffer));
        }

        // Send all responses.
        let mut response_buffer = response_buffer.lock();
        if !response_buffer.is_empty() {
            responses.send_batch(std::mem::take(&mut *response_buffer));
        }
    }
}
