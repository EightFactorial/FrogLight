use bevy_ecs::{entity::Entity, event::EntityEvent, message::Message, reflect::ReflectEvent};
use bevy_reflect::Reflect;

use crate::event::{ClientboundEventEnum, ServerboundEventEnum};

/// An [`Event`] sent from the server to the client.
///
/// Triggered on an [`Entity`] by it's
/// [`ClientConnection`](crate::bevy::ClientConnection)
/// [`Component`](bevy_ecs::component::Component).
#[derive(Debug, Clone, PartialEq, EntityEvent, Reflect)]
#[reflect(Debug, Clone, PartialEq, Event)]
pub struct ClientboundEvent {
    /// The entity associated with the event.
    pub entity: Entity,
    /// The event data.
    pub event: ClientboundEventEnum,
}

impl ClientboundEvent {
    /// Create a new [`ClientboundEvent`] from an [`Entity`] and an event.
    #[inline]
    #[must_use]
    pub fn new<T: Into<ClientboundEventEnum>>(entity: Entity, event: T) -> Self {
        Self { entity, event: event.into() }
    }
}

// -------------------------------------------------------------------------------------------------

/// A [`Message`] sent from the client to the server.
///
/// Sent by the [`Entity`]'s
/// [`ClientConnection`](crate::bevy::ClientConnection)
/// in the order they are received.
#[derive(Debug, Clone, PartialEq, Message, Reflect)]
#[reflect(Debug, Clone, PartialEq)]
pub struct ServerboundMessage {
    /// The entity associated with the message.
    pub entity: Entity,
    /// The event data.
    pub event: ServerboundEventEnum,
}

impl ServerboundMessage {
    /// Create a new [`ServerboundMessage`] from an [`Entity`] and an event.
    pub fn new<T: Into<ServerboundEventEnum>>(entity: Entity, event: T) -> Self {
        Self { entity, event: event.into() }
    }
}
