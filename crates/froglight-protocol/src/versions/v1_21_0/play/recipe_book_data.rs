use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

use crate::common::ResourceKey;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct RecipeBookDataPacket {
    pub recipe: ResourceKey,
}
