#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod prelude;

// Re-export all sub-crates.
pub use froglight_core as core;
pub use froglight_network as network;

mod groups;
pub use groups::{app_plugins::AppPlugins, headless_plugins::HeadlessPlugins};
