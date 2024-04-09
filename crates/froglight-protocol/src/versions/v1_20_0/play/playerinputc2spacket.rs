use froglight_macros::FrogReadWrite;
use glam::Vec2;

use crate::packet::PlayerInputFlags;

#[derive(Debug, Default, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct PlayerInputC2SPacket {
    pub velocity: Vec2,
    pub flags: PlayerInputFlags,
}
