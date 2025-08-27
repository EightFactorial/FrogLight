#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

extern crate alloc;

pub mod generated;
pub mod storage;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::storage::Registry;
}
