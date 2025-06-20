//! This file is auto-generated. Disable this by adding an `@manual` tag.
//!
//! @generated by {COMMIT_HASH}

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::prelude::Identifier;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct StopSoundS2CPacket {
    pub sound_id: Identifier,
    pub category: (),
    pub unknown: i8,
}
