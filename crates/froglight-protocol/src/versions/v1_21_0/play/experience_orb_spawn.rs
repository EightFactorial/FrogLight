//! @generated by `froglight-generator` #ecfea09

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
pub struct ExperienceOrbSpawnPacket {
    #[frog(var)]
    pub field_0: u32,
    pub field_1: f64,
    pub field_2: f64,
    pub field_3: f64,
    pub field_4: i16,
}
