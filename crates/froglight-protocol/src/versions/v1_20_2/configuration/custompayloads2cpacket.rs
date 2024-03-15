use froglight_macros::FrogReadWrite;

use super::CustomPayloadC2SPacket;
use crate::common::{ResourceKey, UnsizedByteBuffer};

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct CustomPayloadS2CPacket {
    pub identifier: ResourceKey,
    pub data: UnsizedByteBuffer,
}

impl From<CustomPayloadC2SPacket> for CustomPayloadS2CPacket {
    fn from(packet: CustomPayloadC2SPacket) -> Self {
        Self { identifier: packet.identifier, data: packet.data }
    }
}
