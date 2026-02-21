use froglight_player::prelude::PlayerProfile;

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct LoginSuccessS2CPacket {
    pub profile: PlayerProfile,
}

impl LoginSuccessS2CPacket {
    /// Creates a new [`LoginSuccessS2CPacket`] with the given
    /// [`PlayerProfile`].
    #[inline]
    #[must_use]
    pub const fn new(profile: PlayerProfile) -> Self { Self { profile } }
}
