#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod connection;
pub use connection::{Clientbound, Connection, ConnectionError, NetworkDirection, Serverbound};
// Re-export froglight-protocol
pub use froglight_protocol::{
    states,
    traits::{PacketEnum, State, Version},
    versions,
};
pub mod resolver;

mod plugin;
pub use plugin::NetworkPlugin;
