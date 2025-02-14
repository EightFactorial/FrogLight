#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(const_type_id)]

pub mod storage;
pub mod traits;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        storage::{AppRegistryStorage, RegistryStorage},
        traits::{RegistryType, RegistryValue},
    };
}
