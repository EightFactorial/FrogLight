use alloc::string::String;
use core::fmt;

use crate::types::indexed::{
    core::IndexCore,
    entry::EntryIndex,
    reference::{IndexedReference, ValueReference},
};

/// A reference to an SNBT entry.
pub struct EntryReference<'data, C: IndexCore> {
    name: IndexedReference<'data, String>,
    value: ValueReference<'data, C>,
}

impl<'data, C: IndexCore> EntryReference<'data, C> {
    /// Create a new [`EntryReference`] with the given index and core.
    ///
    /// # Safety
    ///
    /// The caller must ensure the [`EntryIndex`] is valid for the given core.
    #[inline]
    #[must_use]
    pub unsafe fn new(entry: EntryIndex, core: &'data C) -> Self {
        // SAFETY: The caller ensures that this is safe.
        unsafe {
            Self {
                name: IndexedReference::new(core.root(), entry.name()),
                value: ValueReference::new(core, entry.value()),
            }
        }
    }

    /// Get a reference to the name of this entry.
    #[inline]
    #[must_use]
    pub fn name(&self) -> IndexedReference<'data, String> { self.name }

    /// Get a reference to the value of this entry.
    #[inline]
    #[must_use]
    pub fn value(&self) -> ValueReference<'data, C> { self.value }
}

// -------------------------------------------------------------------------------------------------

impl<C: IndexCore> fmt::Debug for EntryReference<'_, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EntryReference")
            .field("name", &self.name)
            .field("value", &self.value)
            .finish()
    }
}

impl<C: IndexCore> Clone for EntryReference<'_, C> {
    fn clone(&self) -> Self { *self }
}
impl<C: IndexCore> Copy for EntryReference<'_, C> {}

impl<C: IndexCore> PartialEq for EntryReference<'_, C> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name() && self.value() == other.value()
    }
}
impl<C: IndexCore> Eq for EntryReference<'_, C> {}
