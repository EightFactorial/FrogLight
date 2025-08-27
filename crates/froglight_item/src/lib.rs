#![doc = include_str!("../README.md")]
#![allow(unexpected_cfgs, reason = "Allows using the custom `docsrs_dep` cfg")]
#![cfg_attr(any(docsrs, docsrs_dep), feature(doc_auto_cfg, rustdoc_internals))]
#![no_std]

extern crate alloc;

pub mod info;
pub mod item;
pub mod storage;

pub mod generated;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        generated as items,
        item::{GlobalItemId, Item},
        storage::Items,
    };
}
