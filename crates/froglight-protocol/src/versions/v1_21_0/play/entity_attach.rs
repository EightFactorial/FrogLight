//! @generated by `froglight-generator` #73eaa37

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct EntityAttachPacket {
    pub field_0: i32,
    pub field_1: i32,
}
