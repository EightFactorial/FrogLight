use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

use crate::packet::ClientPlayerAbilityFlags;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite,
)]
pub struct UpdatePlayerAbilitiesC2SPacket {
    pub flags: ClientPlayerAbilityFlags,
}
