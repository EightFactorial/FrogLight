use froglight_macros::FrogReadWrite;

use crate::packet::ServerPlayerAbilityFlags;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct PlayerAbilitiesS2CPacket {
    pub flags: ServerPlayerAbilityFlags,
    pub flying_speed: f32,
    pub walking_speed: f32,
}
