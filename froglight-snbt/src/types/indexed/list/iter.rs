#![expect(clippy::into_iter_without_iter, reason = "Ignored")]

use crate::types::indexed::{
    core::IndexCore,
    index::{IndexableValue, numeric::IntegerValue},
    list::{IndexedList, IndexedSlice},
    reference::{Referenceable, ValueReference},
};

/// An iterator over an [`IndexedList`].
pub struct ListIter<'data, C: IndexCore> {
    list: IndexedList<'data, C>,
    index: usize,
}

impl<'data, C: IndexCore> ListIter<'data, C> {
    /// Create a new [`ListIter`] over the given list.
    #[inline]
    #[must_use]
    pub const fn new(list: IndexedList<'data, C>) -> Self { Self { list, index: 0 } }
}

// -------------------------------------------------------------------------------------------------

impl<'data, C: IndexCore> Iterator for ListIter<'data, C> {
    type Item = ValueReference<'data, C>;

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

impl<'data, C: IndexCore> IntoIterator for IndexedList<'data, C> {
    type IntoIter = ListIter<'data, C>;
    type Item = ValueReference<'data, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { ListIter::new(self) }
}
impl<'data, C: IndexCore> IntoIterator for &IndexedList<'data, C> {
    type IntoIter = ListIter<'data, C>;
    type Item = ValueReference<'data, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { ListIter::new(*self) }
}
impl<'data, C: IndexCore> IntoIterator for &mut IndexedList<'data, C> {
    type IntoIter = ListIter<'data, C>;
    type Item = ValueReference<'data, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { ListIter::new(*self) }
}

// -------------------------------------------------------------------------------------------------

/// An iterator over an [`IndexedSlice`].
pub struct SliceIter<'data, C: IndexCore, T: Referenceable + 'data>
where
    T::Indexable: IndexableValue,
{
    slice: IndexedSlice<'data, C, T>,
    index: usize,
}

impl<'data, C: IndexCore, T: Referenceable + 'data> SliceIter<'data, C, T>
where
    T::Indexable: IndexableValue,
{
    /// Create a new [`SliceIter`] over the given slice.
    #[inline]
    #[must_use]
    pub const fn new(slice: IndexedSlice<'data, C, T>) -> Self { Self { slice, index: 0 } }
}

// -------------------------------------------------------------------------------------------------

impl<'data, C: IndexCore, T: Referenceable + 'data> Iterator for SliceIter<'data, C, T>
where
    T::Indexable: IndexableValue,
    IntegerValue: Into<T::Value<'data>>,
{
    type Item = T::Value<'data>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.slice.get(self.index);
        self.index += 1;
        value
    }
}
impl<'data, C: IndexCore, T: Referenceable + 'data> ExactSizeIterator for SliceIter<'data, C, T>
where
    T::Indexable: IndexableValue,
    IntegerValue: Into<T::Value<'data>>,
{
    #[inline]
    fn len(&self) -> usize { self.slice.entries().len() - self.index }
}

impl<'data, C: IndexCore, T: Referenceable + 'data> IntoIterator for IndexedSlice<'data, C, T>
where
    T::Indexable: IndexableValue,
    IntegerValue: Into<T::Value<'data>>,
{
    type IntoIter = SliceIter<'data, C, T>;
    type Item = T::Value<'data>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { SliceIter::new(self) }
}
impl<'data, C: IndexCore, T: Referenceable + 'data> IntoIterator for &IndexedSlice<'data, C, T>
where
    T::Indexable: IndexableValue,
    IntegerValue: Into<T::Value<'data>>,
{
    type IntoIter = SliceIter<'data, C, T>;
    type Item = T::Value<'data>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { SliceIter::new(*self) }
}
impl<'data, C: IndexCore, T: Referenceable + 'data> IntoIterator for &mut IndexedSlice<'data, C, T>
where
    T::Indexable: IndexableValue,
    IntegerValue: Into<T::Value<'data>>,
{
    type IntoIter = SliceIter<'data, C, T>;
    type Item = T::Value<'data>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { SliceIter::new(*self) }
}
