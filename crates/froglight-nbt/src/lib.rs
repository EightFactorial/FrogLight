#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", feature(iter_map_windows))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod convert;
pub mod io;
pub mod mutf8;
pub mod nbt;
pub mod snbt;

mod impls;
#[cfg(test)]
mod test;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use froglight_macros::FrogNbt;

    pub use crate::{
        convert::{FromCompound, FromTag, IntoCompound, IntoTag, NbtError},
        mutf8::{Mutf8Str, Mutf8String},
        nbt::{NamedNbt, NbtCompound, NbtListTag, NbtTag, UnnamedNbt},
        snbt::Snbt,
    };
}
