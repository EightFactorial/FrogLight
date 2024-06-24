//! Re-exports of commonly used items from all `froglight` crates.
//!
//! ### Example:
//! ```ignore,rust
//! use froglight::prelude::*;
//! ```

pub mod plugins;

pub use froglight_network::{
    common::*,
    connection::{Clientbound, Connection, ConnectionError, Serverbound},
    network::{ConnectionChannel, ConnectionTask, ConnectionTrait, PolledTask, StatusTask},
    packet::*,
    resolver::Resolver,
    states::{Configuration, Handshake, Login, Play, Status},
    traits::{State, Version},
};
pub use froglight_registry::definitions::{BlockExt, BlockRegistry, BlockType};
pub use froglight_utils::{
    schedules::{FiveSeconds, OneSecond, OneTick, TenTicks, TwoTicks},
    tracking::{ChunkPositionMap, EntityChunkMap, EntityIdMap, EntityUuidMap},
};
pub use froglight_world::{Chunk, ChunkSection};
//  pub use froglight_registry::registries::*;
