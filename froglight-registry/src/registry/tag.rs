use alloc::vec::Vec;

use foldhash::fast::RandomState;
use froglight_common::identifier::Identifier;
use indexmap::IndexMap;

/// A reference to a [`Tag`] and it's associated values.
#[derive(Debug, Clone)]
pub struct TagRef<'a> {
    identifier: Identifier<'a>,
    values: &'a IndexMap<Identifier<'static>, Vec<u32>, RandomState>,
}

impl<'a> TagRef<'a> {
    /// Create a new [`TagRef`].
    #[inline]
    #[must_use]
    pub const fn new(
        identifier: Identifier<'a>,
        values: &'a IndexMap<Identifier<'static>, Vec<u32>, RandomState>,
    ) -> Self {
        Self { identifier, values }
    }

    /// Get the [`Identifier`] of this [`TagRef`].
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'a> { &self.identifier }

    /// Returns the number of values in this [`TagRef`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.values.len() }

    /// Returns `true` if this [`TagRef`] contains no values.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.values.is_empty() }

    /// Get a [`TagValueRef`] by it's [`Identifier`].
    #[inline]
    #[must_use]
    pub fn get_by_identifier(&self, identifier: &str) -> Option<TagValueRef<'a>> {
        self.values
            .get_key_value(identifier)
            .map(|(identifier, values)| TagValueRef::new(identifier.reborrow(), values.as_slice()))
    }

    /// Get a [`TagValueRef`] by it's index.
    #[inline]
    #[must_use]
    pub fn get_by_index(&self, index: usize) -> Option<TagValueRef<'a>> {
        self.values
            .get_index(index)
            .map(|(identifier, values)| TagValueRef::new(identifier.reborrow(), values.as_slice()))
    }
}

// -------------------------------------------------------------------------------------------------

/// An [`Identifier`] and it's associated values.
#[derive(Debug, Clone)]
pub struct TagValueRef<'a> {
    identifier: Identifier<'a>,
    values: &'a [u32],
}

impl<'a> TagValueRef<'a> {
    /// Create a new [`TagValueRef`].
    #[inline]
    #[must_use]
    pub const fn new(identifier: Identifier<'a>, values: &'a [u32]) -> Self {
        Self { identifier, values }
    }

    /// Get the [`Identifier`] of this [`TagValue`].
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'a> { &self.identifier }

    /// Get the [`u32`] values of this [`TagValue`].
    #[inline]
    #[must_use]
    pub const fn values(&self) -> &'a [u32] { self.values }
}
