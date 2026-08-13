#![expect(clippy::into_iter_without_iter, reason = "Ignored")]

use crate::types::indexed::{
    compound::IndexedCompound, core::IndexCore, reference::EntryReference,
};

/// An iterator over an [`IndexedCompound`].
pub struct CompoundIter<'index, C: IndexCore> {
    compound: IndexedCompound<'index, C>,
    index: usize,
}

impl<'index, C: IndexCore> CompoundIter<'index, C> {
    /// Create a new [`CompoundIter`] over the given compound.
    #[inline]
    #[must_use]
    pub const fn new(compound: IndexedCompound<'index, C>) -> Self { Self { compound, index: 0 } }
}

// -------------------------------------------------------------------------------------------------

impl<'index, C: IndexCore> Iterator for CompoundIter<'index, C> {
    type Item = EntryReference<'index, C>;

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

impl<'index, C: IndexCore> IntoIterator for IndexedCompound<'index, C> {
    type IntoIter = CompoundIter<'index, C>;
    type Item = EntryReference<'index, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { CompoundIter::new(self) }
}
impl<'index, C: IndexCore> IntoIterator for &IndexedCompound<'index, C> {
    type IntoIter = CompoundIter<'index, C>;
    type Item = EntryReference<'index, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { CompoundIter::new(*self) }
}
impl<'index, C: IndexCore> IntoIterator for &mut IndexedCompound<'index, C> {
    type IntoIter = CompoundIter<'index, C>;
    type Item = EntryReference<'index, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { CompoundIter::new(*self) }
}
