//! TODO

use alloc::vec::Vec;

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;

#[cfg(feature = "bevy")]
use crate::reflect::ReflectChunk;
use crate::{
    position::RelativePosition, prelude::BlockPosition, section::Section, storage::SectionStorage,
};

/// A vertical column of [`Section`]s.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect), reflect(Clone, Component, Chunk, opaque))]
pub struct Chunk {
    storage: SectionStorage,
}

impl Chunk {
    /// Create a new, empty [`Chunk`] with the normal world height (256 blocks).
    #[must_use]
    pub fn new_normal() -> Self { Self { storage: SectionStorage::empty_normal() } }

    /// Create a new, empty [`Chunk`] with the large world height (384 blocks).
    #[must_use]
    pub fn new_large() -> Self { Self { storage: SectionStorage::empty_large() } }

    /// Create a new [`Chunk`] from the given [`Section`]s and vertical offset.
    #[must_use]
    pub fn from_sections(sections: Vec<Section>, offset: isize) -> Self {
        Self { storage: SectionStorage::new(sections, offset) }
    }

    /// Returns the number of [`Section`]s in this [`Chunk`].
    #[must_use]
    pub const fn len(&self) -> usize { self.storage.len() }

    /// Returns `true` if this chunk contains no sections or only air.
    #[must_use]
    pub fn is_empty(&self) -> bool { self.storage.is_empty() }

    /// Return the minimum height (inclusive) of this [`Chunk`].
    #[must_use]
    pub const fn min_height(&self) -> isize { self.storage.min_height() }

    /// Return the maximum height (exclusive) of this [`Chunk`].
    #[must_use]
    pub const fn max_height(&self) -> isize { self.storage.max_height() }

    /// Returns the total volume of this [`Chunk`] in blocks.
    #[must_use]
    pub const fn volume(&self) -> usize { self.storage.volume() }

    /// Returns a reference to the internal [`SectionStorage`].
    #[inline]
    #[must_use]
    pub const fn storage(&self) -> &SectionStorage { &self.storage }

    /// Returns a mutable reference to the internal [`SectionStorage`].
    #[inline]
    #[must_use]
    pub const fn storage_mut(&mut self) -> &mut SectionStorage { &mut self.storage }

    /// Get a block ID from this chunk.
    ///
    /// Returns `None` if the section containing the block is not present.
    #[must_use]
    #[allow(
        clippy::needless_else,
        clippy::manual_map,
        reason = "Only needless when `tracing` is disabled."
    )]
    pub fn get_block_raw(&self, position: BlockPosition) -> Option<u32> {
        let position = RelativePosition::from_block(position, self.storage.min_height());
        let index = position.y() as usize / Section::SIDE_LENGTH;

        if let Some(section) = self.storage.get(index) {
            Some(section.get_block(position))
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                "Attempted to get block in Section `{index}`, but only `{}` exist!",
                self.storage.len()
            );
            None
        }
    }

    /// Set a block ID in this chunk, returning the previous block ID.
    ///
    /// Returns `None` if the section containing the block is not present.
    #[allow(
        clippy::needless_else,
        clippy::manual_map,
        reason = "Only needless when `tracing` is disabled."
    )]
    pub fn set_block_raw(
        &mut self,
        block_id: u32,
        position: BlockPosition,
        is_air: impl Fn(u32) -> bool,
    ) -> Option<u32> {
        let position = RelativePosition::from_block(position, self.storage.min_height());
        let index = position.y() as usize / Section::SIDE_LENGTH;

        if let Some(section) = self.storage.get_mut(index) {
            Some(section.set_block(block_id, position, is_air))
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                "Attempted to set block in Section `{index}`, but only `{}` exist!",
                self.storage.len()
            );
            None
        }
    }

    /// Get a biome id from this chunk.
    ///
    /// Returns `None` if the section containing the biome is not present.
    #[must_use]
    #[allow(
        clippy::needless_else,
        clippy::manual_map,
        reason = "Only needless when `tracing` is disabled."
    )]
    pub fn get_biome(&self, position: RelativePosition) -> Option<u32> {
        let index = position.y() as usize / Section::SIDE_LENGTH;
        if let Some(section) = self.storage.get(index) {
            Some(section.get_biome(position))
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                "Attempted to get biome in Section `{index}`, but only `{}` exist!",
                self.storage.len()
            );
            None
        }
    }

    /// Set a biome id in this chunk.
    ///
    /// Returns the previous biome id,
    /// or `None` if the section containing the biome is not present.
    #[allow(
        clippy::needless_else,
        clippy::manual_map,
        reason = "Only needless when `tracing` is disabled."
    )]
    pub fn set_biome(&mut self, biome_id: u32, position: RelativePosition) -> Option<u32> {
        let index = position.y() as usize / Section::SIDE_LENGTH;
        if let Some(section) = self.storage.get_mut(index) {
            Some(section.set_biome(biome_id, position))
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                "Attempted to set biome in Section `{index}`, but only `{}` exist!",
                self.storage.len()
            );
            None
        }
    }
}
