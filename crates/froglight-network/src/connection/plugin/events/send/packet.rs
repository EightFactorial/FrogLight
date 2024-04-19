use std::sync::Arc;

use bevy_derive::Deref;
use bevy_ecs::{entity::Entity, event::Event};
use froglight_protocol::traits::{State, Version};

use crate::connection::{NetworkDirection, Serverbound};

/// An event sends a packet to the server.
#[derive(Debug, Clone, PartialEq, Deref, Event)]
pub struct SendPacketEvent<V: Version, S: State<V>>
where
    S: State<V>,
    Serverbound: NetworkDirection<V, S>,
{
    /// The packet to send.
    #[deref]
    pub packet: Arc<<Serverbound as NetworkDirection<V, S>>::Send>,
    /// The entity with the channel to send the packet.
    pub entity: Option<Entity>,
}
