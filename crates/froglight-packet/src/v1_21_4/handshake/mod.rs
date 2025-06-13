//! TODO
#![expect(missing_docs)]

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{From, TryInto, TryUnwrap};


pub(super) mod c2s_0x00_intention;
pub use c2s_0x00_intention::HandshakeC2SPacket;


#[repr(u8)]
#[derive(Debug, Clone, PartialEq, From, TryInto, TryUnwrap)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogPackets))]
pub enum ClientboundHandshakePackets {
    Handshake(HandshakeC2SPacket) = 0x00,
}


#[derive(Debug, Clone, PartialEq, From, TryInto, TryUnwrap)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogPackets))]
pub enum ServerboundHandshakePackets {
}
