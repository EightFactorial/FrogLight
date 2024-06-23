use derive_more::{Deref, DerefMut, From, Into};
use froglight_components::resourcekey::ResourceKey;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct SelectAdvancementTabPacket {
    pub identifier: Option<ResourceKey>,
}
