//! @generated by `froglight-generator` #ecfea09

use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite)]
pub struct DebugSampleSubscriptionPacket {
    pub field_0: Enum,
}
