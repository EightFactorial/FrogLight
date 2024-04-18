use std::sync::Arc;

use bevy_derive::Deref;
use bevy_ecs::event::Event;
use froglight_protocol::{
    states::Play,
    traits::{State, Version},
};

use crate::connection::{NetworkDirection, Serverbound};

/// An event that is consumed to send a packet.
#[derive(Debug, Clone, PartialEq, Deref, Event)]
pub struct SendPacketEvent<V: Version>(pub Arc<<Serverbound as NetworkDirection<V, Play>>::Send>)
where
    Play: State<V>,
    Serverbound: NetworkDirection<V, Play>;
