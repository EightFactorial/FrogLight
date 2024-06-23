use froglight_macros::FrogReadWrite;
use glam::Vec2;

use crate::packet::PlayerInputFlags;

#[derive(Debug, Default, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct PlayerInputPacket {
    pub velocity: Vec2,
    pub flags: PlayerInputFlags,
}
