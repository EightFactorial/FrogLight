use derive_more::derive::{From, Into};
use glam::IVec3;

use super::ArrayChunk;
use crate::section::Section;

/// A chunk of blocks in a world.
///
/// Has a dynamic amount of [`Sections`].
#[derive(Clone, From, Into)]
pub struct VecChunk(Vec<Section>, i32);

impl VecChunk {
    /// The depth of the [`VecChunk`] in blocks.
    pub const DEPTH: usize = Section::DEPTH;
    /// The width of the [`VecChunk`] in blocks.
    pub const WIDTH: usize = Section::WIDTH;

    /// The total volume of the [`VecChunk`] in blocks.
    #[inline]
    #[must_use]
    pub fn volume(&self) -> usize { self.0.len() * Section::VOLUME }

    /// The height of the [`VecChunk`] in blocks.
    #[inline]
    #[must_use]
    pub fn height(&self) -> usize { self.0.len() * Section::HEIGHT }

    /// The width of the [`VecChunk`] in blocks.
    #[inline]
    #[must_use]
    pub const fn width(&self) -> usize { Self::WIDTH }

    /// The depth of the [`VecChunk`] in blocks.
    #[inline]
    #[must_use]
    pub const fn depth(&self) -> usize { Self::DEPTH }

    /// Create a new [`VecChunk`] with the given offset.
    #[must_use]
    pub const fn new(offset: i32) -> Self { Self(Vec::new(), offset) }

    /// Create a new [`VecChunk`] from a list of [`Section`]s.
    #[must_use]
    pub fn new_from(sections: impl Into<Vec<Section>>, offset: i32) -> Self {
        Self(sections.into(), offset)
    }

    /// Get a reference to the [`Section`]s in the [`VecChunk`].
    #[inline]
    #[must_use]
    pub const fn sections(&self) -> &Vec<Section> { &self.0 }

    /// Get a mutable reference to the [`Section`]s in the [`VecChunk`].
    #[inline]
    #[must_use]
    pub const fn sections_mut(&mut self) -> &mut Vec<Section> { &mut self.0 }

    /// Get a reference to a [`Section`] based on the `y` coordinate.
    #[inline]
    #[must_use]
    pub fn get_section(&self, y_coord: i32) -> Option<&Section> {
        self.get_nonoffset_section(y_coord.checked_add(self.1)?)
    }

    /// Get a reference to a [`Section`] based on the `y` coordinate.
    ///
    /// # Note
    /// This does not take into account the chunk offset.
    #[inline]
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    fn get_nonoffset_section(&self, y_coord: i32) -> Option<&Section> {
        self.0.get(y_coord as usize / Section::HEIGHT)
    }

    /// Get a mutable reference to a [`Section`] based on the `y` coordinate.
    #[inline]
    #[must_use]
    pub fn get_section_mut(&mut self, y_coord: i32) -> Option<&mut Section> {
        self.get_nonoffset_section_mut(y_coord.checked_add(self.1)?)
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

    /// Get a block from the [`VecChunk`].
    ///
    /// Returns `None` if the `y` coordinate is out of bounds.
    #[must_use]
    pub fn get_block_raw(&self, mut position: IVec3) -> Option<u32> {
        position.y = position.y.checked_add(self.1)?;
        self.get_nonoffset_section(position.y).map(|s| s.get_block(position))
    }

    /// Set a block in the [`VecChunk`].
    ///
    /// Returns `None` if the y coordinate is out of bounds.
    #[must_use]
    pub fn set_block_raw(&mut self, mut position: IVec3, block: u32) -> Option<u32> {
        position.y = position.y.checked_add(self.1)?;
        self.get_nonoffset_section_mut(position.y).map(|s| s.set_block(position, block))
    }

    /// Get a block from the [`VecChunk`] with data from the [`BlockStorage`].
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if no matching block is found.
    #[must_use]
    #[cfg(feature = "block")]
    pub fn get_block_untyped<V: froglight_common::version::Version>(
        &self,
        position: IVec3,
        storage: &froglight_block::storage::BlockStorage<V>,
    ) -> Option<froglight_block::block::UntypedBlock<V>> {
        self.get_block_raw(position).and_then(|id| {
            storage.get_untyped(froglight_block::storage::GlobalBlockId::new_unchecked(id))
        })
    }

    /// Set a block in the [`VecChunk`] using data from the [`BlockStorage`].
    ///
    /// Returns the previous block if it was set, or
    /// `None` if the position is out of bounds or no matching block is found.
    #[cfg(feature = "block")]
    pub fn set_block_untyped<V: froglight_common::version::Version>(
        &mut self,
        position: IVec3,
        block: impl Into<froglight_block::block::UntypedBlock<V>>,
        storage: &froglight_block::storage::BlockStorage<V>,
    ) -> Option<froglight_block::block::UntypedBlock<V>> {
        self.set_block_raw(position, *storage.get_global(block)?).and_then(|id| {
            storage.get_untyped(froglight_block::storage::GlobalBlockId::new_unchecked(id))
        })
    }

    /// Try to convert the [`VecChunk`] into an [`ArrayChunk`].
    ///
    /// # Errors
    /// Returns the [`VecChunk`] if the number of sections is not equal to
    /// `SECTIONS`.
    #[inline]
    #[expect(clippy::missing_panics_doc)]
    pub fn try_into_array<const SECTIONS: usize, const OFFSET: i32>(
        self,
    ) -> Result<ArrayChunk<SECTIONS, OFFSET>, Self> {
        if self.sections().len() == SECTIONS {
            let array: [Section; SECTIONS] = self.0.try_into().ok().unwrap();
            Ok(ArrayChunk::<SECTIONS, OFFSET>::from(array))
        } else {
            Err(self)
        }
    }
}

impl<const SECTIONS: usize, const OFFSET: i32> TryFrom<VecChunk> for ArrayChunk<SECTIONS, OFFSET> {
    type Error = VecChunk;

    fn try_from(value: VecChunk) -> Result<Self, Self::Error> { value.try_into_array() }
}

#[test]
fn dimensions() {
    let mut chunk = VecChunk::new(-64);
    assert_eq!(chunk.height(), 0, "VecChunk 0 height is incorrect!");
    assert_eq!(chunk.volume(), 0, "VecChunk 0 volume is incorrect!");

    chunk.sections_mut().push(Section::default());
    assert_eq!(chunk.height(), 16, "VecChunk 1 height is incorrect!");
    assert_eq!(chunk.volume(), 4096, "VecChunk 1 volume is incorrect!");

    chunk.sections_mut().push(Section::default());
    assert_eq!(chunk.height(), 32, "VecChunk 2 height is incorrect!");
    assert_eq!(chunk.volume(), 8192, "VecChunk 2 volume is incorrect!");

    chunk.sections_mut().push(Section::default());
    assert_eq!(chunk.height(), 48, "VecChunk 3 height is incorrect!");
    assert_eq!(chunk.volume(), 12288, "VecChunk 3 volume is incorrect!");
}
