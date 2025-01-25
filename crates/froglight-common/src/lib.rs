#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod identifier;
pub use identifier::Identifier;

pub mod version;
pub use version::Version;
