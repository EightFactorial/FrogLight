//! TODO

use alloc::{boxed::Box, vec::Vec};
use core::fmt::Debug;

#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;

use crate::section::Section;

/// A vertical slice of [`Section`]s.
///
/// Has two common presets for vanilla worlds,
/// as well as a generic variant for any custom worlds.
#[derive(Debug, Clone)]
pub enum SectionStorage {
    /// A normal chunk.
    ///
    /// Typically used for nether and end chunks.
    Normal(ArrayStorage<16, 0>),
    /// A large chunk.
    ///
    /// Typically used for overworld chunks.
    Large(ArrayStorage<24, -64>),
    /// A chunk of some other size.
    ///
    /// May be used for custom worlds or other special cases.
    Other(VecStorage),
}

impl SectionStorage {
    /// Create a new empty [`SectionStorage::Normal`].
    #[must_use]
    pub fn empty_normal() -> Self { Self::Normal(ArrayStorage::<16, 0>::empty()) }

    /// Create a new empty [`SectionStorage::Large`].
    #[must_use]
    pub fn empty_large() -> Self { Self::Large(ArrayStorage::<24, -64>::empty()) }

    /// Create a [`SectionStorage`] from a `Vec<Section>` and a world offset.
    #[must_use]
    pub fn new(sections: Vec<Section>, offset: isize) -> Self {
        match (sections.len(), offset) {
            (16, 0) => Self::Normal(
                ArrayStorage::try_from(sections)
                    .unwrap_or_else(|_| unreachable!("Length of `Vec` is equal to `SECTIONS`")),
            ),
            (24, -64) => Self::Large(
                ArrayStorage::try_from(sections)
                    .unwrap_or_else(|_| unreachable!("Length of `Vec` is equal to `SECTIONS`")),
            ),
            _ => Self::Other(VecStorage::new(sections, offset)),
        }
    }

    /// Return the number of sections in this storage.
    #[must_use]
    pub const fn len(&self) -> usize {
        match self {
            Self::Normal(s) => s.len(),
            Self::Large(s) => s.len(),
            Self::Other(s) => s.len(),
        }
    }

    /// Returns `true` if this storage contains either no [`Section`]s or only
    /// air.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Normal(s) => s.is_empty(),
            Self::Large(s) => s.is_empty(),
            Self::Other(s) => s.is_empty(),
        }
    }

    /// Get a reference to the [`Section`] at the given index.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Section> {
        match self {
            Self::Normal(s) => s.get(index),
            Self::Large(s) => s.get(index),
            Self::Other(s) => s.get(index),
        }
    }

    /// Get a mutable reference to the [`Section`] at the given index.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Section> {
        match self {
            Self::Normal(s) => s.get_mut(index),
            Self::Large(s) => s.get_mut(index),
            Self::Other(s) => s.get_mut(index),
        }
    }

    /// Return the minimum height (inclusive) of this storage.
    #[must_use]
    pub const fn min_height(&self) -> isize {
        match self {
            Self::Normal(s) => s.min_height(),
            Self::Large(s) => s.min_height(),
            Self::Other(s) => s.min_height(),
        }
    }

    /// Return the maximum height (exclusive) of this storage.
    #[must_use]
    pub const fn max_height(&self) -> isize {
        match self {
            Self::Normal(s) => s.max_height(),
            Self::Large(s) => s.max_height(),
            Self::Other(s) => s.max_height(),
        }
    }

    /// Return the total number of blocks able to be stored in this storage.
    #[must_use]
    pub const fn volume(&self) -> usize {
        match self {
            Self::Normal(s) => s.volume(),
            Self::Large(s) => s.volume(),
            Self::Other(s) => s.volume(),
        }
    }

    /// Convert this into a [`SectionStorage::Other`]
    /// by moving all sections into a [`Vec`] without cloning.
    pub fn into_vec(&mut self) {
        if let Self::Other(_) = self {
            // If already `Other`, do nothing.
            return;
        }

        // Replace `self` with an empty `Other` to take ownership of the original.
        let original = core::mem::replace(self, Self::Other(VecStorage::new(Vec::new(), 0)));
        // Get a mutable reference to the newly created `VecStorage`.
        let Self::Other(VecStorage(storage, _)) = self else {
            unreachable!("`self` was just set to `Other`")
        };

        match original {
            Self::Normal(s) => {
                let boxed_slice: Box<[Section]> = s.0;
                *storage = boxed_slice.into_vec();
            }
            Self::Large(s) => {
                let boxed_slice: Box<[Section]> = s.0;
                *storage = boxed_slice.into_vec();
            }
            Self::Other(_) => unreachable!("Impossible to reach, already covered above."),
        }
    }
}

impl AsRef<[Section]> for SectionStorage {
    fn as_ref(&self) -> &[Section] {
        match self {
            Self::Normal(s) => s.as_ref(),
            Self::Large(s) => s.as_ref(),
            Self::Other(s) => s.as_ref(),
        }
    }
}
impl AsMut<[Section]> for SectionStorage {
    fn as_mut(&mut self) -> &mut [Section] {
        match self {
            Self::Normal(s) => s.as_mut(),
            Self::Large(s) => s.as_mut(),
            Self::Other(s) => s.as_mut(),
        }
    }
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
#[repr(transparent)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(opaque, Clone))]
pub struct ArrayStorage<const SECTIONS: usize, const OFFSET: isize>(Box<[Section; SECTIONS]>);

impl<const SECTIONS: usize, const OFFSET: isize> ArrayStorage<SECTIONS, OFFSET> {
    /// The total number of blocks able to be stored in this storage.
    pub const VOLUME: usize = SECTIONS * Section::VOLUME;

    /// Create a new [`ArrayStorage`] from the given [`Section`]s.
    #[inline]
    #[must_use]
    pub fn new(sections: [Section; SECTIONS]) -> Self { Self::const_new(Box::new(sections)) }

    /// Create a new empty [`ArrayStorage`] filled with [`Section::AIR`].
    #[inline]
    #[must_use]
    pub fn empty() -> Self { Self::const_new(Box::new([Section::AIR; SECTIONS])) }

    /// Create a new [`ArrayStorage`] from the given [`Section`]s.
    #[inline]
    #[must_use]
    pub const fn const_new(sections: Box<[Section; SECTIONS]>) -> Self { Self(sections) }

    /// Return the number of sections in this storage.
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize { SECTIONS }

    /// Returns `true` if this storage contains no sections or only air.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { SECTIONS == 0 || self.0.iter().all(Section::is_empty) }

    /// Get a reference to the [`Section`] at the given index.
    ///
    /// Returns `None` if the index is out of bounds.
    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Section> { self.0.get(index) }

    /// Get a mutable reference to the [`Section`] at the given index.
    ///
    /// Returns `None` if the index is out of bounds.
    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Section> { self.0.get_mut(index) }

    /// Return the minimum height (inclusive) of this storage.
    #[inline]
    #[must_use]
    pub const fn min_height(&self) -> isize { OFFSET }

    /// Return the maximum height (exclusive) of this storage.
    #[must_use]
    #[expect(clippy::cast_possible_wrap, reason = "We want to wrap, as negatives are possible.")]
    pub const fn max_height(&self) -> isize {
        (self.len() * Section::SIDE_LENGTH).wrapping_add_signed(self.min_height()) as isize
    }

    /// Return the total number of blocks able to be stored in this storage.
    #[inline]
    #[must_use]
    pub const fn volume(&self) -> usize { Self::VOLUME }

    // /// Get a reference to the [`Section`] that holds the given [`BlockPos`].
    // #[must_use]
    // pub fn get(&self, pos: BlockPos) -> Option<&Section> {
    //     let pos: usize = (pos.y() as
    // isize).saturating_sub(OFFSET).try_into().ok()?;     self.0.get(pos /
    // Section::HEIGHT) }
    //
    // /// Get a mutable reference to the [`Section`] that holds the given
    // /// [`BlockPos`].
    // #[must_use]
    // pub fn get_mut(&mut self, pos: BlockPos) -> Option<&mut Section> {
    //     let pos: usize = (pos.y() as
    // isize).saturating_sub(OFFSET).try_into().ok()?;     self.0.get_mut(pos /
    // Section::HEIGHT) }
}

impl<const SECTIONS: usize, const OFFSET: isize> Debug for ArrayStorage<SECTIONS, OFFSET> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ArrayStorage<{SECTIONS}, {OFFSET}> {{ ... }}")
    }
}

impl<const SECTIONS: usize, const OFFSET: isize> Default for ArrayStorage<SECTIONS, OFFSET> {
    fn default() -> Self { Self::empty() }
}

impl<const SECTIONS: usize, const OFFSET: isize> TryFrom<Vec<Section>>
    for ArrayStorage<SECTIONS, OFFSET>
{
    type Error = Vec<Section>;

    fn try_from(value: Vec<Section>) -> Result<Self, Self::Error> {
        if value.len() == SECTIONS {
            Ok(Self::new(
                value
                    .try_into()
                    .unwrap_or_else(|_| unreachable!("Length of `Vec` is equal to `SECTIONS`")),
            ))
        } else {
            Err(value)
        }
    }
}

impl<const SECTIONS: usize, const OFFSET: isize> AsRef<[Section]>
    for ArrayStorage<SECTIONS, OFFSET>
{
    fn as_ref(&self) -> &[Section] { self.0.as_ref() }
}
impl<const SECTIONS: usize, const OFFSET: isize> AsMut<[Section]>
    for ArrayStorage<SECTIONS, OFFSET>
{
    fn as_mut(&mut self) -> &mut [Section] { self.0.as_mut() }
}

// -------------------------------------------------------------------------------------------------

/// A vertical slice of the world.
///
/// Has a variable number of sections and a known offset.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(opaque, Clone))]
pub struct VecStorage(Vec<Section>, isize);

impl VecStorage {
    /// Create a new [`VecStorage`] from the given [`Section`]s and offset.
    #[inline]
    #[must_use]
    pub const fn new(sections: Vec<Section>, offset: isize) -> Self { Self(sections, offset) }

    /// Return the number of sections in this storage.
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if this storage contains no sections.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() || self.0.iter().all(Section::is_empty) }

    /// Get a reference to the [`Section`] at the given index.
    ///
    /// Returns `None` if the index is out of bounds.
    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Section> { self.0.get(index) }

    /// Get a mutable reference to the [`Section`] at the given index.
    ///
    /// Returns `None` if the index is out of bounds.
    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Section> { self.0.get_mut(index) }

    /// Return the minimum height (inclusive) of this storage.
    #[inline]
    #[must_use]
    pub const fn min_height(&self) -> isize { self.1 }

    /// Return the maximum height (exclusive) of this storage.
    #[must_use]
    #[expect(clippy::cast_possible_wrap, reason = "We want to wrap, as negatives are possible.")]
    pub const fn max_height(&self) -> isize {
        (self.len() * Section::SIDE_LENGTH).wrapping_add_signed(self.min_height()) as isize
    }

    /// Return the total number of blocks able to be stored in this storage.
    #[inline]
    #[must_use]
    pub const fn volume(&self) -> usize { self.len() * Section::VOLUME }

    // /// Get a reference to the [`Section`] that holds the given [`BlockPos`].
    // #[must_use]
    // pub fn get(&self, pos: BlockPos) -> Option<&Section> {
    //     self.0.get::<usize>((pos.y() as
    // isize).saturating_sub(self.1).try_into().ok()?) }
    //
    // /// Get a mutable reference to the [`Section`] that holds the given
    // /// [`BlockPos`].
    // #[must_use]
    // pub fn get_mut(&mut self, pos: BlockPos) -> Option<&mut Section> {
    //     self.0.get_mut::<usize>((pos.y() as
    // isize).saturating_sub(self.1).try_into().ok()?) }
}

impl Debug for VecStorage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "VecStorage {{ len: {}, offset: {} }}", self.len(), self.1)
    }
}

impl AsRef<Vec<Section>> for VecStorage {
    fn as_ref(&self) -> &Vec<Section> { &self.0 }
}
impl AsRef<[Section]> for VecStorage {
    fn as_ref(&self) -> &[Section] { self.0.as_ref() }
}

impl AsMut<Vec<Section>> for VecStorage {
    fn as_mut(&mut self) -> &mut Vec<Section> { &mut self.0 }
}
impl AsMut<[Section]> for VecStorage {
    fn as_mut(&mut self) -> &mut [Section] { self.0.as_mut() }
}
