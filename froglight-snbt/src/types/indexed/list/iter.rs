use crate::types::indexed::{
    core::IndexCore,
    index::{IndexableValue, numeric::IntegerValue},
    list::{IndexedList, IndexedSlice},
    reference::{Referenceable, ValueReference},
};

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

// -------------------------------------------------------------------------------------------------

/// An iterator over an [`IndexedSlice`].
pub struct SliceIter<'iter, 'data, C: IndexCore, T: Referenceable + 'static>
where
    T::Indexable: IndexableValue,
{
    slice: &'iter IndexedSlice<'data, C, T>,
    index: usize,
}

impl<'iter, 'data, C: IndexCore, T: Referenceable> SliceIter<'iter, 'data, C, T>
where
    T::Indexable: IndexableValue,
{
    /// Create a new [`SliceIter`] over the given slice.
    #[inline]
    #[must_use]
    pub const fn new(slice: &'iter IndexedSlice<'data, C, T>) -> Self { Self { slice, index: 0 } }
}

// -------------------------------------------------------------------------------------------------

impl<C: IndexCore, T: Referenceable + 'static> Iterator for SliceIter<'_, '_, C, T>
where
    T::Indexable: IndexableValue,
    IntegerValue: Into<T::Value<'static>>,
{
    type Item = T::Value<'static>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.slice.get(self.index);
        self.index += 1;
        value
    }
}
impl<C: IndexCore, T: Referenceable + 'static> ExactSizeIterator for SliceIter<'_, '_, C, T>
where
    T::Indexable: IndexableValue,
    IntegerValue: Into<T::Value<'static>>,
{
    #[inline]
    fn len(&self) -> usize { self.slice.entries().len() - self.index }
}

impl<'iter, 'data, C: IndexCore, T: Referenceable + 'static> IntoIterator
    for &'iter IndexedSlice<'data, C, T>
where
    T::Indexable: IndexableValue,
    IntegerValue: Into<T::Value<'static>>,
{
    type IntoIter = SliceIter<'iter, 'data, C, T>;
    type Item = T::Value<'static>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { SliceIter::new(self) }
}
#[expect(clippy::into_iter_without_iter, reason = "Incorrect")]
impl<'iter, 'data, C: IndexCore, T: Referenceable + 'static> IntoIterator
    for &'iter mut IndexedSlice<'data, C, T>
where
    T::Indexable: IndexableValue,
    IntegerValue: Into<T::Value<'static>>,
{
    type IntoIter = SliceIter<'iter, 'data, C, T>;
    type Item = T::Value<'static>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { SliceIter::new(self) }
}
