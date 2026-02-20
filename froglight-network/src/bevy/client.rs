use core::error::Error;

use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::EntityEvent,
    lifecycle::HookContext,
    reflect::ReflectEvent,
    world::{DeferredWorld, EntityRef},
};
use bevy_reflect::Reflect;
use bevy_tasks::Task;

use crate::{
    bevy::NetworkVersion,
    connection::{ConnectionError, Runtime},
    event::enums::{ClientboundEventEnum, ServerboundEventEnum},
};

/// The client-side end of a network connection.
///
/// Sends [`ServerboundEventEnum`]s to the server and receives
/// [`ClientboundEventEnum`]s from the server.
#[derive(Component)]
#[component(on_despawn = ClientDespawn::connection_despawn_hook)]
pub struct ClientConnection {
    sender: Box<SenderFn>,
    receiver: Box<ReceiverFn>,
    task: Task<Result<(), Box<dyn Error + Send + Sync>>>,
}

type SenderFn = dyn for<'a> Fn(ServerboundEventEnum, EntityRef<'a>) -> Result<(), ConnectionError>
    + Send
    + Sync;
type ReceiverFn = dyn for<'a> Fn(EntityRef<'a>) -> Result<Option<ClientboundEventEnum>, ConnectionError>
    + Send
    + Sync;

impl ClientConnection {
    /// Create a new [`ClientConnection`] using the given connection.
    #[inline]
    #[must_use]
    pub fn new<V: NetworkVersion, R: Runtime<C>, C: Send>(
        connection: C,
        exit_on_error: bool,
    ) -> Self {
        V::wrap_connection::<R, C>(connection, exit_on_error)
    }

    /// Create a new [`ClientConnection`] from the given
    /// sender and receiver functions and [`Task`].
    ///
    /// This is typically used internally by
    /// [`NetworkVersion::wrap_connection`].
    #[inline]
    #[must_use]
    pub const fn new_from_parts(
        sender: Box<SenderFn>,
        receiver: Box<ReceiverFn>,
        task: Task<Result<(), Box<dyn Error + Send + Sync>>>,
    ) -> Self {
        Self { sender, receiver, task }
    }

    /// Send a [`ServerboundEventEnum`] to the server.
    ///
    /// # Errors
    ///
    /// Returns a [`ConnectionError`] if the event cannot be sent.
    #[inline]
    pub fn send<T: Into<ServerboundEventEnum>>(
        &self,
        event: T,
        entity: EntityRef<'_>,
    ) -> Result<(), ConnectionError> {
        (self.sender)(event.into(), entity)
    }

    /// Receive a [`ClientboundEventEnum`] from the server.
    ///
    /// Returns `None` if there are no events to receive.
    ///
    /// # Errors
    ///
    /// Returns a [`ConnectionError`] if an event cannot be received.
    #[inline]
    pub fn receive(
        &self,
        entity: EntityRef<'_>,
    ) -> Result<Option<ClientboundEventEnum>, ConnectionError> {
        (self.receiver)(entity)
    }

    /// Poll the connection [`Task`] for completion.
    ///
    /// Returns `None` if the task is still running.
    ///
    /// # Warning
    ///
    /// If the task has been completed, this component should be
    /// removed from the ECS to avoid polling it again!
    ///
    /// # Errors
    ///
    /// Returns an error if the task has encountered an error.
    pub fn poll_task(&mut self) -> Option<Result<(), Box<dyn Error + Send + Sync>>> {
        futures_lite::future::block_on(futures_lite::future::poll_once(&mut self.task))
    }
}

// -------------------------------------------------------------------------------------------------

/// An [`EntityEvent`] triggered when a [`ClientConnection`] is despawned.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EntityEvent, Reflect)]
#[reflect(Debug, Clone, PartialEq, Hash, Event)]
pub struct ClientDespawn(pub Entity);

impl ClientDespawn {
    /// Create a new [`ClientDespawn`] event for the given [`Entity`].
    #[inline]
    #[must_use]
    pub const fn new(entity: Entity) -> Self { Self(entity) }

    /// Get the [`Entity`] associated with this event.
    #[inline]
    #[must_use]
    pub const fn entity(&self) -> Entity { self.0 }

    fn connection_despawn_hook(mut world: DeferredWorld, ctx: HookContext) {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_network", "Triggering `ClientDespawn` for Entity {}", ctx.entity);
        world.trigger(ClientDespawn::new(ctx.entity));
    }
}
