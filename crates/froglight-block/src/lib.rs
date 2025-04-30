#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(const_type_id)]
#![no_std]

extern crate alloc;

pub mod block;
pub mod generated;
pub mod resolve;
pub mod storage;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use froglight_macros::StaticBlock;

    pub use crate::{
        block::{Block, BlockConvert, BlockType, BlockTypeExt, StaticBlock, UntypedBlock},
        generated::{attribute, block},
        storage::{AppBlockStorage, BlockStorage, GlobalBlockId},
    };
}
