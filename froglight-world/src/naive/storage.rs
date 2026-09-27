//! TODO

use alloc::{alloc::Global, boxed::Box, vec::Vec};
use core::{
    alloc::Allocator,
    ops::{Deref, DerefMut},
};

use crate::section::Section;

/// A storage container for multiple [`Section`]s.
#[derive(Clone)]
pub enum ChunkStorage<A: Allocator = Global> {
    /// A large chunk.
    ///
    /// Typically used for overworld chunks.
    Large(ArrayStorage<24, -64, A>),
    /// A normal chunk.
    ///
    /// Typically used for nether and end chunks.
    Normal(ArrayStorage<16, 0, A>),
    /// A chunk of some other variable size.
    ///
    /// May be used for custom worlds or in other special cases.
    Variable(VecStorage<A>),
}

impl<A: Allocator + Default> ChunkStorage<A> {
    /// Create a new [`ChunkStorage::Large`].
    #[must_use]
    pub fn new_large(sections: [Section; 24]) -> Self { Self::new_large_in(sections, A::default()) }

    /// Create a new [`ChunkStorage::Normal`].
    #[must_use]
    pub fn new_normal(sections: [Section; 16]) -> Self {
        Self::new_normal_in(sections, A::default())
    }

    /// Create an empty [`ChunkStorage::Large`].
    #[must_use]
    pub fn empty_large() -> Self { Self::Large(ArrayStorage::new_empty_in(A::default())) }

    /// Create an empty [`ChunkStorage::Normal`].
    #[must_use]
    pub fn empty_normal() -> Self { Self::Normal(ArrayStorage::new_empty_in(A::default())) }

    /// Create an empty [`ChunkStorage::Variable`].
    #[must_use]
    pub fn empty_variable(offset: i32) -> Self {
        Self::Variable(VecStorage::new_empty_in(offset, A::default()))
    }
}

impl<A: Allocator> ChunkStorage<A> {
    /// Create a new [`ChunkStorage`] from a [`Vec<Section>`].
    ///
    /// Returns a specialized storage type if the length and offset match
    /// known configurations.
    #[must_use]
    pub fn new(sections: Vec<Section, A>, offset: i32) -> Self {
        match (sections.len(), offset) {
            (24, -64) => {
                // SAFETY: We have already checked that the length is 24.
                #[cfg(feature = "nightly")]
                let sections: Box<[Section; 24], A> =
                    unsafe { sections.into_array().unwrap_unchecked() };

                // SAFETY: We have already checked that the length is 24.
                #[cfg(not(feature = "nightly"))]
                let sections: Box<[Section; 24], A> =
                    unsafe { sections.into_boxed_slice().try_into().unwrap_unchecked() };

                Self::new_large_boxed(sections)
            }
            (16, 0) => {
                // SAFETY: We have already checked that the length is 16.
                #[cfg(feature = "nightly")]
                let sections: Box<[Section; 16], A> =
                    unsafe { sections.into_array().unwrap_unchecked() };

                // SAFETY: We have already checked that the length is 16.
                #[cfg(not(feature = "nightly"))]
                let sections: Box<[Section; 16], A> =
                    unsafe { sections.into_boxed_slice().try_into().unwrap_unchecked() };

                Self::new_normal_boxed(sections)
            }
            _ => Self::new_variable(sections, offset),
        }
    }

    /// Create a new [`ChunkStorage::Large`].
    #[inline]
    #[must_use]
    pub fn new_large_in(sections: [Section; 24], alloc: A) -> Self {
        Self::Large(ArrayStorage::new_in(sections, alloc))
    }

    /// Create a new [`ChunkStorage::Normal`].
    #[inline]
    #[must_use]
    pub fn new_normal_in(sections: [Section; 16], alloc: A) -> Self {
        Self::Normal(ArrayStorage::new_in(sections, alloc))
    }

    /// Create a new [`ChunkStorage::Large`].
    #[inline]
    #[must_use]
    pub const fn new_large_boxed(sections: Box<[Section; 24], A>) -> Self {
        ChunkStorage::Large(ArrayStorage::new(sections))
    }

    /// Create a new [`ChunkStorage::Normal`].
    #[inline]
    #[must_use]
    pub const fn new_normal_boxed(sections: Box<[Section; 16], A>) -> Self {
        ChunkStorage::Normal(ArrayStorage::new(sections))
    }

    /// Create a new [`ChunkStorage::Variable`].
    ///
    /// If you do not specifically need [`ChunkStorage::Variable`], use
    /// [`ChunkStorage::new`] instead.
    #[inline]
    #[must_use]
    pub const fn new_variable(sections: Vec<Section, A>, offset: i32) -> Self {
        Self::Variable(VecStorage::new(sections, offset))
    }

    /// Get the vertical offset of the [`ChunkStorage`].
    #[must_use]
    pub const fn offset(&self) -> i32 {
        match self {
            ChunkStorage::Large(storage) => storage.offset(),
            ChunkStorage::Normal(storage) => storage.offset(),
            ChunkStorage::Variable(storage) => storage.offset(),
        }
    }

    /// Get the number of sections in the [`ChunkStorage`].
    #[must_use]
    pub const fn len(&self) -> usize {
        match self {
            ChunkStorage::Large(storage) => storage.len(),
            ChunkStorage::Normal(storage) => storage.len(),
            ChunkStorage::Variable(storage) => storage.len(),
        }
    }

    /// Returns `true` if the [`ChunkStorage`] contains no sections.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        match self {
            ChunkStorage::Large(storage) => storage.is_empty(),
            ChunkStorage::Normal(storage) => storage.is_empty(),
            ChunkStorage::Variable(storage) => storage.is_empty(),
        }
    }

    /// Get the list of [`Section`]s as a slice.
    #[must_use]
    pub const fn as_slice(&self) -> &[Section] {
        match self {
            ChunkStorage::Large(storage) => storage.0.as_slice(),
            ChunkStorage::Normal(storage) => storage.0.as_slice(),
            ChunkStorage::Variable(storage) => storage.0.as_slice(),
        }
    }

    /// Get the list of [`Section`]s as a mutable slice.
    #[must_use]
    pub const fn as_slice_mut(&mut self) -> &mut [Section] {
        match self {
            ChunkStorage::Large(storage) => storage.0.as_mut_slice(),
            ChunkStorage::Normal(storage) => storage.0.as_mut_slice(),
            ChunkStorage::Variable(storage) => storage.0.as_mut_slice(),
        }
    }
}

impl<A: Allocator> Deref for ChunkStorage<A> {
    type Target = [Section];

    #[inline]
    fn deref(&self) -> &Self::Target { self.as_slice() }
}

impl<A: Allocator + Default> Default for ChunkStorage<A> {
    #[inline]
    fn default() -> Self { Self::empty_large() }
}

impl<A: Allocator> PartialEq for ChunkStorage<A> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.offset() == other.offset() && self.as_slice() == other.as_slice()
    }
}
impl<A: Allocator> Eq for ChunkStorage<A> {}

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
pub struct ArrayStorage<const SECTIONS: usize, const OFFSET: i32, A: Allocator = Global>(
    Box<[Section; SECTIONS], A>,
);

impl<const SECTIONS: usize, const OFFSET: i32, A: Allocator> ArrayStorage<SECTIONS, OFFSET, A> {
    /// Create a new [`ArrayStorage`] from the given [`Section`]s.
    #[inline]
    #[must_use]
    pub const fn new(sections: Box<[Section; SECTIONS], A>) -> Self { Self(sections) }

    /// Create a new [`ArrayStorage`] from the given [`Section`]s and
    /// allocator.
    #[inline]
    #[must_use]
    pub fn new_in(sections: [Section; SECTIONS], alloc: A) -> Self {
        Self(Box::new_in(sections, alloc))
    }

    /// Create a new, empty [`ArrayStorage`] with the given allocator.
    #[must_use]
    pub fn new_empty_in(alloc: A) -> Self {
        Self::new_in(core::array::from_fn(|_| Section::new_empty()), alloc)
    }

    /// Get the vertical offset of the storage.
    #[inline]
    #[must_use]
    pub const fn offset(&self) -> i32 { OFFSET }

    /// Get the number of sections in the storage.
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize { SECTIONS }

    /// Returns `true` if the storage contains no sections.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { SECTIONS == 0 }
}

impl<const SECTIONS: usize, const OFFSET: i32, A: Allocator + Default> From<[Section; SECTIONS]>
    for ArrayStorage<SECTIONS, OFFSET, A>
{
    #[inline]
    fn from(sections: [Section; SECTIONS]) -> Self { Self::new_in(sections, A::default()) }
}
impl<const SECTIONS: usize, const OFFSET: i32, A: Allocator> From<Box<[Section; SECTIONS], A>>
    for ArrayStorage<SECTIONS, OFFSET, A>
{
    #[inline]
    fn from(sections: Box<[Section; SECTIONS], A>) -> Self { Self::new(sections) }
}

impl<const SECTIONS: usize, const OFFSET: i32, A: Allocator> Deref
    for ArrayStorage<SECTIONS, OFFSET, A>
{
    type Target = Box<[Section; SECTIONS], A>;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl<const SECTIONS: usize, const OFFSET: i32, A: Allocator> DerefMut
    for ArrayStorage<SECTIONS, OFFSET, A>
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<const SECTIONS: usize, const OFFSET: i32, A: Allocator> PartialEq
    for ArrayStorage<SECTIONS, OFFSET, A>
{
    #[inline]
    fn eq(&self, other: &Self) -> bool { self.as_slice() == other.as_slice() }
}
impl<const SECTIONS: usize, const OFFSET: i32, A: Allocator> Eq
    for ArrayStorage<SECTIONS, OFFSET, A>
{
}

// -------------------------------------------------------------------------------------------------

/// A vertical slice of the world.
///
/// Has a variable number of sections and a known offset.
#[derive(Clone)]
pub struct VecStorage<A: Allocator = Global>(Vec<Section, A>, i32);

impl<A: Allocator> VecStorage<A> {
    /// Create a new [`VecStorage`] from the given [`Section`]s and offset.
    #[inline]
    #[must_use]
    pub const fn new(sections: Vec<Section, A>, offset: i32) -> Self { Self(sections, offset) }

    /// Create a new, empty [`VecStorage`] from the given offset.
    #[inline]
    #[must_use]
    pub fn new_empty_in(offset: i32, alloc: A) -> Self { Self::new(Vec::new_in(alloc), offset) }

    /// Get the vertical offset of the storage.
    #[inline]
    #[must_use]
    pub const fn offset(&self) -> i32 { self.1 }

    /// Get the number of sections in the storage.
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the storage contains no sections.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.0.is_empty() }
}

impl<A: Allocator> Deref for VecStorage<A> {
    type Target = Vec<Section, A>;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl<A: Allocator> DerefMut for VecStorage<A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<A: Allocator> PartialEq for VecStorage<A> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1 && self.0.as_slice() == other.0.as_slice()
    }
}
impl<A: Allocator> Eq for VecStorage<A> {}
