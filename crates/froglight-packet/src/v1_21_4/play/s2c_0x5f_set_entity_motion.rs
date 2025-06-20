//! This file is auto-generated. Disable this by adding an `@manual` tag.
//!
//! @generated by {COMMIT_HASH}

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct EntityVelocityUpdateS2CPacket {
    #[cfg_attr(feature = "io", frog(var))]
    pub entity_id: u32,
    pub velocity_x: u16,
    pub velocity_y: u16,
    pub velocity_z: u16,
}
