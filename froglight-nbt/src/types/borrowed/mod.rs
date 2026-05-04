//! TODO

use ::alloc::borrow::Cow;
use ::core::range::Range;
use froglight_mutf8::prelude::MStr;

pub mod compound;
use compound::{IndexedCompoundMut, IndexedCompoundRef, IndexedEntry};

pub mod core;
use core::{IndexedCoreMut, IndexedCoreRef};

pub mod reference;
use reference::{BorrowedIndex, BorrowedRef};

pub mod list;
pub mod value;

mod parse;

/// A borrowed NBT structure with an index of its contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedNbtRef<'data> {
    name: Option<BorrowedIndex<MStr>>,
    core: IndexedCoreRef<'data>,
}

impl<'data> IndexedNbtRef<'data> {
    /// Parse a named NBT structure from the given slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the slice does not contain valid NBT data.
    #[allow(clippy::result_unit_err, reason = "WIP")]
    pub fn new_named(data: &'data [u8]) -> Result<Self, ()> { parse::parse_nbt_ref(data, true) }

    /// Parse an unnamed NBT structure from the given slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the slice does not contain valid NBT data.
    #[allow(clippy::result_unit_err, reason = "WIP")]
    pub fn new_unnamed(data: &'data [u8]) -> Result<Self, ()> { parse::parse_nbt_ref(data, false) }

    /// Get the name of this NBT structure, if it has one.
    ///
    /// Returns `None` if this NBT structure is unnamed.
    #[must_use]
    pub const fn name(&self) -> Option<BorrowedRef<'data, MStr>> {
        if let Some(index) = self.name {
            // SAFETY: The entry uses the same `root` so the index is valid.
            Some(unsafe { BorrowedRef::new(self.core.root(), index) })
        } else {
            None
        }
    }

    /// Get the root compound of this NBT structure.
    #[inline]
    #[must_use]
    pub const fn as_compound(&self) -> IndexedCompoundRef<'_> {
        // SAFETY: `IndexedNbtRef` ensures this is valid.
        unsafe { IndexedCompoundRef::new(self.core.reborrow(), 0) }
    }

    /// Create a new [`IndexedNbtRef`] from the given root and entries.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - The name's index is valid for the given root (if it exists).
    /// - The list of entries is valid for the given root.
    /// - The list of indexes is valid for the list of entries.
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(
        root: &'data [u8],
        name: Option<BorrowedIndex<MStr>>,
        entries: Cow<'data, [IndexedEntry]>,
        indexes: Cow<'data, [Range<usize>]>,
    ) -> Self {
        // SAFETY: The caller ensured this is safe.
        Self { name, core: unsafe { IndexedCoreRef::new(root, entries, indexes) } }
    }
}

// -------------------------------------------------------------------------------------------------

/// A mutable, borrowed NBT structure with an index of its contents.
#[derive(Debug, PartialEq, Eq)]
pub struct IndexedNbtMut<'data> {
    name: Option<BorrowedIndex<MStr>>,
    core: IndexedCoreMut<'data>,
}

impl<'data> IndexedNbtMut<'data> {
    /// Parse a named NBT structure from the given slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the slice does not contain valid NBT data.
    #[allow(clippy::result_unit_err, reason = "WIP")]
    pub fn new_named(data: &'data mut [u8]) -> Result<Self, ()> { parse::parse_nbt_mut(data, true) }

    /// Parse an unnamed NBT structure from the given slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the slice does not contain valid NBT data.
    #[allow(clippy::result_unit_err, reason = "WIP")]
    pub fn new_unnamed(data: &'data mut [u8]) -> Result<Self, ()> {
        parse::parse_nbt_mut(data, false)
    }

    /// Get the name of this NBT structure, if it has one.
    ///
    /// Returns `None` if this NBT structure is unnamed.
    #[must_use]
    pub const fn name(&self) -> Option<BorrowedRef<'_, MStr>> {
        if let Some(index) = self.name {
            // SAFETY: The entry uses the same `root` so the index is valid.
            Some(unsafe { BorrowedRef::new(self.core.root(), index) })
        } else {
            None
        }
    }

    /// Get the root compound of this NBT structure.
    #[inline]
    #[must_use]
    pub const fn as_compound(&self) -> IndexedCompoundRef<'_> {
        // SAFETY: `IndexedNbtMut` ensures this is valid.
        unsafe { IndexedCompoundRef::new(self.core.as_ref(), 0) }
    }

    /// Get the root compound of this NBT structure.
    #[inline]
    #[must_use]
    pub const fn as_compound_mut(&mut self) -> IndexedCompoundMut<'_> {
        // SAFETY: `IndexedNbtMut` ensures this is valid.
        unsafe { IndexedCompoundMut::new(self.core.reborrow(), 0) }
    }

    /// Create a new [`IndexedNbtMut`] from the given root and entries.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - The name's index is valid for the given root (if it exists).
    /// - The list of entries is valid for the given root.
    /// - The list of indexes is valid for the list of entries.
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(
        root: &'data mut [u8],
        name: Option<BorrowedIndex<MStr>>,
        entries: Cow<'data, [IndexedEntry]>,
        indexes: Cow<'data, [Range<usize>]>,
    ) -> Self {
        // SAFETY: The caller ensured this is safe.
        Self { name, core: unsafe { IndexedCoreMut::new(root, entries, indexes) } }
    }
}
