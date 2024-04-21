//! Channels for sending and receiving packets.

pub(crate) mod current;
pub use current::{PacketChannel, TaskChannel};

pub(crate) mod legacy;
pub use legacy::{LegacyPacketChannel, LegacyTaskChannel};

pub(crate) mod parts;
pub use parts::{PacketPair, TaskPair};

pub(crate) mod traits;
pub use traits::{PacketChannelTrait, TaskChannelTrait};
