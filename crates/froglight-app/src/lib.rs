#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod prelude;

// Re-export all sub-crates.
pub use froglight_assets as assets;
pub use froglight_client as client;
pub use froglight_core as core;
pub use froglight_network as network;
pub use froglight_protocol::protocol;
pub use froglight_registry as registry;
pub use froglight_render as render;
pub use froglight_settings as settings;
pub use froglight_utils as utils;
pub use froglight_world as world;

mod groups;
#[cfg(feature = "bevy_asset")]
pub use groups::app_plugins::AppPlugins;
pub use groups::{froglight_plugins::FrogLightPlugins, headless_plugins::HeadlessPlugins};
