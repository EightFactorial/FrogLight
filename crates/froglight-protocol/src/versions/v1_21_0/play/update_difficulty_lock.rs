//! @generated by `froglight-generator` #73eaa37

use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct UpdateDifficultyLockPacket {
    pub field_0: bool,
}
