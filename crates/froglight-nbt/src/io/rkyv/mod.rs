//! Implementations for reading and writing NBT data using [`rkyv`].
#![expect(dead_code, unreachable_pub, unused_variables)]

mod compound;
pub use compound::ArchivedNbtCompound;

mod mutf8;

mod named;
pub use named::{ArchivedNamedNbt, ArchivedUnnamedNbt};
