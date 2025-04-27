//! TODO
#![expect(missing_docs)]

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::From;
use froglight_macros::FrogPackets;

mod ping_result;
pub use ping_result::PingResultPacket;

mod query_ping;
pub use query_ping::QueryPingPacket;

#[derive(Debug, Clone, PartialEq, FrogPackets, From)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ClientboundPlayPackets {}

#[derive(Debug, Clone, PartialEq, FrogPackets, From)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ServerboundPlayPackets {}
