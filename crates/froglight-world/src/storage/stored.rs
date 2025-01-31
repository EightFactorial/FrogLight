use derive_more::derive::From;
use glam::IVec3;

use crate::{
    chunk::{ArrayChunk, VecChunk},
    section::Section,
};

/// A chunk that can be stored in a [`ChunkStorage`](super::ChunkStorage).
#[derive(Clone, From)]
pub enum StoredChunk<const SECTIONS: usize, const OFFSET: i32> {
    /// A chunk backed by an array.
    Array(ArrayChunk<SECTIONS, OFFSET>),
    /// A chunk backed by a vector.
    Vec(VecChunk),
}

impl<const SECTIONS: usize, const OFFSET: i32> Default for StoredChunk<SECTIONS, OFFSET> {
    fn default() -> Self { Self::Array(ArrayChunk::default()) }
}

impl<const SECTIONS: usize, const OFFSET: i32> StoredChunk<SECTIONS, OFFSET> {
    /// Get a reference to the [`Section`]s in the [`StoredChunk`].
    #[must_use]
    pub fn sections(&self) -> &[Section] {
        match self {
            Self::Array(chunk) => chunk.sections(),
            Self::Vec(chunk) => chunk.sections().as_slice(),
        }
    }

    /// Get a mutable reference to the [`Section`]s in the [`StoredChunk`].
    #[must_use]
    pub fn sections_mut(&mut self) -> &mut [Section] {
        match self {
            Self::Array(chunk) => chunk.sections_mut(),
            Self::Vec(chunk) => chunk.sections_mut().as_mut(),
        }
    }

    /// Get a reference to a [`Section`] based on the `y` coordinate.
    #[must_use]
    pub fn get_section(&self, y_coord: i32) -> Option<&Section> {
        match self {
            Self::Array(chunk) => chunk.get_section(y_coord),
            Self::Vec(chunk) => chunk.get_section(y_coord),
        }
    }

    /// Get a mutable reference to a [`Section`] based on the `y` coordinate.
    #[must_use]
    pub fn get_section_mut(&mut self, y_coord: i32) -> Option<&mut Section> {
        match self {
            Self::Array(chunk) => chunk.get_section_mut(y_coord),
            Self::Vec(chunk) => chunk.get_section_mut(y_coord),
        }
    }

    /// Get a block from the [`StoredChunk`].
    ///
    /// Returns `None` if the `y` coordinate is out of bounds.
    #[must_use]
    pub fn get_block_raw(&self, position: IVec3) -> Option<u32> {
        match self {
            Self::Array(chunk) => chunk.get_block_raw(position),
            Self::Vec(chunk) => chunk.get_block_raw(position),
        }
    }

    /// Set a block in the [`StoredChunk`].
    ///
    /// Returns `None` if the y coordinate is out of bounds.
    #[must_use]
    pub fn set_block_raw(&mut self, position: IVec3, block: u32) -> Option<u32> {
        match self {
            Self::Array(chunk) => chunk.set_block_raw(position, block),
            Self::Vec(chunk) => chunk.set_block_raw(position, block),
        }
    }
}
