use froglight_macros::FrogReadWrite;

use crate::common::{ResourceKey, UnsizedByteBuffer};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [15, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 98, 114, 97, 110, 100, 5, 77, 67, 45, 82, 83])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct CustomPayloadS2CPacket {
    pub identifier: ResourceKey,
    pub data: UnsizedByteBuffer,
}
