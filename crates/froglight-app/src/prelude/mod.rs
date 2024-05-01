//! Re-exports of commonly used items from all `froglight` crates.
//!
//! ### Example:
//! ```
//! use froglight_app::prelude::*;
//! ```

pub mod plugins;

pub use froglight_client::interface::uiscale::*;
pub use froglight_network::{
    common::*,
    connection::{Clientbound, Connection, ConnectionError, Serverbound},
    packet::*,
    resolver::{Resolver, ResolverError},
    states::*,
    traits::*,
    versions::*,
};
pub use froglight_registry::definitions::*;
pub use froglight_settings::ConfigFolder;
pub use froglight_utils::{schedules::*, tracking::*};
pub use froglight_world::{Chunk, ChunkBlockIter, ChunkSection, SectionBlockIter};
//  pub use froglight_registry::registries::*;
