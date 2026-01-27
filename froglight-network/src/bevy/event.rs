use bevy_ecs::{entity::Entity, message::Message};
use bevy_reflect::Reflect;

use crate::event::{ClientboundEventEnum, ServerboundEventEnum};

/// A [`Message`] sent from the server to the client.
///
/// Received by [`ClientConnection`](crate::bevy::ClientConnection)s
/// in the order they are sent.
#[derive(Debug, Clone, PartialEq, Message, Reflect)]
#[reflect(Debug, Clone, PartialEq)]
pub struct ClientboundMessage {
    /// The entity associated with the event.
    pub entity: Entity,
    /// The event data.
    pub event: ClientboundEventEnum,
}

impl ClientboundMessage {
    /// Create a new [`ClientboundEvent`] from an [`Entity`] and an event.
    #[must_use]
    pub fn new<T: Into<ClientboundEventEnum>>(entity: Entity, event: T) -> Self {
        Self { entity, event: event.into() }
    }
}

// -------------------------------------------------------------------------------------------------

/// A [`Message`] sent from the client to the server.
///
/// Sent by [`ClientConnection`](crate::bevy::ClientConnection)s
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
    #[must_use]
    pub fn new<T: Into<ServerboundEventEnum>>(entity: Entity, event: T) -> Self {
        Self { entity, event: event.into() }
    }
}
