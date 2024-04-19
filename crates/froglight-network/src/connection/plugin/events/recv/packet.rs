use std::sync::Arc;

use bevy_derive::Deref;
use bevy_ecs::event::Event;
use froglight_protocol::traits::{State, Version};

use crate::connection::{NetworkDirection, Serverbound};

/// An event that is sent when a packet is received from the server.
#[derive(Debug, Clone, PartialEq, Deref, Event)]
pub struct RecvPacketEvent<V: Version, S: State<V>>(
    pub Arc<<Serverbound as NetworkDirection<V, S>>::Recv>,
)
where
    S: State<V>,
    Serverbound: NetworkDirection<V, S>;

impl<V: Version, S: State<V>> RecvPacketEvent<V, S>
where
    S: State<V>,
    Serverbound: NetworkDirection<V, S>,
{
    /// Creates a new `RecvPacketEvent`.
    #[must_use]
    pub fn new(packet: <Serverbound as NetworkDirection<V, S>>::Recv) -> Self {
        Self(Arc::new(packet))
    }
}
