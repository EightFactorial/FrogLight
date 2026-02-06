#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub(crate) mod atomic;
pub mod generated;
pub mod item;
pub mod storage;
pub mod version;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        generated::{
            component as item_component,
            item::{self, VanillaItem},
        },
        item::{Item, ItemType},
        version::ItemVersion,
    };
}
