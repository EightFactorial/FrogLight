use derive_more::{Deref, DerefMut, From, Into};
use froglight_common::EntityId;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct SetCameraEntityPacket {
    pub entity_id: EntityId,
}
