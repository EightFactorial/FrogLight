//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:login_finished"

use froglight_player::prelude::PlayerProfile;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct LoginFinishedS2CPacket {
    pub profile: PlayerProfile,
}

impl LoginFinishedS2CPacket {
    /// Creates a new [`LoginFinishedS2CPacket`] with the given
    /// [`PlayerProfile`].
    #[inline]
    #[must_use]
    pub const fn new(profile: PlayerProfile) -> Self { Self { profile } }
}
