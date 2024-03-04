//! use `froglight-client::prelude::*;` to import common types and traits.

pub use froglight_assets::{
    AssetManager, AssetSource, ResourcePack, ResourcePackLoader, ResourcePackLoaderError,
};
pub use froglight_core::{common::*, events::*, resources::*, systemsets::*};
pub use froglight_interface::plugins::uiscale::{UiScaleEnable, UiScaleMaximum};
pub use froglight_world::{
    blocks::Blocks,
    maps::{ChunkEntity, WorldChunkMap, WorldMap, WorldType},
    world::Chunk,
};
