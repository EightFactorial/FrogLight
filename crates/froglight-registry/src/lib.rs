#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(const_type_name)]
#![feature(const_type_id)]

mod error;
pub use error::InvalidKeyError;

pub mod definitions;

#[cfg(feature = "bevy")]
mod plugin;
#[cfg(feature = "bevy")]
pub use plugin::RegistryPlugin;

mod traits;
pub use traits::{ConvertId, ConvertKey};
