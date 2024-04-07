use froglight_macros::FrogReadWrite;

use crate::common::ServerPlayerAbilityFlags;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct PlayerAbilitiesS2CPacket {
    pub flags: ServerPlayerAbilityFlags,
    pub flying_speed: f32,
    pub walking_speed: f32,
}
