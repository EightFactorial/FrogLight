//! @generated by `froglight-generator` #ecfea09

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
pub struct DamageTiltPacket {
    #[frog(var)]
    pub field_0: u32,
    pub field_1: f32,
}
