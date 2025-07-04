//! This file is auto-generated. Disable this by adding an `@manual` tag.
//!
//! @generated by {COMMIT_HASH}

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct ExperienceBarUpdateS2CPacket {
    pub bar_progress: f32,
    #[cfg_attr(feature = "io", frog(var))]
    pub experience: u32,
    #[cfg_attr(feature = "io", frog(var))]
    pub experience_level: u32,
}
