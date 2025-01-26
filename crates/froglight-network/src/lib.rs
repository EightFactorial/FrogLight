#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod connection;
pub mod version;

#[cfg(feature = "resolver")]
pub mod resolver;
