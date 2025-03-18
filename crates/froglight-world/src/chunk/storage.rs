use derive_more::{From, Into};

use super::Section;
use crate::{position::SectionBlockPos, prelude::BlockPos};

/// A vertical slice of the world.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub enum ChunkStorage {
    /// A large chunk.
    ///
    /// Typically used for overworld chunks.
    Large(ArrayChunkStorage<24, -64>),
    /// A normal chunk.
    ///
    /// Typically used for nether and end chunks.
    Normal(ArrayChunkStorage<16, 0>),
    /// A chunk of some other size.
    ///
    /// May be used for custom worlds or other special cases.
    Other(VecChunkStorage),
}

impl ChunkStorage {
    /// The minimum height of this chunk in blocks.
    ///
    /// This is the bedrock floor for this chunk.
    #[must_use]
    pub const fn height_min(&self) -> isize {
        match self {
            ChunkStorage::Large(..) => -64,
            ChunkStorage::Normal(..) => 0,
            ChunkStorage::Other(storage) => storage.1,
        }
    }

    /// The maximum height of this chunk in blocks.
    ///
    /// This is the build limit for this chunk.
    #[must_use]
    pub const fn height_max(&self) -> usize {
        match self {
            ChunkStorage::Large(..) => 384 - 64,
            ChunkStorage::Normal(..) => 256,
            ChunkStorage::Other(storage) => {
                storage.0.len().saturating_mul(Section::HEIGHT).saturating_add_signed(storage.1)
            }
        }
    }

    /// The minimum [`BlockPos`] of this chunk.
    #[must_use]
    pub fn position_min(&self) -> BlockPos {
        if let Ok(y) = self.height_min().try_into() {
            BlockPos::new(0i32, y, 0i32)
        } else {
            unreachable!("Minimum height is out of bounds?")
        }
    }

    /// The maximum [`BlockPos`] of this chunk.
    #[must_use]
    #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    pub fn position_max(&self) -> BlockPos {
        if let Ok(y) = i32::try_from(self.height_max()) {
            BlockPos::new(Section::WIDTH as i32 - 1i32, y - 1i32, Section::DEPTH as i32 - 1i32)
        } else {
            unreachable!("Maximum height is out of bounds?")
        }
    }

    /// The total height of this chunk in blocks.
    ///
    /// This is the total height of the chunk
    /// from the bedrock floor to the build limit.
    #[must_use]
    pub const fn total_height(&self) -> usize {
        match self {
            ChunkStorage::Large(..) => 384,
            ChunkStorage::Normal(..) => 256,
            ChunkStorage::Other(storage) => storage.0.len().saturating_mul(Section::HEIGHT),
        }
    }

    /// The total volume of this chunk in blocks.
    #[must_use]
    pub const fn total_volume(&self) -> usize {
        self.total_height() * Section::WIDTH * Section::DEPTH
    }

    /// The the number of [`Section`]s in this chunk.
    #[must_use]
    pub const fn sections_len(&self) -> usize {
        match self {
            ChunkStorage::Large(..) => 24,
            ChunkStorage::Normal(..) => 16,
            ChunkStorage::Other(storage) => storage.0.len(),
        }
    }

    /// Get a reference to the [`Section`]s in this chunk.
    #[must_use]
    pub fn sections_ref(&self) -> &[Section] {
        match self {
            ChunkStorage::Large(storage) => storage.0.as_ref(),
            ChunkStorage::Normal(storage) => storage.0.as_ref(),
            ChunkStorage::Other(storage) => storage.0.as_ref(),
        }
    }

    /// Get a mutable reference to the [`Section`]s in this chunk.
    #[must_use]
    pub fn sections_mut(&mut self) -> &mut [Section] {
        match self {
            ChunkStorage::Large(storage) => storage.0.as_mut(),
            ChunkStorage::Normal(storage) => storage.0.as_mut(),
            ChunkStorage::Other(storage) => storage.0.as_mut(),
        }
    }

    /// Get a reference to the [`Section`] that holds the given [`BlockPos`].
    #[must_use]
    pub fn get(&self, pos: BlockPos) -> Option<&Section> {
        match self {
            ChunkStorage::Large(storage) => storage.get(pos),
            ChunkStorage::Normal(storage) => storage.get(pos),
            ChunkStorage::Other(storage) => storage.get(pos),
        }
    }

    /// Get a mutable reference to the [`Section`] that holds the given
    /// [`BlockPos`].
    #[must_use]
    pub fn get_mut(&mut self, pos: BlockPos) -> Option<&mut Section> {
        match self {
            ChunkStorage::Large(storage) => storage.get_mut(pos),
            ChunkStorage::Normal(storage) => storage.get_mut(pos),
            ChunkStorage::Other(storage) => storage.get_mut(pos),
        }
    }

    /// Create a new [`ChunkStorage`] from the given [`Section`]s and offset.
    #[must_use]
    pub fn from_sections(sections: Vec<Section>, offset: isize) -> Self {
        match (sections.len(), offset) {
            (24, -64) => ChunkStorage::Large(
                ArrayChunkStorage::try_from(sections)
                    .unwrap_or_else(|_| unreachable!("Length of `Vec` is equal to `SECTIONS`")),
            ),
            (16, 0) => ChunkStorage::Normal(
                ArrayChunkStorage::try_from(sections)
                    .unwrap_or_else(|_| unreachable!("Length of `Vec` is equal to `SECTIONS`")),
            ),
            _ => ChunkStorage::Other(VecChunkStorage::new(sections, offset)),
        }
    }
}

impl ChunkStorage {
    /// Get the block id at the given [`BlockPos`].
    ///
    /// Returns `None` if the position is out of bounds.
    #[must_use]
    pub fn get_raw_block(&self, pos: BlockPos) -> Option<u32> {
        self.get(pos).map(|section| section.get_block(SectionBlockPos::from_block(pos)))
    }

    /// Set the block id at the given [`BlockPos`].
    ///
    /// Returns the previous block id,
    /// or `None` if the position is out of bounds.
    ///
    ///  # Note
    /// `is_air` is a function that returns `true` if the block id is air.
    pub fn set_raw_block(
        &mut self,
        pos: BlockPos,
        block: u32,
        is_air: impl Fn(u32) -> bool,
    ) -> Option<u32> {
        // Set the block and return the previous block id.
        let section = self.get_mut(pos)?;
        let previous = section.set_block(SectionBlockPos::from_block(pos), block);

        // Update the block count using the provided function.
        match (is_air(previous), is_air(block)) {
            // Air -> Block
            (true, false) => {
                *section.blocks_mut() += 1;
            }
            // Block -> Air
            (false, true) => {
                *section.blocks_mut() -= 1;
            }
            _ => {}
        }

        Some(previous)
    }

    /// Get the biome id at the given [`BlockPos`].
    ///
    /// Returns `None` if the position is out of bounds.
    #[must_use]
    pub fn get_raw_biome(&self, pos: BlockPos) -> Option<u32> {
        self.get(pos).map(|section| section.get_biome(SectionBlockPos::from_block(pos)))
    }

    /// Set the biome id at the given [`BlockPos`].
    ///
    /// Returns the previous biome id,
    /// or `None` if the position is out of bounds.
    pub fn set_raw_biome(&mut self, pos: BlockPos, biome: u32) -> Option<u32> {
        self.get_mut(pos).map(|section| section.set_biome(SectionBlockPos::from_block(pos), biome))
    }
}

// -------------------------------------------------------------------------------------------------

/// A vertical slice of the world.
///
/// Has a constant, known number of sections and a known offset.
///
/// ---
///
/// Storing [`Sections`] in a fixed-size array has two main benefits:
///
/// 1. It guarantees that the number of sections is always correct.
/// 2. It prevents unnecessary bounds checks when accessing the array.
#[derive(Clone, From, Into)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect), reflect(opaque))]
pub struct ArrayChunkStorage<const SECTIONS: usize, const OFFSET: isize>(Box<[Section; SECTIONS]>);

impl<const SECTIONS: usize, const OFFSET: isize> ArrayChunkStorage<SECTIONS, OFFSET> {
    /// Create a new [`ArrayChunkStorage`] from the given [`Section`]s.
    #[must_use]
    pub fn new(sections: [Section; SECTIONS]) -> Self { Self(Box::new(sections)) }

    /// Create a new [`ArrayChunkStorage`] from the given [`Section`]s.
    #[must_use]
    pub const fn const_new(sections: Box<[Section; SECTIONS]>) -> Self { Self(sections) }

    /// Get a reference to the [`Section`] that holds the given [`BlockPos`].
    #[must_use]
    pub fn get(&self, pos: BlockPos) -> Option<&Section> {
        let pos: usize = (pos.y() as isize).saturating_sub(OFFSET).try_into().ok()?;
        self.0.get(pos / Section::HEIGHT)
    }

    /// Get a mutable reference to the [`Section`] that holds the given
    /// [`BlockPos`].
    #[must_use]
    pub fn get_mut(&mut self, pos: BlockPos) -> Option<&mut Section> {
        let pos: usize = (pos.y() as isize).saturating_sub(OFFSET).try_into().ok()?;
        self.0.get_mut(pos / Section::HEIGHT)
    }
}

impl<const SECTIONS: usize, const OFFSET: isize> Default for ArrayChunkStorage<SECTIONS, OFFSET>
where [Section; SECTIONS]: Default
{
    fn default() -> Self { Self::new(Default::default()) }
}

impl<const SECTIONS: usize, const OFFSET: isize> TryFrom<Vec<Section>>
    for ArrayChunkStorage<SECTIONS, OFFSET>
{
    type Error = Vec<Section>;

    fn try_from(value: Vec<Section>) -> Result<Self, Self::Error> {
        if value.len() == SECTIONS {
            if let Ok(sections) = value.into_boxed_slice().try_into() {
                Ok(Self::const_new(sections))
            } else {
                unreachable!("Length of `Vec` is equal to `SECTIONS`")
            }
        } else {
            Err(value)
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A vertical slice of the world.
///
/// Has a variable number of sections and a known offset.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct VecChunkStorage(Vec<Section>, isize);

impl VecChunkStorage {
    /// Create a new [`VecChunkStorage`] from the given [`Section`]s and
    /// offset.
    #[must_use]
    pub const fn new(sections: Vec<Section>, offset: isize) -> Self { Self(sections, offset) }

    /// Get a reference to the [`Section`] that holds the given [`BlockPos`].
    #[must_use]
    pub fn get(&self, pos: BlockPos) -> Option<&Section> {
        self.0.get::<usize>((pos.y() as isize).saturating_sub(self.1).try_into().ok()?)
    }

    /// Get a mutable reference to the [`Section`] that holds the given
    /// [`BlockPos`].
    #[must_use]
    pub fn get_mut(&mut self, pos: BlockPos) -> Option<&mut Section> {
        self.0.get_mut::<usize>((pos.y() as isize).saturating_sub(self.1).try_into().ok()?)
    }
}
