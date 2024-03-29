#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod shaders;

mod plugin;
pub use plugin::RenderPlugin;
