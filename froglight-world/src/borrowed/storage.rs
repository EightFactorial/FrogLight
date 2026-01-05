//! TODO

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::ops::Deref;

#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
#[cfg(feature = "alloc")]
use smallvec::SmallVec;

use crate::borrowed::BorrowedSection;

/// A storage container for multiple [`BorrowedSection`]s.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(opaque, Clone))]
#[expect(clippy::large_enum_variant, reason = "This is by design")]
pub enum BorrowedChunkStorage<'a> {
    /// A large chunk.
    ///
    /// Typically used for overworld chunks.
    Large(BorrowedArrayStorage<'a, 24, -64>),
    /// A normal chunk.
    ///
    /// Typically used for nether and end chunks.
    Normal(BorrowedArrayStorage<'a, 16, 0>),
    /// A chunk of some other variable size.
    ///
    /// May be used for custom worlds or in other special cases.
    #[cfg(feature = "alloc")]
    Variable(BorrowedVecStorage<'a>),
}

impl<'a> BorrowedChunkStorage<'a> {
    /// Create a new [`BorrowedChunkStorage::Large`].
    #[must_use]
    pub const fn new_large(sections: [BorrowedSection<'a>; 24]) -> Self {
        Self::Large(BorrowedArrayStorage::new(sections))
    }

    /// Create an empty [`BorrowedChunkStorage::Large`].
    #[must_use]
    pub fn empty_large() -> Self {
        Self::Large(BorrowedArrayStorage::new(core::array::from_fn(|_| BorrowedSection::default())))
    }

    /// Create a new [`BorrowedChunkStorage::Normal`].
    #[must_use]
    pub const fn new_normal(sections: [BorrowedSection<'a>; 16]) -> Self {
        Self::Normal(BorrowedArrayStorage::new(sections))
    }

    /// Create an empty [`BorrowedChunkStorage::Normal`].
    #[must_use]
    pub fn empty_normal() -> Self {
        Self::Normal(BorrowedArrayStorage::new(core::array::from_fn(|_| {
            BorrowedSection::default()
        })))
    }

    /// Create a new [`BorrowedChunkStorage::Variable`].
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn new_variable(sections: Vec<BorrowedSection<'a>>, offset: i32) -> Self {
        Self::Variable(BorrowedVecStorage::new_from_vec(sections, offset))
    }

    /// Get the vertical offset of the [`BorrowedChunkStorage`].
    #[must_use]
    pub const fn offset(&self) -> i32 {
        match self {
            BorrowedChunkStorage::Large(storage) => storage.offset(),
            BorrowedChunkStorage::Normal(storage) => storage.offset(),
            #[cfg(feature = "alloc")]
            BorrowedChunkStorage::Variable(storage) => storage.offset(),
        }
    }

    /// Get the number of sections in the [`BorrowedChunkStorage`].
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            BorrowedChunkStorage::Large(storage) => storage.len(),
            BorrowedChunkStorage::Normal(storage) => storage.len(),
            #[cfg(feature = "alloc")]
            BorrowedChunkStorage::Variable(storage) => storage.len(),
        }
    }

    /// Returns `true` if the [`BorrowedChunkStorage`] contains no sections.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            BorrowedChunkStorage::Large(storage) => storage.is_empty(),
            BorrowedChunkStorage::Normal(storage) => storage.is_empty(),
            #[cfg(feature = "alloc")]
            BorrowedChunkStorage::Variable(storage) => storage.is_empty(),
        }
    }

    /// Get the list of [`BorrowedSection`]s as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[BorrowedSection<'a>] {
        match self {
            BorrowedChunkStorage::Large(storage) => storage.0.as_slice(),
            BorrowedChunkStorage::Normal(storage) => storage.0.as_slice(),
            #[cfg(feature = "alloc")]
            BorrowedChunkStorage::Variable(storage) => storage.0.as_slice(),
        }
    }

    /// Create a new [`BorrowedChunkStorage`] from a [`Vec<BorrowedSection>`].
    ///
    /// Returns a specialized storage type if the length and offset match
    /// known configurations.
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn new_from_vec(
        sections: Vec<BorrowedSection<'a>>,
        offset: i32,
    ) -> BorrowedChunkStorage<'a> {
        match (sections.len(), offset) {
            (24, -64) => {
                // SAFETY: We have already checked that the length is 24.
                let array: [BorrowedSection<'a>; 24] =
                    unsafe { sections.try_into().unwrap_unchecked() };
                BorrowedChunkStorage::Large(BorrowedArrayStorage::new(array))
            }
            (16, 0) => {
                // SAFETY: We have already checked that the length is 16.
                let array: [BorrowedSection<'a>; 16] =
                    unsafe { sections.try_into().unwrap_unchecked() };
                BorrowedChunkStorage::Normal(BorrowedArrayStorage::new(array))
            }
            _ => BorrowedChunkStorage::Variable(BorrowedVecStorage::new_from_vec(sections, offset)),
        }
    }
}

impl<'a> Deref for BorrowedChunkStorage<'a> {
    type Target = [BorrowedSection<'a>];

    fn deref(&self) -> &Self::Target { self.as_slice() }
}

impl Default for BorrowedChunkStorage<'_> {
    fn default() -> Self { Self::empty_large() }
}

// -------------------------------------------------------------------------------------------------

/// A vertical slice of the world.
///
/// Has a constant, known number of sections and a known offset.
///
/// ---
///
/// Storing [`Section`]s in a fixed-size array has two main benefits:
///
/// 1. It guarantees that the number of sections is always correct.
/// 2. It prevents unnecessary bounds checks when accessing the array.
#[derive(Clone)]
pub struct BorrowedArrayStorage<'a, const SECTIONS: usize, const OFFSET: i32>(
    [BorrowedSection<'a>; SECTIONS],
);

impl<'a, const SECTIONS: usize, const OFFSET: i32> BorrowedArrayStorage<'a, SECTIONS, OFFSET> {
    /// Create a new [`ArrayChunkStorage`] from the given [`Section`]s.
    #[must_use]
    pub const fn new(sections: [BorrowedSection<'a>; SECTIONS]) -> Self { Self(sections) }

    /// Get the vertical offset of the storage.
    #[must_use]
    pub const fn offset(&self) -> i32 { OFFSET }

    /// Get the number of sections in the storage.
    #[must_use]
    pub const fn len(&self) -> usize { SECTIONS }

    /// Returns `true` if the storage contains no sections.
    #[must_use]
    pub const fn is_empty(&self) -> bool { SECTIONS == 0 }
}

impl<'a, const SECTIONS: usize, const OFFSET: i32> Deref
    for BorrowedArrayStorage<'a, SECTIONS, OFFSET>
{
    type Target = [BorrowedSection<'a>; SECTIONS];

    fn deref(&self) -> &Self::Target { &self.0 }
}

// -------------------------------------------------------------------------------------------------

/// A vertical slice of the world.
///
/// Has a variable number of sections and a known offset.
#[derive(Clone)]
#[cfg(feature = "alloc")]
pub struct BorrowedVecStorage<'a>(SmallVec<[BorrowedSection<'a>; 16]>, i32);

#[cfg(feature = "alloc")]
impl<'a> BorrowedVecStorage<'a> {
    /// Create a new [`VecChunkStorage`] from the given [`Section`]s and
    /// offset.
    #[must_use]
    pub fn new(sections: SmallVec<[BorrowedSection<'a>; 16]>, offset: i32) -> Self {
        Self(sections, offset)
    }

    /// Create a new [`VecChunkStorage`] from the given [`Section`]s and
    /// offset.
    #[must_use]
    pub fn new_from_vec(sections: Vec<BorrowedSection<'a>>, offset: i32) -> Self {
        Self(SmallVec::from_vec(sections), offset)
    }

    /// Get the vertical offset of the storage.
    #[must_use]
    pub const fn offset(&self) -> i32 { self.1 }

    /// Get the number of sections in the storage.
    #[must_use]
    pub fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the storage contains no sections.
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

#[cfg(feature = "alloc")]
impl<'a> Deref for BorrowedVecStorage<'a> {
    type Target = SmallVec<[BorrowedSection<'a>; 16]>;

    fn deref(&self) -> &Self::Target { &self.0 }
}
