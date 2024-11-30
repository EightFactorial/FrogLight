//! Re-exports of commonly used items from all `froglight` crates.
//!
//! ### Example:
//! ```ignore,rust
//! use froglight::prelude::*;
//! ```

pub mod plugins;

pub use froglight_block::{
    attribute,
    block::{self, Blocks},
    BlockState, BlockStateExt, BlockStorage, BlockStorageArc,
};
pub use froglight_entity::{component, entity};
pub use froglight_network::{
    common::*,
    connection::{Clientbound, Connection, ConnectionError, Serverbound},
    network::{
        BevyConnectionChannel as ConnectionChannel, ChannelRecvPacket, ConnectTrait,
        ConnectionErrorEvent, ConnectionTask, PolledTask, StatusTask,
    },
    packet::*,
    resolver::Resolver,
    states::{Configuration, Handshake, Login, Play, Status},
    traits::{State, Version},
};
pub use froglight_registry::{registry, RegistryId, RegistryKey};
pub use froglight_utils::{
    schedules::{FiveSeconds, OneSecond, OneTick, TenTicks, TwoTicks},
    tracking::{ChunkPositionMap, EntityChunkMap, EntityIdMap, EntityUuidMap},
};
pub use froglight_world::{Chunk, ChunkSection};

#[cfg(feature = "presets")]
pub use crate::{BasicPlugins, DefaultPlugins};
