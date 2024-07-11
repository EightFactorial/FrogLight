#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(build_hasher_default_const_new)]

pub mod definitions;

#[cfg(feature = "bevy")]
mod registry;
#[cfg(feature = "bevy")]
pub use registry::BlockRegistry;

#[cfg(feature = "bevy")]
mod plugin;
#[cfg(feature = "bevy")]
pub use plugin::BlockPlugin;

#[cfg(feature = "bevy")]
mod resolver;
#[cfg(feature = "bevy")]
pub use resolver::{BlockStateResolver, VanillaResolver};

#[cfg(feature = "bevy")]
mod storage;
#[cfg(feature = "bevy")]
pub use storage::BlockStorage;

mod traits;
pub use traits::{BlockExt, BlockType};
