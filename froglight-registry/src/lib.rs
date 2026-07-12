#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod generated;
pub mod registry;
pub mod state;
pub mod storage;
pub mod version;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        registry::{NbtRef, NbtValueRef, TagRef, TagValueRef},
        state::GlobalRegistryId,
        version::RegistryVersion,
    };
}
