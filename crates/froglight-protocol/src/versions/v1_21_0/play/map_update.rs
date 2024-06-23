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
}
