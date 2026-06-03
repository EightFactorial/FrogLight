use crate::types::indexed::{core::IndexCore, list::IndexedList, reference::ValueReference};

/// An iterator over an [`IndexedList`].
pub struct ListIter<'iter, 'data, C: IndexCore> {
    list: &'iter IndexedList<'data, C>,
    index: usize,
}

impl<'iter, 'data, C: IndexCore> ListIter<'iter, 'data, C> {
    /// Create a new [`ListIter`] over the given list.
    #[inline]
    #[must_use]
    pub const fn new(list: &'iter IndexedList<'data, C>) -> Self { Self { list, index: 0 } }
}

// -------------------------------------------------------------------------------------------------

impl<'data, C: IndexCore> Iterator for ListIter<'_, 'data, C> {
    type Item = ValueReference<'data, C>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.list.get(self.index);
        self.index += 1;
        value
    }
}
impl<C: IndexCore> ExactSizeIterator for ListIter<'_, '_, C> {
    #[inline]
    fn len(&self) -> usize { self.list.entries().len() - self.index }
}

impl<'iter, 'data, C: IndexCore> IntoIterator for &'iter IndexedList<'data, C> {
    type IntoIter = ListIter<'iter, 'data, C>;
    type Item = ValueReference<'data, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { ListIter::new(self) }
}
#[expect(clippy::into_iter_without_iter, reason = "Incorrect")]
impl<'iter, 'data, C: IndexCore> IntoIterator for &'iter mut IndexedList<'data, C> {
    type IntoIter = ListIter<'iter, 'data, C>;
    type Item = ValueReference<'data, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { ListIter::new(self) }
}
