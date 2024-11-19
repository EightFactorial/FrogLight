//! Re-exports of commonly used items from all `froglight` crates.
//!
//! ### Example:
//! ```ignore,rust
//! use froglight::prelude::*;
//! ```

pub mod plugins;

// pub use froglight_block::{
//     definitions::{attributes, blocks},
//     BlockExt, BlockRegistry, BlockStateResolver, BlockType, VanillaResolver,
// };
pub use froglight_network::{
    common::*,
    connection::{Clientbound, Connection, ConnectionError, Serverbound},
    network::{ConnectionChannel, ConnectionTask, ConnectionTrait, PolledTask, StatusTask},
    packet::*,
    resolver::Resolver,
    states::{Configuration, Handshake, Login, Play, Status},
    traits::{State, Version},
};
// pub use froglight_registry::{definitions as registries, ConvertId, ConvertKey};
pub use froglight_utils::{
    schedules::{FiveSeconds, OneSecond, OneTick, TenTicks, TwoTicks},
    tracking::{ChunkPositionMap, EntityChunkMap, EntityIdMap, EntityUuidMap},
};
pub use froglight_world::{Chunk, ChunkSection};
