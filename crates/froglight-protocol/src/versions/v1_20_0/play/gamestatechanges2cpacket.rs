use froglight_macros::FrogReadWrite;

use crate::common::GameStateEvent;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [3, 0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct GameStateChangeS2CPacket {
    pub event: GameStateEvent,
    pub value: f32,
}
