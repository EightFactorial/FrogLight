use bevy_math::Vec2;
use froglight_macros::FrogReadWrite;

use crate::common::PlayerInputFlags;

#[derive(Debug, Default, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct PlayerInputC2SPacket {
    pub velocity: Vec2,
    pub flags: PlayerInputFlags,
}
