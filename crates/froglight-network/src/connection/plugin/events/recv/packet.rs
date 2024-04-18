use std::sync::Arc;

use bevy_derive::Deref;
use bevy_ecs::event::Event;
use froglight_protocol::{
    states::Play,
    traits::{State, Version},
};

use crate::connection::{NetworkDirection, Serverbound};

/// An event that is sent when a packet is received.
#[derive(Debug, Clone, PartialEq, Deref, Event)]
pub struct RecvPacketEvent<V: Version>(pub Arc<<Serverbound as NetworkDirection<V, Play>>::Recv>)
where
    Play: State<V>,
    Serverbound: NetworkDirection<V, Play>;
