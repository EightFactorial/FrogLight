//! @generated by `froglight-generator` #3ae6f0f

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct EntityDamagePacket {
    #[frog(var)]
    pub field_0: u32,
    #[frog(var)]
    pub field_1: u32,
    #[frog(var)]
    pub field_2: u32,
    pub field_3: f64,
    pub field_4: f64,
    pub field_5: f64,
    pub field_6: Option<()>,
}
