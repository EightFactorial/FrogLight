use crate::types::indexed::{
    compound::IndexedCompound,
    core::{IndexCore, NbtAccess, Ref},
    entry::{IndexedListType, IndexedMapType},
    list::{IndexedList, IndexedValueList},
    reference::IndexableValue,
};

/// An iterator over the entries in an [`IndexedList`].
pub struct ListIter<'iter, 'data, T: ?Sized, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> {
    list: &'iter IndexedList<'data, T, A, C>,
    index: usize,
}

impl<'iter, 'data, T: ?Sized, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>>
    ListIter<'iter, 'data, T, A, C>
{
    /// Create a new [`ListIter`] from the given [`IndexedList`].
    #[inline]
    #[must_use]
    pub const fn new(list: &'iter IndexedList<'data, T, A, C>) -> Self {
        ListIter { list, index: 0 }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'iter, T: IndexableValue + ?Sized, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> Iterator
    for ListIter<'iter, '_, T, A, C>
{
    type Item = T::Value<'iter>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.list.get(self.index)?;
        self.index += 1;

        Some(entry)
    }
}

impl<'iter, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> Iterator
    for ListIter<'iter, '_, IndexedMapType, A, C>
{
    type Item = IndexedCompound<'iter, Ref, C>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.list.get(self.index)?;
        self.index += 1;

        Some(entry)
    }
}

impl<'iter, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> Iterator
    for ListIter<'iter, '_, IndexedListType, A, C>
{
    type Item = IndexedValueList<'iter, Ref, C>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.list.get(self.index)?;
        self.index += 1;

        Some(entry)
    }
}

// -------------------------------------------------------------------------------------------------

impl<'iter, 'data, T: ?Sized, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> IntoIterator
    for &'iter IndexedList<'data, T, A, C>
where
    ListIter<'iter, 'data, T, A, C>: Iterator,
{
    type IntoIter = ListIter<'iter, 'data, T, A, C>;
    type Item = <Self::IntoIter as Iterator>::Item;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { ListIter::new(self) }
}

impl<'iter, 'data, T: IndexableValue + ?Sized, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>>
    IntoIterator for &'iter mut IndexedList<'data, T, A, C>
where
    ListIter<'iter, 'data, T, A, C>: Iterator,
{
    type IntoIter = ListIter<'iter, 'data, T, A, C>;
    type Item = <Self::IntoIter as Iterator>::Item;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { ListIter::new(self) }
}
