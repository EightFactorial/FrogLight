//! @generated by `froglight-generator` #ecfea09

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
pub struct GameStateChangePacket {
    pub field_0: u8,
    pub field_1: f32,
}
