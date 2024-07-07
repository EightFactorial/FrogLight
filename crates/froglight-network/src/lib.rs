//! # Froglight Network
//!
//! A basic networking implementation built on [Froglight
//! Protocol](../froglight-protocol/).
//!
//! Supports both `Client -> Server` and `Server -> Client` connections.
//!
//! # Note
//! The [`NetworkPlugin`] only supports creating connections to servers.

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
