#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod default_camera;

mod plugin;
pub use plugin::InterfacePlugin;
