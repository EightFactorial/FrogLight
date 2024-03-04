#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod components;
pub mod events;
pub mod resources;
pub mod systemsets;

// Re-export protocol data
pub use froglight_protocol::common;

mod plugin;
pub use plugin::CorePlugin;
