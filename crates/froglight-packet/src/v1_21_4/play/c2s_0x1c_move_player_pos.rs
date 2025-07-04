//! This file is auto-generated. Disable this by adding an `@manual` tag.
//!
//! @manual @generated by {COMMIT_HASH}

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_utils::bitset::FixedBitSet;
use glam::DVec3;

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct PlayerPositionAndOnGroundC2SPacket {
    pub position: DVec3,
    pub collision: FixedBitSet<2>,
}
