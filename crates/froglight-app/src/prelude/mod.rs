//! Re-exports of commonly used items from all `froglight` crates.
//!
//! ### Example:
//! ```
//! use froglight_app::prelude::*;
//! ```

pub mod plugins;

pub use froglight_core::{components::*, events::*, resources::*};
pub use froglight_network::{
    common::*,
    connection::{Clientbound, Connection, ConnectionError, Serverbound},
    packet::*,
    resolver::{Resolver, ResolverError},
    states::*,
    traits::*,
    versions::*,
};
pub use froglight_registry::{
    convert::{ConvertKey, ConvertKeyError, DefaultIdRegistry, MissingKeyError, SimpleIdRegistry},
    // registries::*,
    RegistryOverrideEvent,
};
