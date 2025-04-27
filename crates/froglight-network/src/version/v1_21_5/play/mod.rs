//! TODO
#![expect(missing_docs)]

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::From;
use froglight_macros::FrogPackets;

pub use crate::version::v1_21_4::play::{PingResultPacket, QueryPingPacket};

#[derive(Debug, Clone, PartialEq, FrogPackets, From)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ClientboundPlayPackets {}

#[derive(Debug, Clone, PartialEq, FrogPackets, From)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ServerboundPlayPackets {}
