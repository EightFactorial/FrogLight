use froglight_macros::FrogReadWrite;

use crate::common::UnsizedByteBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct MapUpdateS2CPacket {
    #[frog(var)]
    pub map_id: u32,
    pub map_scale: u8,
    pub locked: bool,
    pub map_data: UnsizedByteBuffer,
}
