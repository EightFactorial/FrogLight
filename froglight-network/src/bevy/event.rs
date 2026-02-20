use bevy_ecs::{entity::Entity, message::Message};
use bevy_reflect::Reflect;

use crate::event::enums::{ClientboundEventEnum, ServerboundEventEnum};

/// A [`Message`] sent from the server to the client.
///
/// Received by [`ClientConnection`](crate::bevy::ClientConnection)s
/// in the order they are sent.
#[derive(Debug, Clone, PartialEq, Message, Reflect)]
#[reflect(Debug, Clone, PartialEq)]
pub struct ClientboundMessage {
    /// The entity associated with the event.
    pub source: Entity,
    /// The event data.
    pub event: ClientboundEventEnum,
}

impl ClientboundMessage {
    /// Create a new [`ClientboundMessage`] from an [`Entity`] and an event.
    #[must_use]
    pub fn new<T: Into<ClientboundEventEnum>>(source: Entity, event: T) -> Self {
        Self { source, event: event.into() }
    }

    /// Get the source [`Entity`] of the message.
    #[inline]
    #[must_use]
    pub const fn source(&self) -> Entity { self.source }

    /// Get a reference to the event data of the message.
    #[inline]
    #[must_use]
    pub const fn event(&self) -> &ClientboundEventEnum { &self.event }

    /// Get a mutable reference to the event data of the message.
    #[inline]
    #[must_use]
    pub const fn event_mut(&mut self) -> &mut ClientboundEventEnum { &mut self.event }
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
    pub target: Entity,
    /// The event data.
    pub event: ServerboundEventEnum,
}

impl ServerboundMessage {
    /// Create a new [`ServerboundMessage`] from an [`Entity`] and an event.
    #[must_use]
    pub fn new<T: Into<ServerboundEventEnum>>(target: Entity, event: T) -> Self {
        Self { target, event: event.into() }
    }

    /// Get the target [`Entity`] of the message.
    #[inline]
    #[must_use]
    pub const fn target(&self) -> Entity { self.target }

    /// Get a reference to the event data of the message.
    #[inline]
    #[must_use]
    pub const fn event(&self) -> &ServerboundEventEnum { &self.event }

    /// Get a mutable reference to the event data of the message.
    #[inline]
    #[must_use]
    pub const fn event_mut(&mut self) -> &mut ServerboundEventEnum { &mut self.event }
}
