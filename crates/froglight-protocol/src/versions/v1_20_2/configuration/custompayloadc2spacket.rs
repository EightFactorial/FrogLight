use froglight_macros::FrogReadWrite;

use super::CustomPayloadS2CPacket;
use crate::common::{ResourceKey, UnsizedByteBuffer};

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
pub struct CustomPayloadC2SPacket {
    pub identifier: ResourceKey,
    pub data: UnsizedByteBuffer,
}

impl From<CustomPayloadS2CPacket> for CustomPayloadC2SPacket {
    fn from(packet: CustomPayloadS2CPacket) -> Self {
        Self { identifier: packet.identifier, data: packet.data }
    }
}
