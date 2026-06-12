//! TODO
#![expect(clippy::result_unit_err, reason = "WIP")]

use alloc::string::String;

use ::core::range::Range;

pub mod compound;
pub mod core;
pub mod entry;
pub mod index;
pub mod list;
mod parse;
pub mod reference;
pub mod types;

/// An SNBT structure with indexed entries.
pub struct IndexedSnbt<C: core::IndexCore> {
    core: C,
}

impl<C: core::IndexCore> IndexedSnbt<C> {
    /// Create a new [`IndexedSnbt`] with the given
    /// [`IndexCore`](core::IndexCore).
    #[inline]
    #[must_use]
    pub const fn new(core: C) -> Self { Self { core } }

    /// Get the root compound of the SNBT structure.
    #[inline]
    #[must_use]
    pub fn root(&self) -> compound::IndexedCompound<'_, C> {
        let range = Range { start: 0, end: 1 };
        let index = unsafe { self.core.get_entries(range).get_unchecked(0) };
        debug_assert!(matches!(index.value(), entry::ValueIndex::Compound(..)));
        // SAFETY: The first entry is always the root compound.
        unsafe { compound::IndexedCompound::new(&self.core, index.value().range()) }
    }
}

impl<'data> IndexedSnbt<core::SliceCore<'data>> {
    /// Parse an SNBT structure from the given string.
    ///
    /// # Errors
    ///
    /// Returns an error if the string slice is not valid SNBT data.
    #[inline]
    pub fn new_ref(string: &'data str) -> Result<Self, ()> { parse::parse_snbt(string) }

    /// Take ownership of the SNBT data using a [`CowCore`](core::CowCore).
    #[inline]
    #[must_use]
    pub fn into_owned(self) -> IndexedSnbt<core::CowCore<'static>> {
        IndexedSnbt::new(core::CowCore::from_slice(self.core).into_owned())
    }
}

impl<'data> IndexedSnbt<core::CowCore<'data>> {
    /// Parse an SNBT structure from the given string.
    ///
    /// # Errors
    ///
    /// Returns an error if the string slice is not valid SNBT data.
    #[inline]
    pub fn new_owned_ref(string: &'data str) -> Result<Self, ()> {
        let borrowed = parse::parse_snbt(string)?;
        Ok(IndexedSnbt::new(core::CowCore::from_slice(borrowed.core)))
    }

    /// Parse an SNBT structure from the given string.
    ///
    /// # Errors
    ///
    /// Returns an error if the string slice is not valid SNBT data.
    #[inline]
    pub fn new_owned(string: String) -> Result<IndexedSnbt<core::CowCore<'static>>, ()> {
        let borrowed = parse::parse_snbt(&string)?;
        let entries = borrowed.core.entries;

        let root = alloc::borrow::Cow::Owned(string);
        Ok(IndexedSnbt::new(core::CowCore { root, entries }))
    }

    /// Take ownership of the SNBT data by cloning the root string.
    #[inline]
    #[must_use]
    pub fn into_owned(self) -> IndexedSnbt<core::CowCore<'static>> {
        IndexedSnbt::new(self.core.into_owned())
    }
}
