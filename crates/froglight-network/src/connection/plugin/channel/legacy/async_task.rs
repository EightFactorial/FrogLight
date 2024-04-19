use std::sync::Arc;

use async_channel::{Receiver, RecvError, SendError, Sender};
use froglight_protocol::{
    states::Play,
    traits::{State, Version},
};

use super::LegacyChannel;
use crate::connection::{ConnectionError, NetworkDirection, Serverbound};

#[derive(Clone)]
pub struct LegacyPacketChannel<V: Version>
where
    Serverbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    pub(crate) receiver: Receiver<Arc<<Serverbound as NetworkDirection<V, Play>>::Send>>,
    pub(crate) sender: Sender<Arc<<Serverbound as NetworkDirection<V, Play>>::Recv>>,
    pub(crate) errors: Sender<ConnectionError>,
}

impl<V: Version> LegacyPacketChannel<V>
where
    Serverbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    pub(crate) fn new_channel() -> (LegacyChannel<V>, Self) { LegacyChannel::new_channel() }

    /// Sends an error to bevy.
    pub(crate) async fn send_error(
        &self,
        error: ConnectionError,
    ) -> Result<(), SendError<ConnectionError>> {
        self.errors.send(error).await
    }

    /// Sends a play packet to bevy.
    pub(crate) async fn send_packet(
        &self,
        packet: Arc<<Serverbound as NetworkDirection<V, Play>>::Recv>,
    ) -> Result<(), SendError<Arc<<Serverbound as NetworkDirection<V, Play>>::Recv>>> {
        self.sender.send(packet).await
    }

    /// Receives a play packet from bevy.
    pub(crate) async fn recv_packet(
        &self,
    ) -> Result<Arc<<Serverbound as NetworkDirection<V, Play>>::Send>, RecvError> {
        self.receiver.recv().await
    }
}
