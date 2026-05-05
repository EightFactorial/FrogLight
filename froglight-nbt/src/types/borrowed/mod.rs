//! TODO

use core::fmt;

use ::alloc::vec::Vec;
use ::core::range::Range;
use froglight_mutf8::prelude::MStr;

pub mod compound;
use compound::{IndexedCompoundMut, IndexedCompoundRef, IndexedEntry};

pub mod reference;
use reference::{BorrowedIndex, BorrowedRef};

pub mod list;
pub mod value;

mod parse;

/// A borrowed NBT structure with an index of its contents.
#[derive(Clone, PartialEq, Eq)]
pub struct IndexedNbtRef<'data> {
    root: &'data [u8],
    name: Option<BorrowedIndex<MStr>>,
    core: IndexedCore,
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
            Some(unsafe { BorrowedRef::new(self.root, index) })
        } else {
            None
        }
    }

    /// Get the root compound of this NBT structure.
    #[inline]
    #[must_use]
    pub const fn as_compound(&self) -> IndexedCompoundRef<'_> {
        // SAFETY: `IndexedNbtRef` ensures this is valid.
        unsafe { IndexedCompoundRef::new(self.root, &self.core, 0) }
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
        entries: Vec<IndexedEntry>,
        indexes: Vec<Range<usize>>,
    ) -> Self {
        // SAFETY: The caller ensured this is safe.
        Self { root, name, core: unsafe { IndexedCore::new(entries, indexes) } }
    }
}

impl fmt::Debug for IndexedNbtRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name().map(|n| n.get_ref());
        let compound = self.as_compound();
        f.debug_struct("IndexedNbtRef").field("name", &name).field("root", &compound).finish()
    }
}

// -------------------------------------------------------------------------------------------------

/// A mutable, borrowed NBT structure with an index of its contents.
#[derive(PartialEq, Eq)]
pub struct IndexedNbtMut<'data> {
    root: &'data mut [u8],
    name: Option<BorrowedIndex<MStr>>,
    core: IndexedCore,
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
            Some(unsafe { BorrowedRef::new(self.root, index) })
        } else {
            None
        }
    }

    /// Get the root compound of this NBT structure.
    #[inline]
    #[must_use]
    pub const fn as_compound(&self) -> IndexedCompoundRef<'_> {
        // SAFETY: `IndexedNbtMut` ensures this is valid.
        unsafe { IndexedCompoundRef::new(self.root, &self.core, 0) }
    }

    /// Get the root compound of this NBT structure.
    #[inline]
    #[must_use]
    pub const fn as_compound_mut(&mut self) -> IndexedCompoundMut<'_> {
        // SAFETY: `IndexedNbtMut` ensures this is valid.
        unsafe { IndexedCompoundMut::new(self.root, &self.core, 0) }
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
        entries: Vec<IndexedEntry>,
        indexes: Vec<Range<usize>>,
    ) -> Self {
        // SAFETY: The caller ensured this is safe.
        Self { root, name, core: unsafe { IndexedCore::new(entries, indexes) } }
    }
}

impl fmt::Debug for IndexedNbtMut<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name().map(|n| n.get_ref());
        let compound = self.as_compound();
        f.debug_struct("IndexedNbtMut").field("name", &name).field("root", &compound).finish()
    }
}

// -------------------------------------------------------------------------------------------------

/// The core of an indexed NBT..
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "facet", derive(facet::Facet), facet(opaque))]
pub(super) struct IndexedCore {
    entries: Vec<IndexedEntry>,
    indexes: Vec<Range<usize>>,
}

impl IndexedCore {
    /// Create a new [`IndexedCoreRef`] from the given root and entries.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the given list of entries and indexes
    /// is valid for the given root slice.
    #[inline]
    #[must_use]
    pub(super) const unsafe fn new(entries: Vec<IndexedEntry>, indexes: Vec<Range<usize>>) -> Self {
        Self { entries, indexes }
    }

    /// Get the core list of [`IndexedEntry`]s.
    #[inline]
    #[must_use]
    pub(super) const fn entries(&self) -> &[IndexedEntry] { self.entries.as_slice() }

    /// Get the core list of index ranges.
    #[inline]
    #[must_use]
    pub(super) const fn indexes(&self) -> &[Range<usize>] { self.indexes.as_slice() }
}
