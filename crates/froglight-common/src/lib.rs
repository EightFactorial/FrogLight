#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod entity;
pub use entity::{EntityId, EntityUuid};

mod identifier;
pub use identifier::Identifier;

pub mod version;
pub use froglight_macros::Version;
pub use version::Version;
