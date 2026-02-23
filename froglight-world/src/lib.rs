#![cfg_attr(feature = "nightly", feature(alloc_slice_into_array))]
#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "bevy")]
pub mod bevy;
pub mod borrowed;
#[cfg(feature = "alloc")]
pub mod chunk;
pub mod component;
pub mod section;

/// The length of a chunk.
const CHUNK_LENGTH: u8 = 16;
/// The width of a chunk.
const CHUNK_WIDTH: u8 = 16;

/// The length of a section.
const SECTION_LENGTH: u8 = CHUNK_LENGTH;
/// The width of a section.
const SECTION_WIDTH: u8 = CHUNK_WIDTH;
/// The height of a section.
const SECTION_HEIGHT: u8 = 16;
/// The volume of a section.
const SECTION_VOLUME: u16 = SECTION_LENGTH as u16 * SECTION_WIDTH as u16 * SECTION_HEIGHT as u16;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "bevy")]
    pub use crate::bevy::{relationship::ChunkOfInstance, world::WorldInstanceChunks};
    #[cfg(feature = "alloc")]
    pub use crate::chunk::NaiveChunk;
    #[cfg(all(feature = "froglight-biome", feature = "froglight-block", feature = "std"))]
    pub use crate::chunk::{Chunk, SharedChunk};
    pub use crate::component::{BlockPos, ChunkPos};
}
