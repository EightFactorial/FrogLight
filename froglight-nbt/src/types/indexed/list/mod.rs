//! TODO

use core::fmt;

use crate::types::indexed::{
    compound::IndexedCompound,
    core::{IndexCore, Mut, NbtAccess, Ref},
    index::Index,
    reference::{IndexableValue, IndexableValueMut},
    types::{IndexedListType, IndexedMapType},
};

mod iter;
pub use iter::{ListIter, ListOwnedIter};

mod value;
pub use value::ValueList;

/// A typed NBT List that is indexed by an [`IndexCore`].
pub struct IndexedList<'data, T: ?Sized, A: NbtAccess, C: IndexCore<A> + 'data> {
    core: A::CORE<'data, C>,
    index: Index<T>,
}

impl<'data, T: ?Sized, A: NbtAccess, C: IndexCore<A> + 'data> IndexedList<'data, T, A, C> {
    /// Create a new [`IndexedList`] from the given core and [`Index`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is valid for the given core.
    #[inline]
    #[must_use]
    pub const unsafe fn new(core: A::CORE<'data, C>, index: Index<T>) -> Self {
        Self { core, index }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'data, T: IndexableValue + ?Sized, A: NbtAccess, C: IndexCore<A> + 'data>
    IndexedList<'data, T, A, C>
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
    #[must_use]
    pub fn get(self, index: usize) -> Option<T::Value<'data>> {
        if index >= self.len() {
            return None;
        }

        let core = A::into_core(self.core);
        let root = <C as IndexCore<A>>::root(core);
        let value_index = self.index.value();

        if T::LIST_INDEX_IS_ENTRY_RANGE {
            // SAFETY: The index is valid for this core.
            let entries = unsafe { <C as IndexCore<A>>::entry_range(core, value_index) };

            // SAFETY: The length was already checked.
            let entry = unsafe { entries.get_unchecked(index) };
            let index = Index::new(entry.value().index());

            // SAFETY: The index is valid for this core.
            Some(unsafe { T::get(root, index) })
        } else {
            let first = Index::new(1 + 4 + value_index);
            let size = unsafe { T::size(root, first) };

            // SAFETY: The index is valid for this core.
            Some(unsafe { T::get(root, Index::new(first.value() + (size * index))) })
        }
    }

    /// Get a reference to the value at the given index,
    /// or `None` if the index is out of bounds.
    #[must_use]
    pub fn get_ref(&self, index: usize) -> Option<T::Value<'_>> {
        if index >= self.len() {
            return None;
        }

        let root = <C as IndexCore<A>>::root(&self.core);
        let value_index = self.index.value();

        if T::LIST_INDEX_IS_ENTRY_RANGE {
            // SAFETY: The index is valid for this core.
            let entries = unsafe { <C as IndexCore<A>>::entry_range(&self.core, value_index) };

            // SAFETY: The length was already checked.
            let entry = unsafe { entries.get_unchecked(index) };
            let index = Index::new(entry.value().index());

            // SAFETY: The index is valid for this core.
            Some(unsafe { T::get(root, index) })
        } else {
            let first = Index::new(1 + 4 + value_index); // Past the `type` and `length` tags
            let size = unsafe { T::size(root, first) };

            // SAFETY: The index is valid for this core.
            Some(unsafe { T::get(root, Index::new(first.value() + (size * index))) })
        }
    }

    /// Return an iterator over the entries in the list.
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> ListIter<'_, 'data, T, A, C>
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
    pub const fn into_iter(self) -> ListOwnedIter<'data, T, A, C>
    where
        <A as NbtAccess>::CORE<'data, C>: Copy,
    {
        ListOwnedIter::new(self)
    }
}

impl<'data, T: IndexableValueMut + ?Sized, C: IndexCore<Mut> + 'data>
    IndexedList<'data, T, Mut, C>
{
    /// Set the value at the given index,
    /// or `None` if the index is out of bounds.
    pub fn set(&mut self, value: T::Value<'_>, index: usize) -> Option<()> {
        if index >= self.len() {
            return None;
        }

        let value_index = self.index.value();

        if T::LIST_INDEX_IS_ENTRY_RANGE {
            // SAFETY: The index is valid for this core.
            let entries = unsafe { <C as IndexCore<Ref>>::entry_range(self.core, value_index) };

            // SAFETY: The length was already checked.
            let entry = unsafe { entries.get_unchecked(index) };
            let index = Index::new(entry.value().index());

            // SAFETY: The index is valid for this core.
            let root = self.core.root_mut();
            unsafe { T::set(root, index, value) };
        } else {
            let root = self.core.root_mut();
            let size = unsafe { T::size(root, Index::new(index + 4)) };
            let index = Index::new(4 + (size * index));

            // SAFETY: The index is valid for this core.
            unsafe { T::set(root, index, value) };
        }

        Some(())
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
    pub const fn iter_mut(&mut self) -> ListIter<'_, 'data, T, Mut, C> { ListIter::new(self) }
}

// -------------------------------------------------------------------------------------------------

impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> IndexedList<'data, IndexedMapType, A, C> {
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
    pub fn get(self, index: usize) -> Option<IndexedCompound<'data, A, C>> {
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
    pub const fn iter(&self) -> ListIter<'_, 'data, IndexedMapType, A, C>
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
    pub const fn into_iter(self) -> ListOwnedIter<'data, IndexedMapType, A, C>
    where
        <A as NbtAccess>::CORE<'data, C>: Copy,
    {
        ListOwnedIter::new(self)
    }
}

impl<'data, C: IndexCore<Mut> + 'data> IndexedList<'data, IndexedMapType, Mut, C> {
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
    pub const fn iter_mut(&mut self) -> ListIter<'_, 'data, IndexedMapType, Mut, C> {
        ListIter::new(self)
    }
}

// -------------------------------------------------------------------------------------------------

impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> IndexedList<'data, IndexedListType, A, C> {
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
    pub fn get(self, index: usize) -> Option<ValueList<'data, A, C>> {
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
    pub const fn iter(&self) -> ListIter<'_, 'data, IndexedListType, A, C>
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
    pub const fn into_iter(self) -> ListOwnedIter<'data, IndexedListType, A, C>
    where
        <A as NbtAccess>::CORE<'data, C>: Copy,
    {
        ListOwnedIter::new(self)
    }
}

impl<'data, C: IndexCore<Mut> + 'data> IndexedList<'data, IndexedListType, Mut, C> {
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
    pub const fn iter_mut(&mut self) -> ListIter<'_, 'data, IndexedListType, Mut, C> {
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

impl<'data, T: ?Sized, A: NbtAccess, C: IndexCore<A> + 'data> Clone for IndexedList<'data, T, A, C>
where
    <A as NbtAccess>::CORE<'data, C>: Clone,
{
    fn clone(&self) -> Self { Self { core: self.core.clone(), index: self.index } }
}
impl<'data, T: ?Sized, A: NbtAccess, C: IndexCore<A> + 'data> Copy for IndexedList<'data, T, A, C> where
    <A as NbtAccess>::CORE<'data, C>: Copy
{
}

impl<'data, T: ?Sized, A: NbtAccess, C: IndexCore<A> + 'data> PartialEq
    for IndexedList<'data, T, A, C>
{
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
            && <C as IndexCore<A>>::root(&self.core) == <C as IndexCore<A>>::root(&other.core)
    }
}
impl<'data, T: ?Sized, A: NbtAccess, C: IndexCore<A> + 'data> Eq for IndexedList<'data, T, A, C> {}
