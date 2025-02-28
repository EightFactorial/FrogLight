//! Implementations for reading and writing using [`rkyv`].

#[cfg(feature = "io")]
mod froglight;

pub mod slice;
pub use slice::{NamedNbtRef, NbtRefIterator, NbtStreamError, UnnamedNbtRef};
