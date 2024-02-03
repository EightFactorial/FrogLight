#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod assets;
mod layout;
mod systemsets;

mod plugin;
pub use plugin::LoadingPlugin;
