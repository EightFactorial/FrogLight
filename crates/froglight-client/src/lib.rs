#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod plugins;
pub mod prelude;

// Re-exports

pub use froglight_assets as assets;
pub use froglight_core as core;
pub use froglight_interface as interface;
pub use froglight_network as network;
pub use froglight_physics as physics;
pub use froglight_protocol as protocol;
pub use froglight_render as render;
pub use froglight_world as world;
