#[cfg(not(feature = "hashbrown"))]
use std::collections::HashMap;

use froglight_components::resourcekey::ResourceKey;
use froglight_macros::FrogReadWrite;
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;

use crate::packet::ServerTagData;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct SynchronizeTagsPacket {
    pub tags: HashMap<ResourceKey, ServerTagData>,
}
