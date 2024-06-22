use froglight_macros::FrogReadWrite;

use crate::common::GameProfile;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct LoginSuccessPacket {
    pub profile: GameProfile,
    pub strict_error_handling: bool,
}
