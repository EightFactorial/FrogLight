//! @generated by `froglight-generator` #ecfea09

use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct FeaturesPacket {
    pub field_0: CompactString,
    pub field_1: Vec<()>,
}
