#![doc = include_str!("../README.md")]
#![allow(unexpected_cfgs, reason = "Allows using the custom `docsrs_dep` cfg")]
#![cfg_attr(any(docsrs, docsrs_dep), feature(doc_auto_cfg, rustdoc_internals))]
#![no_std]

extern crate alloc;

pub mod attribute;
pub mod block;
pub mod info;
pub mod storage;

pub mod generated;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        block::{Block, GlobalBlockState},
        storage::Blocks,
    };

    pub mod blocks {
        //! Re-exports of all block types and attributes.

        pub use crate::generated::{attribute as attributes, block::*};
    }
}
