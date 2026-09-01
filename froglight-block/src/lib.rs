#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod attribute;
pub mod block;
pub mod generated;
pub mod state;
pub mod storage;
pub mod version;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        block::{Block, BlockAttributes, BlockType},
        generated::{
            attribute as block_attribute,
            block::{self, VanillaBlock},
        },
        state::{GlobalBlockId, GlobalStateId, RelativeStateId},
        version::BlockVersion,
    };
}
