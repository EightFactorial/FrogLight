use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct MapUpdatePacket {
    #[frog(var)]
    pub map_id: u32,
    pub map_scale: u8,
    pub locked: bool,
    // TODO: Implement MapData
    pub map_data: UnsizedBuffer,
    // #[frog(var)]
    // pub field_3: u32,
    // pub field_4: u8,
    // pub field_5: u8,
    // pub field_6: u8,
    // pub field_7: Option<Text>,
    // pub field_8: u8,
    // pub field_9: u8,
    // pub field_10: u8,
    // pub field_11: u8,
    // pub field_12: Vec<u8>,
}
