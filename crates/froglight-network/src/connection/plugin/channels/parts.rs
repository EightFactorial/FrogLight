use std::sync::Arc;

use async_channel::{Receiver, Sender};
use froglight_protocol::traits::{State, Version};

use crate::connection::{NetworkDirection, Serverbound};

const CHANNEL_SIZE: usize = 16;

/// Create a new pair of packet and task channels.
pub(crate) fn new_pair<V: Version, S: State<V>>() -> (PacketPair<V, S>, TaskPair<V, S>)
where
    Serverbound: NetworkDirection<V, S>,
{
    let (bevy_send, connection_recv) = async_channel::bounded(CHANNEL_SIZE);
    let (connection_send, bevy_recv) = async_channel::bounded(CHANNEL_SIZE);

    (
        PacketPair { send: bevy_send, recv: bevy_recv },
        TaskPair { send: connection_recv, recv: connection_send },
    )
}

/// A pair of packet channels.
///
/// This is used to send and receive packets between Bevy
/// and the connection task.
#[derive(Debug, Clone)]
pub struct PacketPair<V: Version, S: State<V>>
where
    Serverbound: NetworkDirection<V, S>,
{
    /// Send packets to the connection task
    pub send: Sender<Arc<<Serverbound as NetworkDirection<V, S>>::Send>>,
    /// Receive packets from the connection task
    pub recv: Receiver<Arc<<Serverbound as NetworkDirection<V, S>>::Recv>>,
}

/// A pair of task channels.
///
/// This is used to send and receive a specifc type of packet between the
/// connection task and the server.
#[derive(Debug, Clone)]
pub struct TaskPair<V: Version, S: State<V>>
where
    Serverbound: NetworkDirection<V, S>,
{
    /// Receive packets and forward them to the server
    pub send: Receiver<Arc<<Serverbound as NetworkDirection<V, S>>::Send>>,
    /// Receive packets from the server and forward them to bevy
    pub recv: Sender<Arc<<Serverbound as NetworkDirection<V, S>>::Recv>>,
}
