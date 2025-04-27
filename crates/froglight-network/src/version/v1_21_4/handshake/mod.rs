//! TODO
#![expect(missing_docs, clippy::module_inception)]

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::From;
use froglight_macros::FrogPackets;

mod handshake;
pub use handshake::*;

#[derive(Debug, Clone, PartialEq, FrogPackets, From)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ClientboundHandshakePackets {}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, FrogPackets, From)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ServerboundHandshakePackets {
    Handshake(HandshakePacket) = 0x0,
}
