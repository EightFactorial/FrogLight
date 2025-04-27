//! TODO
#![expect(missing_docs)]

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::From;
use froglight_macros::FrogPackets;

#[derive(Debug, Clone, PartialEq, FrogPackets, From)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ClientboundLoginPackets {}

#[derive(Debug, Clone, PartialEq, FrogPackets, From)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum ServerboundLoginPackets {}
