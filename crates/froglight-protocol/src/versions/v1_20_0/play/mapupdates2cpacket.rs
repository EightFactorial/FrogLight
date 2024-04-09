use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct MapUpdateS2CPacket {
    #[frog(var)]
    pub map_id: u32,
    pub map_scale: u8,
    pub locked: bool,
    pub map_data: UnsizedBuffer,
}
