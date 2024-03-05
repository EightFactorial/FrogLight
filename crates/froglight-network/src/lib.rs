#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod connection;
pub use connection::{Clientbound, Connection, ConnectionError, Direction, Serverbound};

mod plugin;
pub use plugin::NetworkPlugin;
