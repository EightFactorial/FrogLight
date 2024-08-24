#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod prelude;

// Re-export all sub-crates.
#[cfg(feature = "client")]
pub use bevy_prng;
#[cfg(feature = "client")]
pub use bevy_rand;
#[cfg(feature = "client")]
pub use froglight_asset as asset;
#[cfg(feature = "client")]
pub use froglight_interface as interface;
#[cfg(feature = "client")]
pub use froglight_model as model;
pub use froglight_network as network;
pub use froglight_protocol::protocol;
pub use froglight_registry as registry;
#[cfg(feature = "client")]
pub use froglight_render as render;
pub use froglight_utils as utils;
pub use froglight_world as world;

mod groups;
pub use groups::*;
