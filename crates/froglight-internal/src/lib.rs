#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod prelude;

// Re-export all sub-crates.
pub use froglight_network as network;
pub use froglight_protocol::protocol;
pub use froglight_registry as registry;
pub use froglight_utils as utils;
pub use froglight_world as world;

#[cfg(feature = "presets")]
mod groups;
#[cfg(feature = "presets")]
pub use groups::*;
