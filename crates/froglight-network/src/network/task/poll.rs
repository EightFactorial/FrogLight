use bevy_app::{App, PostUpdate};
use bevy_ecs::prelude::*;
use bevy_hierarchy::DespawnRecursiveExt;
use bevy_reflect::{std_traits::ReflectDefault, Reflect};
use parking_lot::Mutex;

use super::{
    ConnectionClosedEvent, ConnectionErrorEvent, ConnectionTask, StatusResponseEvent, StatusTask,
};
use crate::network::NetworkPostUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<PolledTask>();

    app.add_systems(
        PostUpdate,
        PolledTask::poll_tasks
            .run_if(any_with_component::<PolledTask>)
            .in_set(NetworkPostUpdateSet),
    );
}

/// A marker component for tasks that are automatically polled.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, Reflect, Component)]
#[reflect(Default, Component)]
pub struct PolledTask;

#[allow(clippy::too_many_arguments)]
impl PolledTask {
    /// Poll all [`ConnectionTask`]s and [`StatusTask`]s,
    /// if they are marked [`PolledTask`].
    #[expect(clippy::type_complexity)]
    fn poll_tasks(
        mut query: Query<
            (Entity, Option<&mut ConnectionTask>, Option<&mut StatusTask>),
            With<Self>,
        >,
        commands: ParallelCommands,

        mut conn_events: EventWriter<ConnectionClosedEvent>,
        conn_buffer: Local<Mutex<Vec<ConnectionClosedEvent>>>,

        mut status_events: EventWriter<StatusResponseEvent>,
        status_buffer: Local<Mutex<Vec<StatusResponseEvent>>>,

        mut error_events: EventWriter<ConnectionErrorEvent>,
        error_buffer: Local<Mutex<Vec<ConnectionErrorEvent>>>,
    ) {
        query.par_iter_mut().for_each(|(entity, connection, status)| {
            // Poll all `ConnectionTask`s
            if let Some(result) = connection.and_then(|mut c| c.poll(entity)) {
                match result {
                    Ok(event) => conn_buffer.lock().push(event),
                    Err(error) => error_buffer.lock().push(error),
                }
                // Despawn the entity
                commands.command_scope(|mut commands| {
                    commands.entity(entity).despawn_recursive();
                });
            }

            // Poll all `StatusTask`s
            if let Some(result) = status.and_then(|mut s| s.poll(entity)) {
                match result {
                    Ok(event) => status_buffer.lock().push(event),
                    Err(error) => error_buffer.lock().push(error),
                }
                // Despawn the entity
                commands.command_scope(|mut commands| {
                    commands.entity(entity).despawn_recursive();
                });
            }
        });

        // Send all buffered events
        for event in conn_buffer.lock().drain(..) {
            conn_events.send(event);
        }
        for event in status_buffer.lock().drain(..) {
            status_events.send(event);
        }
        for error in error_buffer.lock().drain(..) {
            error_events.send(error);
        }
    }
}
