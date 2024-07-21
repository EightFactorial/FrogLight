#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(trivial_bounds)]

mod entity;
pub use entity::{EntityId, EntityUuid};

mod resourcekey;
pub use resourcekey::{ResourceKey, ResourceKeyError};

mod version;
pub use version::Version;
