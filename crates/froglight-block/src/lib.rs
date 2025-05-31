#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", feature(const_type_id))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

extern crate alloc;

pub mod attribute;
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
