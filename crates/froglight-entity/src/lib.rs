#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod generated;
pub use generated::{component, entity};

mod plugin;
pub use plugin::EntityPlugin;

mod entity_size;
pub use entity_size::EntitySize;
