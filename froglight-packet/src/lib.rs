#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod common;
pub mod core;
pub mod generated;
pub mod version;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[allow(unused_imports, unreachable_pub, reason = "Prelude")]
    pub use crate::generated::*;
}
