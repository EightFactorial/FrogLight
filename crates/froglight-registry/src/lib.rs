#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(build_hasher_default_const_new)]
#![feature(const_type_name)]
#![feature(const_type_id)]

mod generated;
pub use generated::registry;

mod traits;
pub use traits::{RegistryId, RegistryKey};

#[cfg(feature = "reflect")]
mod plugin;
#[cfg(feature = "reflect")]
pub use plugin::RegistryPlugin;
