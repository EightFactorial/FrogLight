use crate::types::indexed::{
    compound::IndexedCompound,
    core::{IndexCore, NbtAccess, Ref},
    list::{IndexedList, IndexedValueList},
    reference::IndexableValue,
    types::{IndexedListType, IndexedMapType},
};

/// An iterator over the entries in an [`IndexedList`].
pub struct ListOwnedIter<'data, T: ?Sized, A: NbtAccess, C: IndexCore<A>>
where
    <A as NbtAccess>::CORE<'data, C>: Clone,
{
    list: IndexedList<'data, T, A, C>,
    index: usize,
}

impl<'data, T: ?Sized, A: NbtAccess, C: IndexCore<A>> ListOwnedIter<'data, T, A, C>
where
    <A as NbtAccess>::CORE<'data, C>: Clone,
{
    /// Create a new [`ListIter`] from the given [`IndexedList`].
    #[inline]
    #[must_use]
    pub const fn new(list: IndexedList<'data, T, A, C>) -> Self { ListOwnedIter { list, index: 0 } }
}

impl<'data, T: IndexableValue + ?Sized, A: NbtAccess, C: IndexCore<A>> Iterator
    for ListOwnedIter<'data, T, A, C>
where
    <A as NbtAccess>::CORE<'data, C>: Clone,
{
    type Item = T::Value<'data>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.list.clone().get(self.index)?;
        self.index += 1;

        Some(entry)
    }
}

impl<'data, T: ?Sized, A: NbtAccess, C: IndexCore<A>> IntoIterator for IndexedList<'data, T, A, C>
where
    ListOwnedIter<'data, T, A, C>: Iterator,
    <A as NbtAccess>::CORE<'data, C>: Clone,
{
    type IntoIter = ListOwnedIter<'data, T, A, C>;
    type Item = <Self::IntoIter as Iterator>::Item;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { ListOwnedIter::new(self) }
}
impl<'data, T: IndexableValue + ?Sized, A: NbtAccess, C: IndexCore<A>> ExactSizeIterator
    for ListOwnedIter<'data, T, A, C>
where
    <A as NbtAccess>::CORE<'data, C>: Clone,
{
    fn len(&self) -> usize { self.list.len() - self.index }
}

impl<'data, A: NbtAccess, C: IndexCore<A>> Iterator for ListOwnedIter<'data, IndexedMapType, A, C>
where
    <A as NbtAccess>::CORE<'data, C>: Clone,
{
    type Item = IndexedCompound<'data, A, C>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.list.clone().get(self.index)?;
        self.index += 1;

        Some(entry)
    }
}
impl<'data, A: NbtAccess, C: IndexCore<A>> ExactSizeIterator
    for ListOwnedIter<'data, IndexedMapType, A, C>
where
    <A as NbtAccess>::CORE<'data, C>: Clone,
{
    fn len(&self) -> usize { self.list.len() - self.index }
}

impl<'data, A: NbtAccess, C: IndexCore<A>> Iterator for ListOwnedIter<'data, IndexedListType, A, C>
where
    <A as NbtAccess>::CORE<'data, C>: Clone,
{
    type Item = IndexedValueList<'data, A, C>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.list.clone().get(self.index)?;
        self.index += 1;

        Some(entry)
    }
}
impl<'data, A: NbtAccess, C: IndexCore<A>> ExactSizeIterator
    for ListOwnedIter<'data, IndexedListType, A, C>
where
    <A as NbtAccess>::CORE<'data, C>: Clone,
{
    fn len(&self) -> usize { self.list.len() - self.index }
}

// -------------------------------------------------------------------------------------------------

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

impl<'iter, T: IndexableValue + ?Sized, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> Iterator
    for ListIter<'iter, '_, T, A, C>
{
    type Item = T::Value<'iter>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.list.get_ref(self.index)?;
        self.index += 1;

        Some(entry)
    }
}
impl<T: IndexableValue + ?Sized, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> ExactSizeIterator
    for ListIter<'_, '_, T, A, C>
{
    fn len(&self) -> usize { self.list.len() - self.index }
}

impl<'iter, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> Iterator
    for ListIter<'iter, '_, IndexedMapType, A, C>
{
    type Item = IndexedCompound<'iter, Ref, C>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.list.get_ref(self.index)?;
        self.index += 1;

        Some(entry)
    }
}
impl<A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> ExactSizeIterator
    for ListIter<'_, '_, IndexedMapType, A, C>
{
    fn len(&self) -> usize { self.list.len() - self.index }
}

impl<'iter, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> Iterator
    for ListIter<'iter, '_, IndexedListType, A, C>
{
    type Item = IndexedValueList<'iter, Ref, C>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.list.get_ref(self.index)?;
        self.index += 1;

        Some(entry)
    }
}
impl<A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> ExactSizeIterator
    for ListIter<'_, '_, IndexedListType, A, C>
{
    fn len(&self) -> usize { self.list.len() - self.index }
}
