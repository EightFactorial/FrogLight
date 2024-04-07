//! Re-exports of commonly used items from all `froglight` crates.
//!
//! ### Example:
//! ```
//! use froglight_app::prelude::*;
//! ```

pub mod plugins;

pub use froglight_core::{components::*, events::*, resources::*};
pub use froglight_network::{
    connection::{Clientbound, Connection, ConnectionError, Serverbound},
    protocol::{
        common::*,
        protocol::{FrogRead, FrogVarRead, FrogVarWrite, FrogWrite},
        registries::*,
        states::*,
        traits::{State, Version},
    },
    resolver::{Resolver, ResolverError},
};
