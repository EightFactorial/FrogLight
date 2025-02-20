#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod mutf8;
pub mod nbt;

pub mod prelude {
    //! Re-exports of common types and traits.

    pub use crate::{
        mutf8::{Mutf8Str, Mutf8String},
        nbt::{NamedNbt, NbtCompound, NbtListTag, NbtTag, UnnamedNbt},
    };
}
