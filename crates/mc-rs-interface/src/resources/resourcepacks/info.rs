use bevy::prelude::*;
use compact_str::CompactString;

use crate::resourcepacks::ResourcePackAsset;

/// Information about a resourcepack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourcePackInfo {
    pub path: CompactString,
    pub loaded: bool,

    pub handle: Handle<ResourcePackAsset>,
}
