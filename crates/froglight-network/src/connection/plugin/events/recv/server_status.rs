use bevy_app::{App, PreUpdate};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::{Event, EventReader, EventWriter},
    schedule::{common_conditions::any_with_component, IntoSystemConfigs},
    system::{Commands, Query, Res},
};
use bevy_log::{debug, error, warn};
use bevy_tasks::{block_on, poll_once, IoTaskPool, Task};
use froglight_core::systemsets::NetworkPreUpdateSet;
use froglight_protocol::{packet::ServerStatus, traits::Version};

use crate::{
    connection::{
        plugin::systems::traits::handler::ConnectionHandler, ConnectionError, RequestStatusEvent,
    },
    resolver::Resolver,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.add_event::<ServerStatusEvent>();

    app.add_systems(
        PreUpdate,
        PendingRequestTask::poll_status_tasks
            .run_if(any_with_component::<PendingRequestTask>)
            .in_set(NetworkPreUpdateSet),
    );
}

/// An event that is sent when a server status response is received.
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct ServerStatusEvent {
    /// The entity that requested the server status.
    pub entity: Entity,
    /// The server status.
    pub status: ServerStatus,
}

#[derive(Component)]
pub(crate) struct PendingRequestTask {
    task: Task<Result<ServerStatus, ConnectionError>>,
}

impl PendingRequestTask {
    /// Listens for status events and creates new tasks.
    pub(crate) fn listen_for_status_events<V: Version>(
        mut events: EventReader<RequestStatusEvent>,
        resolver: Res<Resolver>,
        mut commands: Commands,
    ) {
        for event in events.read().filter(|e| e.version_id == V::ID) {
            if let Some(mut entity) = commands.get_entity(event.entity) {
                // Create a new task
                let future =
                    ConnectionHandler::connect_status(event.address.clone(), resolver.clone());
                let task = IoTaskPool::get().spawn(future);

                entity.insert(PendingRequestTask { task });
            } else {
                warn!("Failed to find entity for `RequestStatusEvent`?");
            }
        }
    }

    /// Polls the status tasks and sends the results.
    pub(crate) fn poll_status_tasks(
        mut query: Query<(Entity, &mut PendingRequestTask)>,
        mut events: EventWriter<ServerStatusEvent>,
        mut commands: Commands,
    ) {
        for (entity, mut task) in &mut query {
            // Poll the task
            if let Some(result) = block_on(poll_once(&mut task.task)) {
                match result {
                    Ok(status) => {
                        events.send(ServerStatusEvent { entity, status });
                    }
                    Err(err) => error!("Failed to connect to server: \"{err}\""),
                }

                debug!("Task is done, despawning Entity {entity:?}");

                // Task is done, despawn the entity
                commands.entity(entity).despawn();
            }
        }
    }
}
