#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(const_type_id)]

pub mod block;
pub mod generated;
pub mod storage;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        block::{Block, BlockConvert, BlockConverter, BlockType, BlockTypeExt, UntypedBlock},
        storage::{AppBlockStorage, BlockStorage, GlobalBlockId},
    };
}
