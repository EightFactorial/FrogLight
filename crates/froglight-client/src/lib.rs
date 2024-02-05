#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod plugins;
pub mod prelude;

// Re-exports

pub use froglight_debug as debug;
#[cfg(feature = "default-loading")]
pub use froglight_loading as loading;
pub use froglight_world as world;
