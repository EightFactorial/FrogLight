//! Implementations for reading and writing using [`rkyv`].

#[cfg(feature = "io")]
mod froglight;

pub mod reference;
pub use reference::{NamedNbtRef, NbtRefIterator, NbtStreamError, UnnamedNbtRef};
