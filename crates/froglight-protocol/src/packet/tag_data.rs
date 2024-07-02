#[cfg(not(feature = "hashbrown"))]
use std::collections::HashMap;

use derive_more::{Deref, DerefMut, From, Into};
use froglight_components::resourcekey::ResourceKey;
use froglight_macros::FrogReadWrite;
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;

/// Data for a tag sent by the server
#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, From, Into, FrogReadWrite)]
pub struct ServerTagData {
    /// The tag data
    #[frog(var)]
    pub data: HashMap<ResourceKey, Vec<u32>>,
}
