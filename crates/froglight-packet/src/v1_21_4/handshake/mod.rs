//! TODO
#![expect(missing_docs, clippy::module_inception)]

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::From;

mod handshake;
pub use handshake::*;

#[derive(Debug, Clone, PartialEq, From)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogPackets))]
pub enum ClientboundHandshakePackets {}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, From)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogPackets))]
pub enum ServerboundHandshakePackets {
    Handshake(HandshakePacket) = 0x0,
}
