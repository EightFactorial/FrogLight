use alloc::string::String;
use core::fmt;

use crate::types::indexed::{
    core::IndexCore,
    entry::EntryIndex,
    reference::{IndexedReference, ValueReference},
};

/// A reference to an SNBT entry.
pub struct EntryReference<'index, C: IndexCore> {
    name: IndexedReference<'index, String>,
    value: ValueReference<'index, C>,
}

impl<'index, C: IndexCore> EntryReference<'index, C> {
    /// Create a new [`EntryReference`] with the given index and core.
    ///
    /// # Safety
    ///
    /// The caller must ensure the [`EntryIndex`] is valid for the given core.
    #[inline]
    #[must_use]
    pub unsafe fn new(core: &'index C, entry: EntryIndex) -> Self {
        // SAFETY: The caller ensures that this is safe.
        unsafe {
            Self {
                name: IndexedReference::new(core.root(), entry.name()),
                value: ValueReference::new(core, entry.value()),
            }
        }
    }

    /// Get the [`IndexedReference`] to the name of this entry.
    #[inline]
    #[must_use]
    pub const fn name(self) -> IndexedReference<'index, String> { self.name }

    /// Get the [`ValueReference`] to the value of this entry.
    #[inline]
    #[must_use]
    pub const fn value(self) -> ValueReference<'index, C> { self.value }

    /// Get the [`IndexedReference`] and [`ValueReference`] of this entry.
    #[inline]
    #[must_use]
    pub const fn pair(self) -> (IndexedReference<'index, String>, ValueReference<'index, C>) {
        (self.name(), self.value())
    }
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
