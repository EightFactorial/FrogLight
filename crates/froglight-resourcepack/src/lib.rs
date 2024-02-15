#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod tracker;
pub use tracker::ResourcePackTracker;

mod loader;
pub use loader::ResourcePackLoader;

mod manager;
pub use manager::ResourcePackManager;

mod plugin;
pub use plugin::ResourcePackPlugin;

pub mod settings;

mod resourcepack;
pub use resourcepack::{meta, ResourcePack};
