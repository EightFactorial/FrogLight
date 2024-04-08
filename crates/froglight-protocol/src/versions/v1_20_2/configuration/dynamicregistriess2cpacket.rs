use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Default, Clone, PartialEq, Eq, FrogReadWrite)]
pub struct DynamicRegistriesS2CPacket {
    // TODO: Implement this
    pub registries: UnsizedBuffer,
}
