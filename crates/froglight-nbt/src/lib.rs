#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod convert;
pub mod io;
pub mod mutf8;
pub mod nbt;
pub mod snbt;

#[cfg(test)]
mod test;

pub mod prelude {
    //! Re-exports of common types and traits.

    pub use crate::{
        convert::ConvertNbt,
        mutf8::{Mutf8Str, Mutf8String},
        nbt::{NamedNbt, NbtCompound, NbtTag, UnnamedNbt},
        snbt::Snbt,
    };
}
