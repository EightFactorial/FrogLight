//! Re-exports of commonly used items from all `froglight` crates.
//!
//! ### Example:
//! ```
//! use froglight_app::prelude::*;
//! ```

pub mod plugins;

pub use froglight_client::interface::uiscale::{UiScaleEnable, UiScaleLimit};
pub use froglight_network::{
    common::*,
    connection::{Clientbound, Connection, ConnectionError, Serverbound},
    packet::*,
    resolver::{Resolver, ResolverError},
    states::{Configuration, Handshaking, Login, Play, Status},
    traits::{State, Version},
};
pub use froglight_registry::definitions::{BlockExt, BlockRegistry, BlockType};
pub use froglight_utils::{
    schedules::{FiveSeconds, OneSecond, OneTick, TenTicks, TwoTicks},
    tracking::{ChunkPositionMap, EntityChunkMap, EntityIdMap, EntityUuidMap},
};
pub use froglight_world::{Chunk, ChunkSection};
//  pub use froglight_registry::registries::*;
