//! @generated by `froglight-generator` #ecfea09

use compact_str::CompactString;
use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite)]
pub struct RenameItemPacket {
    pub field_0: CompactString,
}
