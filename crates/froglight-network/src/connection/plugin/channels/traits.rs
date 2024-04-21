use froglight_protocol::{
    states::Login,
    traits::{State, Version},
};

use super::parts::TaskPair;
use crate::connection::{NetworkDirection, Serverbound};

/// A trait for a packet channel that can be paired with a task channel.
pub trait PacketChannelTrait<V: Version>: Sized
where
    Serverbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    /// The type of the task channel that can be paired with this packet
    /// channel.
    type TaskHalf: TaskChannelTrait<V> + Send + Sync + Sized;

    /// Create a new pair of packet and task channels.
    fn new() -> (Self, Self::TaskHalf);
}

/// A trait for a task channel that can be paired with a packet channel.
pub trait TaskChannelTrait<V: Version>: Sized
where
    Serverbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    /// Gets the login [`TaskPair`].
    fn login(&self) -> &TaskPair<V, Login>;
}
