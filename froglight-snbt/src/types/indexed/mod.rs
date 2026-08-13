//! TODO
#![expect(clippy::result_unit_err, reason = "WIP")]

use alloc::string::String;

use ::core::{fmt, range::Range};

use crate::types::indexed::core::SliceCore;

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

    /// Get the root [`IndexedCompound`](compound::IndexedCompound) of the SNBT
    /// structure.
    #[inline]
    #[must_use]
    pub fn as_compound(&self) -> compound::IndexedCompound<'_, C> {
        let range = Range { start: 0, end: 1 };
        let index = unsafe { self.core.get_entries(range).get_unchecked(0) };
        debug_assert!(matches!(index.value(), entry::ValueIndex::Compound(..)));

        // SAFETY: The first entry is always the root compound.
        unsafe { compound::IndexedCompound::new(&self.core, index.value().range()) }
    }

    /// Get the root [`IndexedCompound`](compound::IndexedCompound) of the SNBT
    /// structure as a [`ValueReference`](reference::ValueReference).
    #[inline]
    #[must_use]
    pub fn as_value(&self) -> reference::ValueReference<'_, C> {
        reference::ValueReference::Compound(self.as_compound())
    }

    /// Get the root SNBT string.
    #[inline]
    #[must_use]
    pub fn as_root(&self) -> &str { self.core.root() }

    /// Get the root SNBT string.
    ///
    /// With certain [`IndexCore`](core::IndexCore)s (notably [`SliceCore`]),
    /// this may allow for longer borrows.
    ///
    /// See [`IndexedReference::upgrade`](reference::IndexedReference::upgrade).
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> C::RootLong<'_> { self.core.root_long() }
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
    pub fn new_owned_ref(string: &'data str) -> Result<Self, ()> {
        let borrowed = parse::parse_snbt(string)?;
        Ok(IndexedSnbt::new(core::CowCore::from_slice(borrowed.core)))
    }

    /// Parse an SNBT structure from the given string.
    ///
    /// # Errors
    ///
    /// Returns an error if the string slice is not valid SNBT data.
    pub fn new_owned(string: String) -> Result<IndexedSnbt<core::CowCore<'static>>, ()> {
        let borrowed = parse::parse_snbt(&string)?;
        let entries = borrowed.core.entries;

        let root = alloc::borrow::Cow::Owned(string);
        Ok(IndexedSnbt::new(core::CowCore { root, entries }))
    }

    /// Access this [`CowCore`]-based SNBT as [`SliceCore`]-based SNBT in the
    /// provided closure.
    pub fn as_scoped_slice<R>(
        mut self,
        f: impl FnOnce(&IndexedSnbt<SliceCore<'_>>) -> R,
    ) -> (Self, R) {
        let (core, result) = self.core.as_slice_for::<R>(f);
        self.core = core;
        (self, result)
    }

    /// Take ownership of the SNBT data by cloning the root string.
    #[inline]
    #[must_use]
    pub fn into_owned(self) -> IndexedSnbt<core::CowCore<'static>> {
        IndexedSnbt::new(self.core.into_owned())
    }
}

impl<C: core::IndexCore> fmt::Debug for IndexedSnbt<C> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_compound(), f)
    }
}
