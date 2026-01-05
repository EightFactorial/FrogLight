#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod borrowed;
#[cfg(feature = "alloc")]
pub mod chunk;
pub mod component;

/// The length of a chunk.
pub const CHUNK_LENGTH: u8 = 16;
/// The width of a chunk.
pub const CHUNK_WIDTH: u8 = 16;

/// The length of a section.
pub const SECTION_LENGTH: u8 = CHUNK_LENGTH;
/// The width of a section.
pub const SECTION_WIDTH: u8 = CHUNK_WIDTH;
/// The height of a section.
pub const SECTION_HEIGHT: u8 = 16;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "alloc")]
    pub use crate::chunk::Chunk;
    pub use crate::component::{BlockPos, ChunkPos};
}
