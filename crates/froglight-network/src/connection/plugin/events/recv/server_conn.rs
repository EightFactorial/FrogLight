use bevy_app::App;
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::{Event, EventReader, EventWriter},
    system::{Commands, Query, Res},
};
use bevy_log::{debug, error, warn};
use bevy_tasks::{block_on, poll_once, Task};
use froglight_protocol::{
    states::{Handshaking, Login, Play, Status},
    traits::{State, Version},
};

use crate::{
    connection::{
        plugin::systems::traits::handler::ConnectionHandler, ConnectionBundle, ConnectionError,
        NetworkDirection, RequestConnectionEvent, Serverbound,
    },
    resolver::Resolver,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.add_event::<ServerConnectionEvent>(); }

/// An event sent when a new server connection is created.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub struct ServerConnectionEvent {
    /// The entity that requested the server connection.
    pub entity: Entity,
}

#[derive(Component)]
pub(crate) struct PendingConnectionTask<V: Version + ConnectionHandler>
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
    task: Task<Result<ConnectionBundle<V>, ConnectionError>>,
}

impl<V: Version + ConnectionHandler> PendingConnectionTask<V>
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
    pub(crate) fn listen_for_requestconn_event(
        mut events: EventReader<RequestConnectionEvent>,
        resolver: Res<Resolver>,
        mut commands: Commands,
    ) {
        for event in events.read().filter(|e| e.version_id == V::ID) {
            if let Some(mut entity) = commands.get_entity(event.entity) {
                // Create a new task
                let future = V::connect_game(event.address.clone(), resolver.clone());
                let task = bevy_tasks::IoTaskPool::get().spawn(future);

                entity.insert(PendingConnectionTask { task });
            } else {
                warn!("Failed to find entity for `ServerConnectionEvent`?");
            }
        }
    }

    pub(crate) fn poll_conn_tasks(
        mut query: Query<(Entity, &mut PendingConnectionTask<V>)>,
        mut events: EventWriter<ServerConnectionEvent>,
        mut commands: Commands,
    ) where
        Serverbound: NetworkDirection<V, Handshaking>
            + NetworkDirection<V, Status>
            + NetworkDirection<V, Login>
            + NetworkDirection<V, Play>,
        Handshaking: State<V>,
        Status: State<V>,
        Login: State<V>,
        Play: State<V>,
    {
        for (entity, mut task) in &mut query {
            // Poll the task
            if let Some(result) = block_on(poll_once(&mut task.task)) {
                let mut entity_commands = commands.entity(entity);

                match result {
                    Ok(bundle) => {
                        events.send(ServerConnectionEvent { entity });
                        entity_commands.insert(bundle);
                    }
                    Err(err) => error!("Failed to connect to server: \"{err}\""),
                }

                debug!("Task is done, despawning Entity {entity:?}");

                // Task is done, despawn the entity
                entity_commands.despawn();
            }
        }
    }
}
