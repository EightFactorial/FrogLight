use core::ops::Deref;

use foldhash::fast::RandomState;
use froglight_common::identifier::Identifier;
use froglight_nbt::types::indexed::{
    alloc::{CowCore, IndexedNbtCow},
    compound::IndexedCompound,
    core::Ref,
};
use indexmap::IndexMap;

/// A reference to a [`Nbt`] and it's associated values.
#[derive(Debug, Clone)]
pub struct NbtRef<'a> {
    identifier: Identifier<'a>,
    values: &'a IndexMap<Identifier<'static>, IndexedNbtCow<'static>, RandomState>,
}

impl<'a> NbtRef<'a> {
    /// Create a new [`NbtRef`].
    #[inline]
    #[must_use]
    pub const fn new(
        identifier: Identifier<'a>,
        values: &'a IndexMap<Identifier<'static>, IndexedNbtCow<'static>, RandomState>,
    ) -> Self {
        Self { identifier, values }
    }

    /// Get the [`Identifier`] of this [`NbtRef`].
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'a> { &self.identifier }

    /// Returns the number of values in this [`NbtRef`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.values.len() }

    /// Returns `true` if this [`NbtRef`] contains no values.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.values.is_empty() }

    /// Get a [`NbtValueRef`] by it's [`Identifier`].
    #[inline]
    #[must_use]
    pub fn get_by_identifier(&self, identifier: &str) -> Option<NbtValueRef<'a>> {
        self.values.get_key_value(identifier).map(|(identifier, values)| {
            NbtValueRef::new(identifier.reborrow(), values.as_compound())
        })
    }

    /// Get a [`NbtValueRef`] by it's index.
    #[inline]
    #[must_use]
    pub fn get_by_index(&self, index: usize) -> Option<NbtValueRef<'a>> {
        self.values.get_index(index).map(|(identifier, values)| {
            NbtValueRef::new(identifier.reborrow(), values.as_compound())
        })
    }
}

// -------------------------------------------------------------------------------------------------

/// An [`Identifier`] and it's associated values.
#[derive(Debug, Clone)]
pub struct NbtValueRef<'a> {
    identifier: Identifier<'a>,
    values: IndexedCompound<'a, Ref, CowCore<'static>>,
}

impl<'a> NbtValueRef<'a> {
    /// Create a new [`NbtValueRef`].
    #[inline]
    #[must_use]
    pub const fn new(
        identifier: Identifier<'a>,
        values: IndexedCompound<'a, Ref, CowCore<'static>>,
    ) -> Self {
        Self { identifier, values }
    }

    /// Get the [`Identifier`] of this [`NbtValue`].
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'a> { &self.identifier }
}

impl<'a> Deref for NbtValueRef<'a> {
    type Target = IndexedCompound<'a, Ref, CowCore<'static>>;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.values }
}
