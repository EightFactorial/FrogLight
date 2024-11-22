//! Channel module for network communication.
//!
//! Handles sending packets between bevy and a connection task.

mod connection;
pub use connection::{new as channel, AsyncConnectionChannel, BevyConnectionChannel};

mod holder;
pub use holder::ConnectionHolder;

mod packet;
pub use packet::{ChannelRecvPacket, ChannelSendPacket};
