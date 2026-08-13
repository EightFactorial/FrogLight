#![expect(clippy::into_iter_without_iter, reason = "Ignored")]

use crate::types::indexed::{
    compound::IndexedCompound,
    core::{IndexCore, NbtAccess, Ref},
    list::{IndexedList, ValueList},
    reference::{IndexableValue, ValueReference},
    types::{IndexedListType, IndexedMapType},
};

pub struct ListValueIter<'index, A: NbtAccess, C: IndexCore<A>>
where
    ValueList<'index, A, C>: Clone,
{
    list: ValueList<'index, A, C>,
    index: usize,
}

impl<'index, A: NbtAccess, C: IndexCore<A>> ListValueIter<'index, A, C>
where
    ValueList<'index, A, C>: Clone,
{
    /// Create a new [`ListValueIter`] from the given [`ValueList`].
    #[inline]
    #[must_use]
    pub const fn new(list: ValueList<'index, A, C>) -> Self { ListValueIter { list, index: 0 } }
}

impl<'index, A: NbtAccess, C: IndexCore<A>> Iterator for ListValueIter<'index, A, C>
where
    ValueList<'index, A, C>: Clone,
{
    type Item = ValueReference<'index, A, C>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.list.clone().get(self.index)?;
        self.index += 1;

        Some(entry)
    }
}

impl<'index, A: NbtAccess, C: IndexCore<A>> ExactSizeIterator for ListValueIter<'index, A, C>
where
    ValueList<'index, A, C>: Clone,
{
    fn len(&self) -> usize { self.list.len() - self.index }
}

impl<'index, A: NbtAccess, C: IndexCore<A>> IntoIterator for ValueList<'index, A, C>
where
    ListValueIter<'index, A, C>: Iterator,
    ValueList<'index, A, C>: Clone,
{
    type IntoIter = ListValueIter<'index, A, C>;
    type Item = <Self::IntoIter as Iterator>::Item;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { ListValueIter::new(self) }
}

impl<'index, A: NbtAccess, C: IndexCore<A>> IntoIterator for &'index ValueList<'index, A, C>
where
    ListValueIter<'index, A, C>: Iterator,
    ValueList<'index, A, C>: Copy,
{
    type IntoIter = ListValueIter<'index, A, C>;
    type Item = <Self::IntoIter as Iterator>::Item;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { ListValueIter::new(*self) }
}
impl<'index, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> IntoIterator
    for &'index mut ValueList<'index, A, C>
where
    ListValueIter<'index, A, C>: Iterator,
    ValueList<'index, A, C>: Copy,
{
    type IntoIter = ListValueIter<'index, A, C>;
    type Item = <Self::IntoIter as Iterator>::Item;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { ListValueIter::new(*self) }
}

// -------------------------------------------------------------------------------------------------

/// An iterator over the entries in an [`IndexedList`].
pub struct ListOwnedIter<'index, T: ?Sized, A: NbtAccess, C: IndexCore<A>>
where
    <A as NbtAccess>::CORE<'index, C>: Clone,
{
    list: IndexedList<'index, T, A, C>,
    index: usize,
}

impl<'index, T: ?Sized, A: NbtAccess, C: IndexCore<A>> ListOwnedIter<'index, T, A, C>
where
    <A as NbtAccess>::CORE<'index, C>: Clone,
{
    /// Create a new [`ListIter`] from the given [`IndexedList`].
    #[inline]
    #[must_use]
    pub const fn new(list: IndexedList<'index, T, A, C>) -> Self {
        ListOwnedIter { list, index: 0 }
    }
}

impl<'index, T: IndexableValue + ?Sized, A: NbtAccess, C: IndexCore<A>> Iterator
    for ListOwnedIter<'index, T, A, C>
where
    <A as NbtAccess>::CORE<'index, C>: Clone,
{
    type Item = T::Value<'index>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.list.clone().get(self.index)?;
        self.index += 1;

        Some(entry)
    }
}

impl<'index, T: ?Sized, A: NbtAccess, C: IndexCore<A>> IntoIterator for IndexedList<'index, T, A, C>
where
    ListOwnedIter<'index, T, A, C>: Iterator,
    <A as NbtAccess>::CORE<'index, C>: Clone,
{
    type IntoIter = ListOwnedIter<'index, T, A, C>;
    type Item = <Self::IntoIter as Iterator>::Item;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { ListOwnedIter::new(self) }
}
impl<'index, T: IndexableValue + ?Sized, A: NbtAccess, C: IndexCore<A>> ExactSizeIterator
    for ListOwnedIter<'index, T, A, C>
where
    <A as NbtAccess>::CORE<'index, C>: Clone,
{
    fn len(&self) -> usize { self.list.len() - self.index }
}

impl<'index, A: NbtAccess, C: IndexCore<A>> Iterator for ListOwnedIter<'index, IndexedMapType, A, C>
where
    <A as NbtAccess>::CORE<'index, C>: Clone,
{
    type Item = IndexedCompound<'index, A, C>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.list.clone().get(self.index)?;
        self.index += 1;

        Some(entry)
    }
}
impl<'index, A: NbtAccess, C: IndexCore<A>> ExactSizeIterator
    for ListOwnedIter<'index, IndexedMapType, A, C>
where
    <A as NbtAccess>::CORE<'index, C>: Clone,
{
    fn len(&self) -> usize { self.list.len() - self.index }
}

impl<'index, A: NbtAccess, C: IndexCore<A>> Iterator
    for ListOwnedIter<'index, IndexedListType, A, C>
where
    <A as NbtAccess>::CORE<'index, C>: Clone,
{
    type Item = ValueList<'index, A, C>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.list.clone().get(self.index)?;
        self.index += 1;

        Some(entry)
    }
}
impl<'index, A: NbtAccess, C: IndexCore<A>> ExactSizeIterator
    for ListOwnedIter<'index, IndexedListType, A, C>
where
    <A as NbtAccess>::CORE<'index, C>: Clone,
{
    fn len(&self) -> usize { self.list.len() - self.index }
}

// -------------------------------------------------------------------------------------------------

/// An iterator over the entries in an [`IndexedList`].
pub struct ListIter<'iter, 'index, T: ?Sized, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> {
    list: &'iter IndexedList<'index, T, A, C>,
    index: usize,
}

impl<'iter, 'index, T: ?Sized, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>>
    ListIter<'iter, 'index, T, A, C>
{
    /// Create a new [`ListIter`] from the given [`IndexedList`].
    #[inline]
    #[must_use]
    pub const fn new(list: &'iter IndexedList<'index, T, A, C>) -> Self {
        ListIter { list, index: 0 }
    }
}

impl<'iter, 'index, T: ?Sized, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> IntoIterator
    for &'iter IndexedList<'index, T, A, C>
where
    ListIter<'iter, 'index, T, A, C>: Iterator,
{
    type IntoIter = ListIter<'iter, 'index, T, A, C>;
    type Item = <Self::IntoIter as Iterator>::Item;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { ListIter::new(self) }
}
impl<'iter, 'index, T: IndexableValue + ?Sized, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>>
    IntoIterator for &'iter mut IndexedList<'index, T, A, C>
where
    ListIter<'iter, 'index, T, A, C>: Iterator,
{
    type IntoIter = ListIter<'iter, 'index, T, A, C>;
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
    type Item = ValueList<'iter, Ref, C>;

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
