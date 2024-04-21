use std::sync::Arc;

use bevy_ecs::{entity::Entity, event::Event};
use froglight_protocol::traits::{State, Version};

use crate::connection::{NetworkDirection, Serverbound};

/// An [`Event`] that sends a packet through a
/// [`Connection`](crate::connection::Connection).
#[derive(Debug, Clone, PartialEq, Event)]
pub struct SendPacket<V: Version, S: State<V>>
where
    Serverbound: NetworkDirection<V, S>,
{
    /// The packet to send.
    pub packet: Arc<<Serverbound as NetworkDirection<V, S>>::Send>,
    /// The connection to send the packet through.
    pub connection: Option<Entity>,
}

impl<V: Version, S: State<V>> SendPacket<V, S>
where
    Serverbound: NetworkDirection<V, S>,
{
    /// Create a new [`SendPacket`] event.
    #[must_use]
    pub const fn new(
        packet: Arc<<Serverbound as NetworkDirection<V, S>>::Send>,
        connection: Option<Entity>,
    ) -> Self {
        Self { packet, connection }
    }

    /// Create a new [`SendPacket`] event from a packet
    #[must_use]
    pub fn from_packet(
        packet: <Serverbound as NetworkDirection<V, S>>::Send,
        connection: Option<Entity>,
    ) -> Self {
        Self::new(Arc::new(packet), connection)
    }
}

/// A packet received through a [`Connection`](crate::connection::Connection).
#[derive(Debug, Clone, PartialEq, Event)]
pub struct RecvPacket<V: Version, S: State<V>>
where
    Serverbound: NetworkDirection<V, S>,
{
    /// The packet that was received.
    pub packet: Arc<<Serverbound as NetworkDirection<V, S>>::Recv>,
    /// The connection that received the packet.
    pub connection: Entity,
}

impl<V: Version, S: State<V>> RecvPacket<V, S>
where
    Serverbound: NetworkDirection<V, S>,
{
    /// Create a new [`RecvPacket`] event.
    #[must_use]
    pub const fn new(
        packet: Arc<<Serverbound as NetworkDirection<V, S>>::Recv>,
        connection: Entity,
    ) -> Self {
        Self { packet, connection }
    }

    /// Create a new [`RecvPacket`] event from a packet
    #[must_use]
    pub fn from_packet(
        packet: <Serverbound as NetworkDirection<V, S>>::Recv,
        connection: Entity,
    ) -> Self {
        Self::new(Arc::new(packet), connection)
    }

    /// Check if the packet is from a specific connection.
    #[must_use]
    pub fn is_from(&self, connection: Entity) -> bool { self.connection == connection }
}
