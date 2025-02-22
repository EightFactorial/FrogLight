//! Implementations for reading and writing NBT data using [`rkyv`].

mod compound;
pub use compound::ArchivedNbtCompound;

mod mutf8;

mod named;
pub use named::{ArchivedNamedNbt, ArchivedUnnamedNbt};
