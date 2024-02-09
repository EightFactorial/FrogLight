#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod plugin;
pub use plugin::{SettingsPlugin, SettingsSource};

mod settings;

mod source;
