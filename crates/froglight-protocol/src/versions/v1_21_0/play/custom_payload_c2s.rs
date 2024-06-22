use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct CustomPayloadC2SPacket {
    #[frog(var)]
    pub id: u32,
    pub payload: Option<UnsizedBuffer>,
}
