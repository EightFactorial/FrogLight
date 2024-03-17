use froglight_protocol::{
    io::{FrogRead, FrogWrite},
    traits::{PacketEnum, State, Version},
};

/// A trait defining the direction in which packets are sent and received.
pub trait NetworkDirection<V: Version, S: State<V>> {
    /// The packet type that is sent.
    type Send: PacketEnum + FrogWrite;
    /// The packet type that is received.
    type Recv: PacketEnum + FrogRead;
}

/// The clientbound [`NetworkDirection`].
///
/// Packets are sent from the server to the client.
pub struct Clientbound;

impl<V: Version, S: State<V>> NetworkDirection<V, S> for Clientbound
where
    S::ClientboundPacket: FrogWrite,
    S::ServerboundPacket: FrogRead,
{
    type Send = S::ClientboundPacket;
    type Recv = S::ServerboundPacket;
}

/// The serverbound [`NetworkDirection`].
///
/// Packets are sent from the client to the server.
pub struct Serverbound;

impl<V: Version, S: State<V>> NetworkDirection<V, S> for Serverbound
where
    S::ClientboundPacket: FrogRead,
    S::ServerboundPacket: FrogWrite,
{
    type Send = S::ServerboundPacket;
    type Recv = S::ClientboundPacket;
}
