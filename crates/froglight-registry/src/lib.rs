#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", feature(const_type_id))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

extern crate alloc;

pub mod generated;
pub mod storage;
pub mod traits;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::storage::{AppRegistryStorage, RegistryStorage};
}
