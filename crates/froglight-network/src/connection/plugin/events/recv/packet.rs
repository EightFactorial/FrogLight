use std::sync::Arc;

use bevy_derive::Deref;
use bevy_ecs::{entity::Entity, event::Event};
use froglight_protocol::traits::{State, Version};

use crate::connection::{NetworkDirection, Serverbound};

/// An event that is sent when a packet is received from the server.
#[derive(Debug, Clone, PartialEq, Deref, Event)]
pub struct RecvPacketEvent<V: Version, S: State<V>>
where
    S: State<V>,
    Serverbound: NetworkDirection<V, S>,
{
    /// The packet that was received.
    #[deref]
    pub packet: Arc<<Serverbound as NetworkDirection<V, S>>::Recv>,
    /// The entity that received the packet.
    pub entity: Entity,
}

impl<V: Version, S: State<V>> RecvPacketEvent<V, S>
where
    S: State<V>,
    Serverbound: NetworkDirection<V, S>,
{
    /// Creates a new `RecvPacketEvent`.
    #[must_use]
    pub fn new(packet: Arc<<Serverbound as NetworkDirection<V, S>>::Recv>, entity: Entity) -> Self {
        Self { packet, entity }
    }

    /// Creates a new `RecvPacketEvent` with a packet that is not shared.
    #[must_use]
    pub fn new_packet(
        packet: <Serverbound as NetworkDirection<V, S>>::Recv,
        entity: Entity,
    ) -> Self {
        Self::new(Arc::new(packet), entity)
    }
}
