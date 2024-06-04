#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

// Re-export most of `froglight-protocol` for convenience.
pub use froglight_protocol::{common, packet, states, traits, versions};

pub mod connection;

#[cfg(feature = "bevy")]
pub mod network;
#[cfg(feature = "bevy")]
pub use network::NetworkPlugin;

#[cfg(feature = "bevy")]
mod plugin;
#[cfg(feature = "bevy")]
pub use plugin::NetworkPlugins;

#[cfg(feature = "resolver")]
pub mod resolver;
#[cfg(feature = "resolver")]
pub use resolver::ResolverPlugin;
