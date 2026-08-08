#![expect(clippy::into_iter_without_iter, reason = "Ignored")]

use crate::types::indexed::{
    compound::IndexedCompound, core::IndexCore, reference::EntryReference,
};

/// An iterator over an [`IndexedCompound`].
pub struct CompoundIter<'data, C: IndexCore> {
    compound: IndexedCompound<'data, C>,
    index: usize,
}

impl<'data, C: IndexCore> CompoundIter<'data, C> {
    /// Create a new [`CompoundIter`] over the given compound.
    #[inline]
    #[must_use]
    pub const fn new(compound: IndexedCompound<'data, C>) -> Self { Self { compound, index: 0 } }
}

// -------------------------------------------------------------------------------------------------

impl<'data, C: IndexCore> Iterator for CompoundIter<'data, C> {
    type Item = EntryReference<'data, C>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.compound.get_index(self.index)?;
        self.index += 1;
        Some(entry)
    }
}
impl<C: IndexCore> ExactSizeIterator for CompoundIter<'_, C> {
    #[inline]
    fn len(&self) -> usize { self.compound.entries().len() - self.index }
}

impl<'data, C: IndexCore> IntoIterator for IndexedCompound<'data, C> {
    type IntoIter = CompoundIter<'data, C>;
    type Item = EntryReference<'data, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { CompoundIter::new(self) }
}
impl<'data, C: IndexCore> IntoIterator for &IndexedCompound<'data, C> {
    type IntoIter = CompoundIter<'data, C>;
    type Item = EntryReference<'data, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { CompoundIter::new(*self) }
}
impl<'data, C: IndexCore> IntoIterator for &mut IndexedCompound<'data, C> {
    type IntoIter = CompoundIter<'data, C>;
    type Item = EntryReference<'data, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { CompoundIter::new(*self) }
}
