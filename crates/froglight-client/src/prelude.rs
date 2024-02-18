//! use `froglight-client::prelude::*;` to import common types and traits.

pub use froglight_core::{data::*, resources::*, systemsets::*};
pub use froglight_interface::{UiScaleEnable, UiScaleMaximum};
pub use froglight_resourcepack::{ResourcePack, ResourcePackManager};
pub use froglight_settings::SettingsSource;
pub use froglight_world::{
    blocks::Blocks,
    map::{ChunkEntity, WorldChunkMap, WorldMap, WorldType},
    world::Chunk,
};
