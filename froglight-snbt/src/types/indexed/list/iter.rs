#![expect(clippy::into_iter_without_iter, reason = "Ignored")]

use crate::types::indexed::{
    core::IndexCore,
    index::{IndexableValue, numeric::IntegerValue},
    list::{IndexedList, IndexedSlice},
    reference::{Referenceable, ValueReference},
};

/// An iterator over an [`IndexedList`].
pub struct ListIter<'index, C: IndexCore> {
    list: IndexedList<'index, C>,
    index: usize,
}

impl<'index, C: IndexCore> ListIter<'index, C> {
    /// Create a new [`ListIter`] over the given list.
    #[inline]
    #[must_use]
    pub const fn new(list: IndexedList<'index, C>) -> Self { Self { list, index: 0 } }
}

// -------------------------------------------------------------------------------------------------

impl<'index, C: IndexCore> Iterator for ListIter<'index, C> {
    type Item = ValueReference<'index, C>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.list.get(self.index);
        self.index += 1;
        value
    }
}
impl<C: IndexCore> ExactSizeIterator for ListIter<'_, C> {
    #[inline]
    fn len(&self) -> usize { self.list.entries().len() - self.index }
}

impl<'index, C: IndexCore> IntoIterator for IndexedList<'index, C> {
    type IntoIter = ListIter<'index, C>;
    type Item = ValueReference<'index, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { ListIter::new(self) }
}
impl<'index, C: IndexCore> IntoIterator for &IndexedList<'index, C> {
    type IntoIter = ListIter<'index, C>;
    type Item = ValueReference<'index, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { ListIter::new(*self) }
}
impl<'index, C: IndexCore> IntoIterator for &mut IndexedList<'index, C> {
    type IntoIter = ListIter<'index, C>;
    type Item = ValueReference<'index, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { ListIter::new(*self) }
}

// -------------------------------------------------------------------------------------------------

/// An iterator over an [`IndexedSlice`].
pub struct SliceIter<'index, C: IndexCore, T: Referenceable + 'index>
where
    T::Indexable: IndexableValue,
{
    slice: IndexedSlice<'index, C, T>,
    index: usize,
}

impl<'index, C: IndexCore, T: Referenceable + 'index> SliceIter<'index, C, T>
where
    T::Indexable: IndexableValue,
{
    /// Create a new [`SliceIter`] over the given slice.
    #[inline]
    #[must_use]
    pub const fn new(slice: IndexedSlice<'index, C, T>) -> Self { Self { slice, index: 0 } }
}

// -------------------------------------------------------------------------------------------------

impl<'index, C: IndexCore, T: Referenceable + 'index> Iterator for SliceIter<'index, C, T>
where
    T::Indexable: IndexableValue,
    IntegerValue: Into<T::Value<'index>>,
{
    type Item = T::Value<'index>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.slice.get(self.index);
        self.index += 1;
        value
    }
}
impl<'index, C: IndexCore, T: Referenceable + 'index> ExactSizeIterator for SliceIter<'index, C, T>
where
    T::Indexable: IndexableValue,
    IntegerValue: Into<T::Value<'index>>,
{
    #[inline]
    fn len(&self) -> usize { self.slice.entries().len() - self.index }
}

impl<'index, C: IndexCore, T: Referenceable + 'index> IntoIterator for IndexedSlice<'index, C, T>
where
    T::Indexable: IndexableValue,
    IntegerValue: Into<T::Value<'index>>,
{
    type IntoIter = SliceIter<'index, C, T>;
    type Item = T::Value<'index>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { SliceIter::new(self) }
}
impl<'index, C: IndexCore, T: Referenceable + 'index> IntoIterator for &IndexedSlice<'index, C, T>
where
    T::Indexable: IndexableValue,
    IntegerValue: Into<T::Value<'index>>,
{
    type IntoIter = SliceIter<'index, C, T>;
    type Item = T::Value<'index>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { SliceIter::new(*self) }
}
impl<'index, C: IndexCore, T: Referenceable + 'index> IntoIterator
    for &mut IndexedSlice<'index, C, T>
where
    T::Indexable: IndexableValue,
    IntegerValue: Into<T::Value<'index>>,
{
    type IntoIter = SliceIter<'index, C, T>;
    type Item = T::Value<'index>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { SliceIter::new(*self) }
}
