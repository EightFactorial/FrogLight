//! Re-exports of commonly used items from all `froglight` crates.
//!
//! ### Example:
//! ```
//! use froglight_app::prelude::*;
//! ```

pub mod plugins;

pub use froglight_core as core;
pub use froglight_core::{components::*, events::*, resources::*, systemsets::*};
pub use froglight_network as network;
pub use froglight_network::resolver::Resolver;
