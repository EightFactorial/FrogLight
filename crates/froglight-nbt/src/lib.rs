#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(iter_map_windows)]
// #![cfg_attr(not(feature = "std"), no_std)]

// #[cfg(not(feature = "std"))]
// extern crate alloc;

pub mod convert;
pub mod io;
pub mod mutf8;
pub mod nbt;
pub mod snbt;

#[cfg(test)]
mod test;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use froglight_macros::FrogNbt;

    pub use crate::{
        convert::{FromCompound, FromTag, IntoCompound, IntoTag},
        mutf8::{Mutf8Str, Mutf8String},
        nbt::{NamedNbt, NbtCompound, NbtTag, UnnamedNbt},
        snbt::Snbt,
    };
}
