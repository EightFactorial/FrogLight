use froglight_macros::FrogReadWrite;

use crate::packet::GameEvent;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [3, 0, 0, 0, 0])]
pub struct GameStateChangeS2CPacket {
    pub event: GameEvent,
    pub value: f32,
}
