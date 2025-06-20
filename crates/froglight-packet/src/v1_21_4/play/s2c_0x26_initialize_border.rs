//! This file is auto-generated. Disable this by adding an `@manual` tag.
//!
//! @generated by {COMMIT_HASH}

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct WorldBorderInitializeS2CPacket {
    pub center_x: f64,
    pub center_z: f64,
    pub size: f64,
    pub size_lerp_target: f64,
    #[cfg_attr(feature = "io", frog(var))]
    pub size_lerp_time: u64,
    #[cfg_attr(feature = "io", frog(var))]
    pub max_radius: u32,
    #[cfg_attr(feature = "io", frog(var))]
    pub warning_blocks: u32,
    #[cfg_attr(feature = "io", frog(var))]
    pub warning_time: u32,
}
