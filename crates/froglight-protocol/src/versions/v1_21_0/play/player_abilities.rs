use froglight_macros::FrogReadWrite;

use crate::packet::ServerPlayerAbilityFlags;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct PlayerAbilitiesPacket {
    pub flags: ServerPlayerAbilityFlags,
    pub flying_speed: f32,
    pub walking_speed: f32,
}
