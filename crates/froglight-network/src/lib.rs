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

mod plugin;
pub use plugin::NetworkPlugin;

pub mod resolver;
pub use resolver::ResolverPlugin;
