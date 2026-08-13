//! TODO

use core::fmt;

use crate::types::indexed::{
    compound::IndexedCompound,
    core::{IndexCore, Mut, NbtAccess, Ref},
    index::Index,
    reference::{IndexableValue, IndexableValueMut, IndexedReference},
    types::{IndexedListType, IndexedMapType},
};

mod iter;
pub use iter::{ListIter, ListOwnedIter};

mod value;
pub use value::ValueList;

/// A typed NBT List that is indexed by an [`IndexCore`].
pub struct IndexedList<'index, T: ?Sized, A: NbtAccess, C: IndexCore<A> + 'index> {
    core: A::CORE<'index, C>,
    index: Index<T>,
}

impl<'index, T: ?Sized, A: NbtAccess, C: IndexCore<A> + 'index> IndexedList<'index, T, A, C> {
    /// Create a new [`IndexedList`] from the given core and [`Index`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is valid for the given core.
    #[inline]
    #[must_use]
    pub const unsafe fn new(core: A::CORE<'index, C>, index: Index<T>) -> Self {
        Self { core, index }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'index, T: IndexableValue + ?Sized, A: NbtAccess, C: IndexCore<A> + 'index>
    IndexedList<'index, T, A, C>
{
    /// Get the length of the list.
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "Should never panic")]
    pub fn len(&self) -> usize {
        let value_index = self.index.value();
        if T::LIST_INDEX_IS_ENTRY_RANGE {
            unsafe { <C as IndexCore<A>>::entry_range(&self.core, value_index).len() }
        } else {
            let root = <C as IndexCore<A>>::root(&self.core);
            let index = Index::new(1 + value_index); // Past the `type` tag
            let length = unsafe { <u32 as IndexableValue>::get(root, index) }; // Read the `length` tag
            usize::try_from(length).expect("Length is too large!")
        }
    }

    /// Returns `true` if the length of the list is zero.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get the value at the given index,
    /// or `None` if the index is out of bounds.
    #[inline]
    #[must_use]
    pub fn get(self, index: usize) -> Option<T::Value<'index>> {
        self.get_indexed(index).map(IndexedReference::get)
    }

    /// Get a reference to the value at the given index,
    /// or `None` if the index is out of bounds.
    #[inline]
    #[must_use]
    pub fn get_ref(&self, index: usize) -> Option<T::Value<'_>> {
        self.get_indexed_ref(index).map(IndexedReference::get)
    }

    /// Get an [`IndexedReference`] to the value at the given index,
    /// or `None` if the index is out of bounds.
    #[must_use]
    pub fn get_indexed(self, index: usize) -> Option<IndexedReference<'index, T, A>> {
        if index >= self.len() {
            return None;
        }

        let value_index = self.index.value();

        if T::LIST_INDEX_IS_ENTRY_RANGE {
            // SAFETY: The index is valid for this core.
            let entries = unsafe { <C as IndexCore<A>>::entry_range(&self.core, value_index) };

            // SAFETY: The length was already checked.
            let entry = unsafe { entries.get_unchecked(index) };
            let index = Index::new(entry.value().index());

            // SAFETY: The index is valid for this core.
            let root = A::into_slice(self.core);
            Some(unsafe { IndexedReference::<T, A>::new(root, index) })
        } else {
            let root = A::into_slice(self.core);
            let first = Index::new(1 + 4 + value_index); // Past the `type` and `length` tags
            let size = unsafe { T::size(&root, first) };

            // SAFETY: The index is valid for this core.
            let index = Index::new(first.value() + (size * index));
            Some(unsafe { IndexedReference::<T, A>::new(root, index) })
        }
    }

    /// Get an [`IndexedReference`] to the value at the given index,
    /// or `None` if the index is out of bounds.
    #[must_use]
    pub fn get_indexed_ref(&self, index: usize) -> Option<IndexedReference<'_, T, Ref>> {
        if index >= self.len() {
            return None;
        }

        let root = self.core.root();
        let value_index = self.index.value();

        if T::LIST_INDEX_IS_ENTRY_RANGE {
            // SAFETY: The index is valid for this core.
            let entries = unsafe { <C as IndexCore<A>>::entry_range(&self.core, value_index) };

            // SAFETY: The length was already checked.
            let entry = unsafe { entries.get_unchecked(index) };
            let index = Index::new(entry.value().index());

            // SAFETY: The index is valid for this core.
            Some(unsafe { IndexedReference::<T, Ref>::new(root, index) })
        } else {
            let first = Index::new(1 + 4 + value_index); // Past the `type` and `length` tags
            let size = unsafe { T::size(root, first) };

            // SAFETY: The index is valid for this core.
            let index = Index::new(first.value() + (size * index));
            Some(unsafe { IndexedReference::<T, Ref>::new(root, index) })
        }
    }

    /// Return an iterator over the entries in the list.
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> ListIter<'_, 'index, T, A, C>
    where
        C: IndexCore<Ref>,
    {
        ListIter::new(self)
    }

    /// Return an owned iterator over the entries in the list.
    ///
    /// # Note
    ///
    /// This function requires `Copy`, but the iterator only requires `Clone`!
    ///
    /// This is to prevent accidental `Clone`s, which can be very expensive.
    #[inline]
    #[must_use]
    pub const fn into_iter(self) -> ListOwnedIter<'index, T, A, C>
    where
        <A as NbtAccess>::CORE<'index, C>: Copy,
    {
        ListOwnedIter::new(self)
    }
}

impl<'index, T: IndexableValueMut + ?Sized, C: IndexCore<Mut> + 'index>
    IndexedList<'index, T, Mut, C>
{
    /// Get an [`IndexedReference`] to the value at the given index,
    /// or `None` if the index is out of bounds.
    #[must_use]
    pub fn get_indexed_mut(&mut self, index: usize) -> Option<IndexedReference<'_, T, Mut>> {
        if index >= self.len() {
            return None;
        }

        let value_index = self.index.value();

        if T::LIST_INDEX_IS_ENTRY_RANGE {
            // SAFETY: The index is valid for this core.
            let entries = unsafe { <C as IndexCore<Mut>>::entry_range(self.core, value_index) };

            // SAFETY: The length was already checked.
            let entry = unsafe { entries.get_unchecked(index) };
            let index = Index::new(entry.value().index());

            // SAFETY: The index is valid for this core.
            let root = self.core.root_mut();
            Some(unsafe { IndexedReference::<T, Mut>::new(root, index) })
        } else {
            let root = self.core.root_mut();
            let first = Index::new(1 + 4 + value_index); // Past the `type` and `length` tags
            let size = unsafe { T::size(root, first) };

            // SAFETY: The index is valid for this core.
            let index = Index::new(first.value() + (size * index));
            Some(unsafe { IndexedReference::<T, Mut>::new(root, index) })
        }
    }

    /// Set the value at the given index,
    /// or `None` if the index is out of bounds.
    pub fn set(&mut self, value: T::Value<'_>, index: usize) -> Option<()> {
        self.get_indexed_mut(index).map(|mut r| r.set(value))
    }

    /// Set the value at the given index, returning the previous value.
    #[must_use]
    #[expect(clippy::useless_conversion, reason = "Static lifetime")]
    pub fn replace(&mut self, value: T::Value<'_>, index: usize) -> Option<T::Value<'static>>
    where
        for<'a> T::Value<'a>: Into<T::Value<'static>>,
    {
        let previous = self.get_ref(index)?.into();
        self.set(value, index);
        Some(previous)
    }

    /// Get an iterator over the entries in the list.
    #[inline]
    #[must_use]
    pub const fn iter_mut(&mut self) -> ListIter<'_, 'index, T, Mut, C> { ListIter::new(self) }
}

// -------------------------------------------------------------------------------------------------

impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> IndexedList<'index, IndexedMapType, A, C> {
    /// Get the length of the list.
    #[must_use]
    pub fn len(&self) -> usize {
        unsafe { <C as IndexCore<A>>::entry_range(&self.core, self.index.value()).len() }
    }

    /// Returns `true` if the length of the list is zero.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get the value at the given index,
    /// or `None` if the index is out of bounds.
    #[must_use]
    pub fn get(self, index: usize) -> Option<IndexedCompound<'index, A, C>> {
        if index >= self.len() {
            return None;
        }

        unsafe {
            // SAFETY: The index is valid for this core.
            let entries = <C as IndexCore<A>>::entry_range(&self.core, self.index.value());
            // SAFETY: The length was already checked.
            let index = entries.get_unchecked(index).value().index();
            // SAFETY: The index is valid for this core.
            Some(IndexedCompound::<A, C>::new(self.core, index))
        }
    }

    /// Get a reference to value at the given index,
    /// or `None` if the index is out of bounds.
    #[must_use]
    pub fn get_ref(&self, index: usize) -> Option<IndexedCompound<'_, Ref, C>>
    where
        C: IndexCore<Ref>,
    {
        if index >= self.len() {
            return None;
        }

        unsafe {
            // SAFETY: The index is valid for this core.
            let entries = <C as IndexCore<Ref>>::entry_range(&self.core, self.index.value());
            // SAFETY: The length was already checked.
            let entry = entries.get_unchecked(index);
            // SAFETY: The index is valid for this core.
            Some(IndexedCompound::<Ref, C>::new(&self.core, entry.value().index()))
        }
    }

    /// Get an iterator over the entries in the list.
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> ListIter<'_, 'index, IndexedMapType, A, C>
    where
        C: IndexCore<Ref>,
    {
        ListIter::new(self)
    }

    /// Get an owned iterator over the entries in the list.
    ///
    /// # Note
    ///
    /// This function requires `Copy`, but the iterator only requires `Clone`!
    ///
    /// This is to prevent accidental `Clone`s, which can be very expensive.
    #[inline]
    #[must_use]
    pub const fn into_iter(self) -> ListOwnedIter<'index, IndexedMapType, A, C>
    where
        <A as NbtAccess>::CORE<'index, C>: Copy,
    {
        ListOwnedIter::new(self)
    }
}

impl<'index, C: IndexCore<Mut> + 'index> IndexedList<'index, IndexedMapType, Mut, C> {
    /// Get the value at the given index,
    /// or `None` if the index is out of bounds.
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<IndexedCompound<'_, Mut, C>> {
        if index >= self.len() {
            return None;
        }

        unsafe {
            // SAFETY: The index is valid for this core.
            let entries = <C as IndexCore<Ref>>::entry_range(self.core, self.index.value());
            // SAFETY: The length was already checked.
            let entry = entries.get_unchecked(index);
            // SAFETY: The index is valid for this core.
            Some(IndexedCompound::<Mut, C>::new(self.core, entry.value().index()))
        }
    }

    /// Get an iterator over the entries in the list.
    #[inline]
    #[must_use]
    #[expect(clippy::iter_without_into_iter, reason = "Not correct")]
    pub const fn iter_mut(&mut self) -> ListIter<'_, 'index, IndexedMapType, Mut, C> {
        ListIter::new(self)
    }
}

// -------------------------------------------------------------------------------------------------

impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> IndexedList<'index, IndexedListType, A, C> {
    /// Get the length of the list.
    #[must_use]
    pub fn len(&self) -> usize {
        unsafe { <C as IndexCore<A>>::entry_range(&self.core, self.index.value()).len() }
    }

    /// Returns `true` if the length of the list is zero.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get the value at the given index,
    /// or `None` if the index is out of bounds.
    #[must_use]
    pub fn get(self, index: usize) -> Option<ValueList<'index, A, C>> {
        if index >= self.len() {
            return None;
        }

        Some(crate::types::indexed::entry::value::create_list::<C, A>(self.core, self.index))
    }

    /// Get a reference to value at the given index,
    /// or `None` if the index is out of bounds.
    #[must_use]
    pub fn get_ref(&self, index: usize) -> Option<ValueList<'_, Ref, C>>
    where
        C: IndexCore<Ref>,
    {
        if index >= self.len() {
            return None;
        }

        Some(crate::types::indexed::entry::value::create_list::<C, Ref>(&self.core, self.index))
    }

    /// Get an iterator over the entries in the list.
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> ListIter<'_, 'index, IndexedListType, A, C>
    where
        C: IndexCore<Ref>,
    {
        ListIter::new(self)
    }

    /// Get an owned iterator over the entries in the list.
    ///
    /// # Note
    ///
    /// This function requires `Copy`, but the iterator only requires `Clone`!
    ///
    /// This is to prevent accidental `Clone`s, which can be very expensive.
    #[inline]
    #[must_use]
    pub const fn into_iter(self) -> ListOwnedIter<'index, IndexedListType, A, C>
    where
        <A as NbtAccess>::CORE<'index, C>: Copy,
    {
        ListOwnedIter::new(self)
    }
}

impl<'index, C: IndexCore<Mut> + 'index> IndexedList<'index, IndexedListType, Mut, C> {
    /// Get a reference to value at the given index,
    /// or `None` if the index is out of bounds.
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<ValueList<'_, Mut, C>> {
        if index >= self.len() {
            return None;
        }

        Some(crate::types::indexed::entry::value::create_list_mut(self.core, self.index))
    }

    /// Get an iterator over the entries in the list.
    #[inline]
    #[must_use]
    #[allow(clippy::iter_without_into_iter, reason = "Not correct")]
    pub const fn iter_mut(&mut self) -> ListIter<'_, 'index, IndexedListType, Mut, C> {
        ListIter::new(self)
    }
}

// -------------------------------------------------------------------------------------------------

impl<T: ?Sized, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> fmt::Debug
    for IndexedList<'_, T, A, C>
where
    for<'a> &'a Self: IntoIterator,
    for<'a> <&'a Self as IntoIterator>::Item: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<'index, T: ?Sized, A: NbtAccess, C: IndexCore<A> + 'index> Clone
    for IndexedList<'index, T, A, C>
where
    <A as NbtAccess>::CORE<'index, C>: Clone,
{
    fn clone(&self) -> Self { Self { core: self.core.clone(), index: self.index } }
}
impl<'index, T: ?Sized, A: NbtAccess, C: IndexCore<A> + 'index> Copy
    for IndexedList<'index, T, A, C>
where
    <A as NbtAccess>::CORE<'index, C>: Copy,
{
}

impl<'index, T: ?Sized, A: NbtAccess, C: IndexCore<A> + 'index> PartialEq
    for IndexedList<'index, T, A, C>
{
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
            && <C as IndexCore<A>>::root(&self.core) == <C as IndexCore<A>>::root(&other.core)
    }
}
impl<'index, T: ?Sized, A: NbtAccess, C: IndexCore<A> + 'index> Eq
    for IndexedList<'index, T, A, C>
{
}
