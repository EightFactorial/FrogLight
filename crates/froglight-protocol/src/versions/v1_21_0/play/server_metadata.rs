//! @generated by `froglight-generator` #3ae6f0f

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ServerMetadataPacket {
    pub field_0: Text,
    pub field_1: Vec<u8>,
}
