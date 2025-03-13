//! Implementations for reading and writing NBT data.

#[cfg(feature = "io")]
mod froglight;

pub mod slice;
pub use slice::{NamedNbtRef, NbtRefIterator, NbtStreamError, UnnamedNbtRef};
