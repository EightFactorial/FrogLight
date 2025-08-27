//! TODO

use alloc::vec::Vec;
use core::{
    any::TypeId,
    ops::{Deref, DerefMut},
};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
use froglight_block::{prelude::*, storage::StaticBlockMap};
use froglight_common::version::Version;

#[cfg(feature = "bevy")]
use crate::reflect::ReflectChunk;
use crate::{chunk::Chunk, prelude::BlockPosition, section::Section};

/// A thread-safe, shared reference to a [`VersionedChunk`].
#[cfg(feature = "async")]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Clone, Component, Chunk, opaque, from_reflect = false))]
pub struct SharedChunk(alloc::sync::Arc<async_lock::RwLock<VersionedChunk>>);

#[cfg(feature = "async")]
impl SharedChunk {
    /// Create a new [`SharedChunk`] from the given [`VersionedChunk`].
    #[inline]
    #[must_use]
    pub const fn new_arc(chunk: alloc::sync::Arc<async_lock::RwLock<VersionedChunk>>) -> Self {
        Self(chunk)
    }

    /// Create a new [`SharedChunk`] from the given [`VersionedChunk`].
    #[must_use]
    pub fn new(chunk: VersionedChunk) -> Self {
        Self::new_arc(alloc::sync::Arc::new(async_lock::RwLock::new(chunk)))
    }
}

#[cfg(feature = "async")]
impl Deref for SharedChunk {
    type Target = alloc::sync::Arc<async_lock::RwLock<VersionedChunk>>;

    fn deref(&self) -> &Self::Target { &self.0 }
}
#[cfg(feature = "async")]
impl DerefMut for SharedChunk {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// -------------------------------------------------------------------------------------------------

/// A vertical column of [`Section`]s.
///
/// Stores the [`Version`] of blocks used in this [`Chunk`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Clone, Component, Chunk, from_reflect = false))]
pub struct VersionedChunk {
    #[cfg_attr(feature = "bevy", reflect(ignore))]
    blocks: &'static StaticBlockMap,
    version: TypeId,
    chunk: Chunk,
}

impl VersionedChunk {
    /// Create a new, empty [`VersionedChunk`]
    /// with the normal world height (256 blocks).
    #[must_use]
    pub fn new_normal<V: Blocks>() -> Self {
        Self { blocks: V::blocks(), version: TypeId::of::<V>(), chunk: Chunk::new_normal() }
    }

    /// Create a new, empty [`VersionedChunk`]
    /// with the large world height (384 blocks).
    #[must_use]
    pub fn new_large<V: Blocks>() -> Self {
        Self { blocks: V::blocks(), version: TypeId::of::<V>(), chunk: Chunk::new_large() }
    }

    /// Create a new [`VersionedChunk`] from
    /// the given [`Section`]s and vertical offset.
    #[must_use]
    pub fn from_sections<V: Blocks>(sections: Vec<Section>, offset: isize) -> Self {
        Self {
            blocks: V::blocks(),
            version: TypeId::of::<V>(),
            chunk: Chunk::from_sections(sections, offset),
        }
    }

    /// Returns `true` if this [`VersionedChunk`]
    /// is using the given [`Version`].
    #[must_use]
    pub fn is_version<V: Version>(&self) -> bool { self.version == TypeId::of::<V>() }

    /// Get the [`StaticBlockMap`] used by this [`VersionedChunk`].
    #[must_use]
    pub const fn block_map(&self) -> &'static StaticBlockMap { self.blocks }

    /// Get a [`Block`] from this [`VersionedChunk`].
    ///
    /// Returns `None` if the id does not resolve to a valid block,
    /// or if the section containing the block is not present.
    #[must_use]
    pub fn get_block(&self, position: BlockPosition) -> Option<Block> {
        self.get_block_raw(position).and_then(|block_id| {
            self.blocks.read_blocking().get_block(GlobalBlockState::from(block_id))
        })
    }

    /// Set a [`Block`] in this [`VersionedChunk`].
    ///
    /// Returns the previous block if the id resolves to a valid block,
    /// or `None` if the section containing the block is not present.
    pub fn set_block(&mut self, block: GlobalBlockState, position: BlockPosition) -> Option<Block> {
        let blocks = self.blocks.read_blocking();
        let previous = self.set_block_raw(*block, position, |block_id: u32| -> bool {
            match blocks.get_info(GlobalBlockState::from(block_id)) {
                Some(info) => info.settings().is_air(),
                None => false,
            }
        });

        previous.and_then(|block_id| blocks.get_block(GlobalBlockState::from(block_id)))
    }
}

impl Deref for VersionedChunk {
    type Target = Chunk;

    fn deref(&self) -> &Self::Target { &self.chunk }
}
impl DerefMut for VersionedChunk {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.chunk }
}
