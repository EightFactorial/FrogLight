use crate::buffer::{Decode, Encode};

/// A Minecraft protocol version.
pub trait Version: Sized + Send + Sync + 'static {
    /// The protocol id of this version.
    const ID: i32;
}

/// A state in the Minecraft protocol.
pub trait State<V: Version>: Sized + Send + Sync + 'static {
    /// Packets sent from the server to the client.
    type Clientbound: Decode + Send + Sync;

    /// Packets sent from the client to the server.
    type Serverbound: Encode + Send + Sync;
}
