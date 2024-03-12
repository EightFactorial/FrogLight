//! use `froglight-client::prelude::*;` to import common types and traits.

pub use froglight_assets::{
    AssetManager, AssetMcMeta, AssetSource, AssetTracker, AtlasManager, FallbackImage,
    ResourcePack, ResourcePackLoader, ResourcePackLoaderError, ResourcePackSettings,
    ResourcePackState,
};
pub use froglight_core::{common::*, components::*, events::*, resources::*, systemsets::*};
pub use froglight_interface::{
    materials::*,
    plugins::{
        inspector::InspectorEnable,
        uiscale::{UiScaleEnable, UiScaleMaximum},
    },
};
pub use froglight_network::{
    Clientbound, Connection, ConnectionError, NetworkDirection, Serverbound,
};
pub use froglight_protocol::{states::*, traits::*, versions};
pub use froglight_world::{
    biomes::{biome_list as biomes, BiomeRegistry, BiomeType, ReflectBiomeType},
    blocks::{block_list as blocks, BlockRegistry, BlockType, ReflectBlockType},
    maps::{ChunkEntity, WorldChunkMap, WorldMap, WorldType},
    world::{Chunk, Section},
};
