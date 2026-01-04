#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub(crate) mod atomic;
pub mod block;
pub mod generated;
pub mod storage;
pub mod version;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        block::{Block, GlobalId, StateId},
        generated::{attribute as block_attr, block},
        version::BlockVersion,
    };
}
