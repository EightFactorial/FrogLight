use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct CustomPayloadS2CPacket {
    #[frog(var)]
    pub id: u32,
    pub payload: Option<UnsizedBuffer>,
}
