//! Implementations for reading and writing using [`rkyv`].

#[cfg(feature = "io")]
mod froglight;
#[cfg(feature = "rkyv")]
pub mod rkyv;
