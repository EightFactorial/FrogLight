//! @generated by `froglight-generator` #ecfea09

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct ProfilelessChatMessagePacket {
    pub field_0: Text,
    #[frog(var)]
    pub field_1: u32,
    pub field_2: Text,
    pub field_3: Option<Text>,
}
