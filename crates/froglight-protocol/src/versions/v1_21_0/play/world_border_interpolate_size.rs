//! @generated by `froglight-generator` #ecfea09

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
pub struct WorldBorderInterpolateSizePacket {
    pub field_0: f64,
    pub field_1: f64,
    #[frog(var)]
    pub field_2: u64,
}
