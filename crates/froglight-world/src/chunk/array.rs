//! TODO

use derive_more::derive::{From, Into};
use glam::IVec3;

use super::VecChunk;
use crate::section::Section;

/// A chunk of blocks in a world.
///
/// Has a fixed amount of [`Sections`].
#[derive(Clone, From, Into)]
pub struct ArrayChunk<const SECTIONS: usize, const OFFSET: i32>([Section; SECTIONS]);

impl<const SECTIONS: usize, const OFFSET: i32> Default for ArrayChunk<SECTIONS, OFFSET> {
    fn default() -> Self { Self(std::array::from_fn(|_| Section::default())) }
}

impl<const SECTIONS: usize, const OFFSET: i32> ArrayChunk<SECTIONS, OFFSET> {
    /// The total volume of the [`ArrayChunk`] in blocks.
    pub const VOLUME: usize = Section::VOLUME * SECTIONS;
    /// The height of the [`ArrayChunk`] in blocks.
    pub const HEIGHT: usize = Section::HEIGHT * SECTIONS;
    /// The width of the [`ArrayChunk`] in blocks.
    pub const WIDTH: usize = Section::WIDTH;
    /// The depth of the [`ArrayChunk`] in blocks.
    pub const DEPTH: usize = Section::DEPTH;

    /// The total volume of the [`ArrayChunk`] in blocks.
    #[inline]
    #[must_use]
    pub const fn volume(&self) -> usize { Self::VOLUME }
    /// The height of the [`ArrayChunk`] in blocks.
    #[inline]
    #[must_use]
    pub const fn height(&self) -> usize { Self::HEIGHT }
    /// The width of the [`ArrayChunk`] in blocks.
    #[inline]
    #[must_use]
    pub const fn width(&self) -> usize { Self::WIDTH }
    /// The depth of the [`ArrayChunk`] in blocks.
    #[inline]
    #[must_use]
    pub const fn depth(&self) -> usize { Self::DEPTH }

    /// Get a reference to the [`Section`]s in the [`ArrayChunk`].
    #[inline]
    #[must_use]
    pub const fn sections(&self) -> &[Section; SECTIONS] { &self.0 }

    /// Get a mutable reference to the [`Section`]s in the [`ArrayChunk`].
    #[inline]
    #[must_use]
    pub const fn sections_mut(&mut self) -> &mut [Section; SECTIONS] { &mut self.0 }

    /// Get a reference to a [`Section`] based on the `y` coordinate.
    #[inline]
    #[must_use]
    pub fn get_section(&self, y_coord: i32) -> Option<&Section> {
        self.get_nonoffset_section(y_coord.checked_add(OFFSET)?)
    }
    /// Get a reference to a [`Section`] based on the `y` coordinate.
    ///
    /// # Note
    /// This does not take into account the chunk offset.
    #[inline]
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    fn get_nonoffset_section(&self, y_coord: i32) -> Option<&Section> {
        self.0.get(y_coord as usize / Self::HEIGHT)
    }

    /// Get a mutable reference to a [`Section`] based on the `y` coordinate.
    #[inline]
    #[must_use]
    pub fn get_section_mut(&mut self, y_coord: i32) -> Option<&mut Section> {
        self.get_nonoffset_section_mut(y_coord.checked_add(OFFSET)?)
    }
    /// Get a mutable reference to a [`Section`] based on the `y` coordinate.
    ///
    /// # Note
    /// This does not take into account the chunk offset.
    #[inline]
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    fn get_nonoffset_section_mut(&mut self, y_coord: i32) -> Option<&mut Section> {
        self.0.get_mut(y_coord as usize / Section::HEIGHT)
    }

    /// Get a block from the [`ArrayChunk`].
    ///
    /// Returns `None` if the `y` coordinate is out of bounds.
    #[must_use]
    pub fn get_block_raw(&self, mut position: IVec3) -> Option<u32> {
        position.y = position.y.checked_add(OFFSET)?;
        self.get_nonoffset_section(position.y).map(|s| s.get_block(position))
    }
    /// Set a block in the [`ArrayChunk`].
    ///
    /// Returns `None` if the y coordinate is out of bounds.
    #[must_use]
    pub fn set_block_raw(&mut self, mut position: IVec3, block: u32) -> Option<u32> {
        position.y = position.y.checked_add(OFFSET)?;
        self.get_nonoffset_section_mut(position.y).map(|s| s.set_block(position, block))
    }

    /// Get a block from the [`ArrayChunk`] with data from the [`BlockStorage`].
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if no matching block is found.
    #[must_use]
    #[cfg(feature = "block")]
    pub fn get_block_untyped<V: froglight_common::Version>(
        &self,
        position: IVec3,
        storage: &froglight_block::storage::BlockStorage<V>,
    ) -> Option<froglight_block::block::UntypedBlock<V>> {
        self.get_block_raw(position).and_then(|id| {
            storage.get_untyped(froglight_block::storage::GlobalBlockId::new_unchecked(id))
        })
    }
    /// Set a block in the [`ArrayChunk`] using data from the [`BlockStorage`].
    ///
    /// Returns the previous block if it was set, or
    /// `None` if the position is out of bounds or no matching block is found.
    #[cfg(feature = "block")]
    pub fn set_block_untyped<V: froglight_common::Version>(
        &mut self,
        position: IVec3,
        block: impl Into<froglight_block::block::UntypedBlock<V>>,
        storage: &froglight_block::storage::BlockStorage<V>,
    ) -> Option<froglight_block::block::UntypedBlock<V>> {
        self.set_block_raw(position, *storage.get_global(block)?).and_then(|id| {
            storage.get_untyped(froglight_block::storage::GlobalBlockId::new_unchecked(id))
        })
    }

    /// Convert an [`ArrayChunk`] into a [`VecChunk`].
    #[inline]
    #[must_use]
    pub fn into_vec(self) -> VecChunk { VecChunk::new_from(self.0, OFFSET) }
}

impl<const SECTIONS: usize, const OFFSET: i32> From<ArrayChunk<SECTIONS, OFFSET>> for VecChunk {
    fn from(chunk: ArrayChunk<SECTIONS, OFFSET>) -> Self { chunk.into_vec() }
}

#[test]
fn dimensions() {
    // This is the same size as the Nether and End.
    let normal = ArrayChunk::<16, 0>::default();
    assert_eq!(normal.height(), 256, "Normal ArrayChunk height is incorrect!");
    assert_eq!(normal.volume(), 65536, "Normal ArrayChunk volume is incorrect!");
    let normal = normal.into_vec();
    assert_eq!(normal.height(), 256, "Normal VecChunk height is incorrect!");
    assert_eq!(normal.volume(), 65536, "Normal VecChunk volume is incorrect!");

    // This is the same size as the Overworld.
    let large = ArrayChunk::<24, -64>::default();
    assert_eq!(large.height(), 384, "Large ArrayChunk height is incorrect!");
    assert_eq!(large.volume(), 98304, "Large ArrayChunk volume is incorrect!");
    let large = large.into_vec();
    assert_eq!(large.height(), 384, "Large VecChunk height is incorrect!");
    assert_eq!(large.volume(), 98304, "Large VecChunk volume is incorrect!");
}
