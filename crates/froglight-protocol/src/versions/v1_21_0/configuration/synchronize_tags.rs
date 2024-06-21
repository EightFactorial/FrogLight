//! @generated by `froglight-generator` #ecfea09

#[cfg(not(feature = "hashbrown"))]
use std::collections::HashMap;

use compact_str::CompactString;
use froglight_macros::FrogReadWrite;
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;

use crate::common::ResourceKey;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
pub struct SynchronizeTagsPacket {
    pub field_0: ResourceKey,
    pub field_1: CompactString,
    #[frog(var)]
    pub field_2: u32,
    #[frog(var)]
    pub field_3: u32,
    pub field_4: HashMap<(), ()>,
    pub field_5: HashMap<(), ()>,
}
