use std::marker::PhantomData;

use bevy_app::{App, PostUpdate};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::{Event, EventReader, EventWriter},
    schedule::{
        common_conditions::{any_with_component, on_event},
        IntoSystemConfigs,
    },
    system::{Commands, Query, Res},
};
use bevy_log::error;
use bevy_tasks::{IoTaskPool, Task};
use compact_str::CompactString;
use froglight_core::common::ServerStatus;
use froglight_protocol::{
    states::{Handshaking, Status},
    traits::{State, Version},
};

use super::{versions::Queryable, NetworkStatusVersionSet};
use crate::{resolver::Resolver, ConnectionError, NetworkDirection, Serverbound};

/// An [`Event`] that sends a status request to a server.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub struct StatusRequest<V: Queryable>
where
    Handshaking: State<V>,
    Serverbound: NetworkDirection<V, Handshaking>,

    Status: State<V>,
    Serverbound: NetworkDirection<V, Status>,
{
    /// The entity that is sending the request.
    pub entity: Entity,
    /// The URL to query.
    pub address: CompactString,
    pub(crate) _phantom: PhantomData<V>,
}

impl<V: Queryable> StatusRequest<V>
where
    Handshaking: State<V>,
    Serverbound: NetworkDirection<V, Handshaking>,

    Status: State<V>,
    Serverbound: NetworkDirection<V, Status>,
{
    /// Create a new [`StatusRequest`] with the given URL.
    #[must_use]
    pub fn new(entity: Entity, address: &(impl AsRef<str> + ?Sized)) -> Self {
        Self { entity, address: address.as_ref().into(), _phantom: PhantomData }
    }

    pub(super) fn build(app: &mut App) {
        app.add_event::<Self>();

        app.add_systems(
            PostUpdate,
            (
                Self::receive_requests.run_if(on_event::<Self>()).ambiguous_with_all(),
                Self::receive_responses
                    .run_if(any_with_component::<StatusRequestTask<V>>)
                    .ambiguous_with_all(),
            )
                .chain()
                .in_set(NetworkStatusVersionSet::<V>::default()),
        );
    }

    fn receive_requests(
        mut events: EventReader<Self>,
        resolver: Res<Resolver>,
        mut commands: Commands,
    ) {
        let pool = IoTaskPool::get();
        for event in events.read() {
            let task = pool.spawn(V::get_status(event.clone(), resolver.clone()));
            commands
                .entity(event.entity)
                .insert(StatusRequestTask { task, _phantom: PhantomData::<V> });
        }
    }

    fn receive_responses(
        mut query: Query<(Entity, &mut StatusRequestTask<V>)>,
        mut events: EventWriter<StatusResponse>,
        mut commands: Commands,
    ) {
        for (entity, mut task) in &mut query {
            if let Some(response) = task.poll_once() {
                let mut commands = commands.entity(entity);

                match response {
                    Ok(ping) => {
                        #[cfg(debug_assertions)]
                        bevy_log::debug!("Received status response: {ping:?}");
                        events.send(ping);
                    }
                    Err(err) => {
                        error!("Connection error during status: {err}");
                    }
                }

                commands.remove::<StatusRequestTask<V>>();
            }
        }
    }
}

#[derive(Debug, Component)]
pub(crate) struct StatusRequestTask<V: Version> {
    task: Task<Result<StatusResponse, ConnectionError>>,
    _phantom: PhantomData<V>,
}

impl<V: Version> StatusRequestTask<V> {
    pub(crate) fn poll_once(&mut self) -> Option<Result<StatusResponse, ConnectionError>> {
        bevy_tasks::block_on(bevy_tasks::poll_once(&mut self.task))
    }
}

/// An [`Event`] that is received in response to a [`StatusRequest`].
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct StatusResponse {
    /// The entity that sent the request.
    pub entity: Entity,
    /// The URL that was queried.
    pub url: CompactString,
    /// The status of the server.
    pub status: ServerStatus,
}
