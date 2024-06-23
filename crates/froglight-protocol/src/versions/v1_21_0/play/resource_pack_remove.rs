use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ResourcePackRemovePacket {
    pub uuid: Option<Uuid>,
}
