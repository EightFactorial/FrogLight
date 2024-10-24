//! @generated by `froglight-generator` #8ddd9f0

use froglight_macros::FrogReadWrite;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct EntitySpawnPacket {
    #[frog(var)]
    pub field_0: u32,
    pub field_1: Uuid,
    pub field_2: f64,
    pub field_3: f64,
    pub field_4: f64,
    pub field_5: u8,
    pub field_6: u8,
    pub field_7: u8,
    #[frog(var)]
    pub field_8: u32,
    pub field_9: i16,
    pub field_10: i16,
    pub field_11: i16,
}
