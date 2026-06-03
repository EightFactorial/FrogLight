use crate::types::indexed::{
    compound::IndexedCompound, core::IndexCore, reference::EntryReference,
};

/// An iterator over an [`IndexedCompound`].
pub struct CompoundIter<'iter, 'data, C: IndexCore> {
    compound: &'iter IndexedCompound<'data, C>,
    index: usize,
}

impl<'iter, 'data, C: IndexCore> CompoundIter<'iter, 'data, C> {
    /// Create a new [`CompoundIter`] over the given compound.
    #[inline]
    #[must_use]
    pub const fn new(compound: &'iter IndexedCompound<'data, C>) -> Self {
        Self { compound, index: 0 }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'data, C: IndexCore> Iterator for CompoundIter<'_, 'data, C> {
    type Item = EntryReference<'data, C>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.compound.get_index(self.index)?;
        self.index += 1;
        Some(entry)
    }
}
impl<C: IndexCore> ExactSizeIterator for CompoundIter<'_, '_, C> {
    #[inline]
    fn len(&self) -> usize { self.compound.entries().len() - self.index }
}

impl<'iter, 'data, C: IndexCore> IntoIterator for &'iter IndexedCompound<'data, C> {
    type IntoIter = CompoundIter<'iter, 'data, C>;
    type Item = EntryReference<'data, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { CompoundIter::new(self) }
}
#[expect(clippy::into_iter_without_iter, reason = "Incorrect")]
impl<'iter, 'data, C: IndexCore> IntoIterator for &'iter mut IndexedCompound<'data, C> {
    type IntoIter = CompoundIter<'iter, 'data, C>;
    type Item = EntryReference<'data, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { CompoundIter::new(self) }
}
