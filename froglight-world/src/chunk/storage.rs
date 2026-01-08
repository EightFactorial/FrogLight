//! TODO

use alloc::{boxed::Box, vec::Vec};
use core::ops::Deref;

#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;

use crate::chunk::Section;

/// A storage container for multiple [`Section`]s.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(opaque, Clone))]
#[allow(clippy::large_enum_variant, reason = "This is by design")]
pub enum ChunkStorage {
    /// A large chunk.
    ///
    /// Typically used for overworld chunks.
    Large(ArrayChunkStorage<24, -64>),
    /// A normal chunk.
    ///
    /// Typically used for nether and end chunks.
    Normal(ArrayChunkStorage<16, 0>),
    /// A chunk of some other variable size.
    ///
    /// May be used for custom worlds or in other special cases.
    Variable(VecChunkStorage),
}

impl ChunkStorage {
    /// Create a new [`ChunkStorage::Large`].
    #[must_use]
    pub fn new_large(sections: [Section; 24]) -> Self {
        Self::Large(ArrayChunkStorage::new(sections))
    }

    /// Create an empty [`ChunkStorage::Large`].
    #[must_use]
    pub fn empty_large() -> Self {
        Self::Large(ArrayChunkStorage::new(core::array::from_fn(|_| Section::default())))
    }

    /// Create a new [`ChunkStorage::Normal`].
    #[must_use]
    pub fn new_normal(sections: [Section; 16]) -> Self {
        Self::Normal(ArrayChunkStorage::new(sections))
    }

    /// Create an empty [`ChunkStorage::Normal`].
    #[must_use]
    pub fn empty_normal() -> Self {
        Self::Normal(ArrayChunkStorage::new(core::array::from_fn(|_| Section::default())))
    }

    /// Create a new [`ChunkStorage::Variable`].
    #[must_use]
    pub const fn new_variable(sections: Vec<Section>, offset: i32) -> Self {
        Self::Variable(VecChunkStorage::new(sections, offset))
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
    pub const fn as_mut_slice(&mut self) -> &mut [Section] {
        match self {
            ChunkStorage::Large(storage) => storage.0.as_mut_slice(),
            ChunkStorage::Normal(storage) => storage.0.as_mut_slice(),
            ChunkStorage::Variable(storage) => storage.0.as_mut_slice(),
        }
    }

    /// Create a new [`ChunkStorage`] from a [`Vec<Section>`].
    ///
    /// Returns a specialized storage type if the length and offset match
    /// known configurations.
    #[must_use]
    pub fn new_from_vec(sections: Vec<Section>, offset: i32) -> ChunkStorage {
        match (sections.len(), offset) {
            #[cfg(feature = "nightly")]
            (24, -64) => {
                // SAFETY: We have already checked that the length is 24.
                let array: Box<[Section; 24]> =
                    unsafe { sections.into_boxed_slice().into_array().unwrap_unchecked() };
                ChunkStorage::Large(ArrayChunkStorage::new_from(array))
            }
            #[cfg(feature = "nightly")]
            (16, 0) => {
                // SAFETY: We have already checked that the length is 16.
                let array: Box<[Section; 16]> =
                    unsafe { sections.into_boxed_slice().into_array().unwrap_unchecked() };
                ChunkStorage::Normal(ArrayChunkStorage::new_from(array))
            }
            #[cfg(not(feature = "nightly"))]
            (24, -64) => {
                // SAFETY: We have already checked that the length is 24.
                let array: [Section; 24] = unsafe { sections.try_into().unwrap_unchecked() };
                ChunkStorage::Large(ArrayChunkStorage::new_from(Box::new(array)))
            }
            #[cfg(not(feature = "nightly"))]
            (16, 0) => {
                // SAFETY: We have already checked that the length is 16.
                let array: [Section; 16] = unsafe { sections.try_into().unwrap_unchecked() };
                ChunkStorage::Normal(ArrayChunkStorage::new_from(Box::new(array)))
            }
            _ => ChunkStorage::Variable(VecChunkStorage::new(sections, offset)),
        }
    }
}

impl Deref for ChunkStorage {
    type Target = [Section];

    fn deref(&self) -> &Self::Target { self.as_slice() }
}

impl Default for ChunkStorage {
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
pub struct ArrayChunkStorage<const SECTIONS: usize, const OFFSET: i32>(Box<[Section; SECTIONS]>);

impl<const SECTIONS: usize, const OFFSET: i32> ArrayChunkStorage<SECTIONS, OFFSET> {
    /// Create a new [`ArrayChunkStorage`] from the given [`Section`]s.
    #[must_use]
    pub fn new(sections: [Section; SECTIONS]) -> Self { Self::new_from(Box::new(sections)) }

    /// Create a new [`ArrayChunkStorage`] from the given boxed [`Section`]s.
    #[must_use]
    pub const fn new_from(sections: Box<[Section; SECTIONS]>) -> Self { Self(sections) }

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

impl<const SECTIONS: usize, const OFFSET: i32> Deref for ArrayChunkStorage<SECTIONS, OFFSET> {
    type Target = [Section; SECTIONS];

    fn deref(&self) -> &Self::Target { &self.0 }
}

// -------------------------------------------------------------------------------------------------

/// A vertical slice of the world.
///
/// Has a variable number of sections and a known offset.
#[derive(Clone)]
pub struct VecChunkStorage(Vec<Section>, i32);

impl VecChunkStorage {
    /// Create a new [`VecChunkStorage`] from the given [`Section`]s and
    /// offset.
    #[must_use]
    pub const fn new(sections: Vec<Section>, offset: i32) -> Self { Self(sections, offset) }

    /// Get the vertical offset of the storage.
    #[must_use]
    pub const fn offset(&self) -> i32 { self.1 }

    /// Get the number of sections in the storage.
    #[must_use]
    pub const fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the storage contains no sections.
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.0.is_empty() }
}

impl Deref for VecChunkStorage {
    type Target = Vec<Section>;

    fn deref(&self) -> &Self::Target { &self.0 }
}
