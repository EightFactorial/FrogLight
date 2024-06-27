use std::sync::Arc;

use froglight_protocol::{
    common::ChunkBlockPosition,
    protocol::{FrogRead, ReadError},
};
use parking_lot::RwLock;

use crate::{ChunkBlockIter, ChunkSection};

/// A [`Chunk`] is a `16 x Y x 16 (X,Y,Z)`  section of blocks.
///
/// Because heights and offsets vary between `Worlds`, [`Chunks`](Self)
/// belonging to different `Worlds` can have a different amount of
/// [`ChunkSections`](ChunkSection).
///
/// Height Examples:
/// - `minecraft:overworld`: 384 (offset: -64, height: 320)
/// - `minecraft:the_nether`: 256 (offset: 0, height: 256)
/// - `minecraft:the_end`: 256 (offset: 0, height: 256)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
pub struct Chunk {
    /// The maximum height of the chunk.
    max_height: u32,
    /// The height offset of the chunk.
    height_offset: i32,

    /// The chunk's sections.
    pub sections: Arc<RwLock<Vec<ChunkSection>>>,
}

impl Chunk {
    /// The width of a [`Chunk`].
    pub const WIDTH: u32 = 16u32;
    /// The depth of a [`Chunk`].
    pub const DEPTH: u32 = 16u32;

    /// Returns the volume of the [`Chunk`].
    #[must_use]
    pub const fn volume(&self) -> u32 { Self::WIDTH * self.max_height * Self::DEPTH }

    /// Returns the maximum height of the [`Chunk`].
    ///
    /// This is relative to `Y = 0`, and does not include the height offset.
    ///
    /// Examples:
    /// - `minecraft:overworld`: 320
    /// - `minecraft:the_nether`: 256
    /// - `minecraft:the_end`: 256
    #[must_use]
    pub const fn max_height(&self) -> u32 { self.max_height }

    /// Returns the height offset of the [`Chunk`].
    ///
    /// This is the minimum height of the [`Chunk`], and can be negative.
    ///
    /// Examples:
    /// - `minecraft:overworld`: -64
    /// - `minecraft:the_nether`: 0
    /// - `minecraft:the_end`: 0
    #[must_use]
    pub const fn height_offset(&self) -> i32 { self.height_offset }

    /// Returns the absolute height of the [`Chunk`]
    /// in blocks from the bottom of the world.
    ///
    /// Examples:
    /// - `minecraft:overworld`: 384 (320 - -64)
    /// - `minecraft:the_nether`: 256 (256 - 0)
    /// - `minecraft:the_end`: 256 (256 - 0)
    #[must_use]
    pub const fn height(&self) -> u32 {
        Self::calc_internal_height(self.max_height, self.height_offset)
    }

    #[allow(clippy::cast_sign_loss)]
    const fn calc_internal_height(max_height: u32, height_offset: i32) -> u32 {
        max_height.wrapping_sub(height_offset as u32)
    }

    /// Returns the number of *expected* [`ChunkSections`](ChunkSection) in the
    /// [`Chunk`] based on the [`maximum height`](Self::max_height) and
    /// [`height offset`](Self::height_offset).
    ///
    /// ### Note
    /// This does not count the actual number of [`ChunkSection`]s in
    /// [`Chunk::sections`], for that use [`Chunk::sections`].
    #[must_use]
    pub const fn expected_sections(&self) -> u32 { self.height() / ChunkSection::HEIGHT }

    /// Returns the number of [`ChunkSections`](ChunkSection) in the [`Chunk`].
    ///
    /// # Note
    /// This acquires a [`read lock`](RwLock::read) on the [`Chunk`],
    /// and may block other threads.
    #[must_use]
    pub fn sections(&self) -> usize { self.sections.read().len() }

    /// Returns the index of the [`ChunkSection`] at the given height.
    #[must_use]
    pub const fn section_index(position: ChunkBlockPosition) -> usize {
        (position.y() / ChunkSection::HEIGHT) as usize
    }

    /// Creates a new empty [`Chunk`] with the given height.
    #[must_use]
    pub fn new_empty(max_height: u32, height_offset: i32) -> Self {
        let section_count =
            Self::calc_internal_height(max_height, height_offset) / ChunkSection::HEIGHT;

        Self {
            max_height,
            height_offset,
            sections: Arc::new(RwLock::new(vec![ChunkSection::default(); section_count as usize])),
        }
    }

    /// Returns an iterator over the blocks of the [`Chunk`].
    #[must_use]
    pub fn block_iter(&self) -> ChunkBlockIter<'_> { ChunkBlockIter::new(self) }
}

impl Chunk {
    /// Returns the `block id` at the given position.
    ///
    /// # Note
    /// This acquires a [`read lock`](RwLock::read) on the [`Chunk`],
    /// and may block other threads.
    #[must_use]
    pub fn get_block_id(&self, position: ChunkBlockPosition) -> Option<u32> {
        self.sections
            .read()
            .get(Self::section_index(position))
            .map(|s| s.get_block(position.into()))
    }

    /// Returns the `Block` at the given position.
    ///
    /// # Note
    /// This calls
    /// [`BlockStorage::get_block`](froglight_registry::definitions::BlockStorage)
    /// and can be very expensive compared to [`Chunk::get_block_id`].
    ///
    /// This acquires a [`read lock`](RwLock::read) on the [`Chunk`],
    /// and may block other threads.
    #[must_use]
    #[cfg(feature = "froglight-registry")]
    pub fn get_block<
        V: froglight_protocol::traits::Version,
        Res: froglight_registry::definitions::BlockStateResolver<V>,
    >(
        &self,
        position: ChunkBlockPosition,
        storage: &froglight_registry::definitions::BlockStorage<V>,
    ) -> Option<Res::Result> {
        self.get_block_id(position).map(|id| storage.get_block::<Res>(id))
    }

    /// Sets the `block id` at the given position.
    ///
    /// Returns the previous `block id` at the position.
    ///
    /// # Note
    /// This acquires a [`write lock`](RwLock::write) on the [`Chunk`],
    /// and will block other threads.
    #[allow(clippy::must_use_candidate)]
    pub fn set_block_id(&self, position: ChunkBlockPosition, block_id: u32) -> Option<u32> {
        self.sections
            .write()
            .get_mut(Self::section_index(position))
            .map(|s| s.set_block(position.into(), block_id))
    }

    /// Sets the `Block` at the given position.
    ///
    /// Returns the previous `Block` at the position.
    ///
    /// # Note
    /// This calls
    /// [`BlockStorage::get_block`](froglight_registry::definitions::BlockStorage)
    /// and can be very expensive compared to [`Chunk::set_block_id`].
    ///
    /// This acquires a [`write lock`](RwLock::write) on the [`Chunk`],
    /// and will block other threads.
    #[allow(clippy::must_use_candidate)]
    #[cfg(feature = "froglight-registry")]
    pub fn set_block<
        V: froglight_protocol::traits::Version,
        Res: froglight_registry::definitions::BlockStateResolver<V>,
    >(
        &self,
        position: ChunkBlockPosition,
        block: &impl froglight_registry::definitions::BlockExt<V>,
        storage: &froglight_registry::definitions::BlockStorage<V>,
    ) -> Option<Res::Result> {
        // Get the block id from the storage.
        let Some(block_id) = storage.get_block_id(block) else {
            #[cfg(feature = "bevy")]
            bevy_log::warn!("Block not found in storage: \"{}\"", block.to_key());
            return None;
        };

        // Set the block id and convert the old block id into a block.
        self.set_block_id(position, block_id).map(|old_id| storage.get_block::<Res>(old_id))
    }

    /// Sets the `Biome ID` at the given position.
    ///
    /// # Note
    /// This acquires a [`read lock`](RwLock::read) on the [`Chunk`],
    /// and may block other threads.
    #[must_use]
    pub fn get_biome_id(&self, position: ChunkBlockPosition) -> Option<u32> {
        self.sections
            .read()
            .get(Self::section_index(position))
            .map(|s| s.get_biome(position.into()))
    }

    /// Sets the `Biome ID` at the given position.
    ///
    /// Returns the previous `Biome ID` at the position.
    ///
    /// # Note
    /// This acquires a [`write lock`](RwLock::write) on the [`Chunk`],
    /// and will block other threads.
    #[allow(clippy::must_use_candidate)]
    pub fn set_biome_id(&self, position: ChunkBlockPosition, biome_id: u32) -> Option<u32> {
        self.sections
            .write()
            .get_mut(Self::section_index(position))
            .map(|s| s.set_biome(position.into(), biome_id))
    }
}

impl Chunk {
    /// Reads a [`Chunk`] from the given buffer.
    ///
    /// Requires knowing the maximum height and height offset of the chunk.
    ///
    /// # Errors
    /// If the chunk could not be read from the buffer.
    pub fn read_from(
        max_height: u32,
        height_offset: i32,
        buf: &mut std::io::Cursor<&[u8]>,
    ) -> Result<Self, ReadError> {
        let section_count =
            Self::calc_internal_height(max_height, height_offset) / ChunkSection::HEIGHT;

        let mut sections = Vec::with_capacity(section_count as usize);
        for _ in 0..section_count {
            sections.push(ChunkSection::fg_read(buf)?);
        }

        // TODO: Read heightmaps

        Ok(Self { max_height, height_offset, sections: Arc::new(RwLock::new(sections)) })
    }
}
