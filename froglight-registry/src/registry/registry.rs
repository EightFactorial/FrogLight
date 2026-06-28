use alloc::vec::Vec;

use foldhash::fast::RandomState;
use froglight_common::identifier::Identifier;
use indexmap::IndexMap;

/// A reference to a [`Registry`] and it's associated values.
#[derive(Debug, Clone)]
pub struct RegistryRef<'a> {
    identifier: Identifier<'a>,
    values: &'a IndexMap<Identifier<'static>, Vec<u32>, RandomState>,
}

impl<'a> RegistryRef<'a> {
    /// Create a new [`RegistryRef`].
    #[inline]
    #[must_use]
    pub const fn new(
        identifier: Identifier<'a>,
        values: &'a IndexMap<Identifier<'static>, Vec<u32>, RandomState>,
    ) -> Self {
        Self { identifier, values }
    }

    /// Get the [`Identifier`] of this [`RegistryRef`].
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'a> { &self.identifier }

    /// Returns the number of values in this [`RegistryRef`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.values.len() }

    /// Returns `true` if this [`RegistryRef`] contains no values.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.values.is_empty() }

    /// Get a [`RegistryValueRef`] by it's [`Identifier`].
    #[inline]
    #[must_use]
    pub fn get_by_identifier(&self, identifier: &str) -> Option<RegistryValueRef<'a>> {
        self.values.get_key_value(identifier).map(|(identifier, values)| {
            RegistryValueRef::new(identifier.reborrow(), values.as_slice())
        })
    }

    /// Get a [`RegistryValueRef`] by it's index.
    #[inline]
    #[must_use]
    pub fn get_by_index(&self, index: usize) -> Option<RegistryValueRef<'a>> {
        self.values.get_index(index).map(|(identifier, values)| {
            RegistryValueRef::new(identifier.reborrow(), values.as_slice())
        })
    }
}

// -------------------------------------------------------------------------------------------------

/// An [`Identifier`] and it's associated values.
#[derive(Debug, Clone)]
pub struct RegistryValueRef<'a> {
    identifier: Identifier<'a>,
    values: &'a [u32],
}

impl<'a> RegistryValueRef<'a> {
    /// Create a new [`RegistryValueRef`].
    #[inline]
    #[must_use]
    pub const fn new(identifier: Identifier<'a>, values: &'a [u32]) -> Self {
        Self { identifier, values }
    }

    /// Get the [`Identifier`] of this [`RegistryValue`].
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'a> { &self.identifier }

    /// Get the [`u32`] values of this [`RegistryValue`].
    #[inline]
    #[must_use]
    pub const fn values(&self) -> &'a [u32] { self.values }
}
