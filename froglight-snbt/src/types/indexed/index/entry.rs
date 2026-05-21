//! TODO

use crate::types::indexed::index::{Index, ValueIndex};

/// A pair of name and value [`Index`]es.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntryIndex {
    name: Index<str>,
    value: ValueIndex,
}

impl EntryIndex {
    /// Create a new [`EntryIndex`] with the given name and value [`Index`]es.
    #[inline]
    #[must_use]
    pub const fn new(name: Index<str>, value: ValueIndex) -> Self { Self { name, value } }

    /// Get the [`Index`] of the name of this entry.
    #[inline]
    #[must_use]
    pub const fn name(&self) -> Index<str> { self.name }

    /// Get the [`Index`] of the value of this entry.
    #[inline]
    #[must_use]
    pub const fn value(&self) -> ValueIndex { self.value }
}
