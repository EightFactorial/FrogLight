#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub use froglight_macros::Version;

mod entity;
pub use entity::{EntityId, EntityUuid};

mod identifier;
pub use identifier::Identifier;

pub mod version;
pub use version::Version;
