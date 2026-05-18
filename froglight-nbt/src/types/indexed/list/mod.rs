//! TODO

use crate::types::indexed::{
    compound::IndexedCompound,
    core::{IndexCore, Mut, NbtAccess, Ref},
    entry::{IndexedListType, IndexedMapType},
    index::Index,
    reference::{IndexableValue, IndexableValueMut},
};

mod iter;
pub use iter::ListIter;

mod value;
pub use value::IndexedValueList;

cfg_select! {
    feature = "alloc" => {
        /// A typed NBT List that is indexed by an [`IndexCore`].
        pub struct IndexedList<'data, T: ?Sized, A: NbtAccess, C: IndexCore<A> + 'data = super::alloc::SliceCore<'data, A>> {
            core: A::CORE<'data, C>,
            index: Index<T>,
        }
    }
    _ => {
        /// A typed NBT List that is indexed by an [`IndexCore`].
        pub struct IndexedList<'data, T: ?Sized, A: NbtAccess, C: IndexCore<A> + 'data> {
            core: A::CORE<'data, C>,
            index: Index<T>,
        }
    }
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

impl<'data, T: IndexableValue + ?Sized, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A> + 'data>
    IndexedList<'data, T, A, C>
{
    /// Get the length of the list.
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "Should never panic")]
    pub fn len(&self) -> usize {
        let value_index = self.index.value();
        if T::LIST_INDEX_IS_ENTRY_RANGE {
            unsafe { <C as IndexCore<Ref>>::entry_range(&self.core, value_index).len() }
        } else {
            let root = <C as IndexCore<Ref>>::root(&self.core);
            let index = Index::new(1 + value_index);
            let length = unsafe { <u32 as IndexableValue>::get(root, index) };
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
    pub fn get(&self, index: usize) -> Option<T::Value<'_>> {
        if index >= self.len() {
            return None;
        }

        let root = <C as IndexCore<Ref>>::root(&self.core);
        let value_index = self.index.value();

        if T::LIST_INDEX_IS_ENTRY_RANGE {
            // SAFETY: The index is valid for this core.
            let entries = unsafe { <C as IndexCore<Ref>>::entry_range(&self.core, value_index) };

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

    /// Get an iterator over the entries in the list.
    #[inline]
    #[must_use]
    pub fn iter(&self) -> ListIter<'_, 'data, T, A, C> { ListIter::new(self) }
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
        let previous = self.get(index)?.into();
        self.set(value, index).map(|()| previous)
    }

    /// Get an iterator over the entries in the list.
    #[inline]
    #[must_use]
    pub fn iter_mut(&mut self) -> ListIter<'_, 'data, T, Mut, C> { ListIter::new(self) }
}

// -------------------------------------------------------------------------------------------------

impl<'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A> + 'data>
    IndexedList<'data, IndexedMapType, A, C>
{
    /// Get the length of the list.
    #[must_use]
    pub fn len(&self) -> usize {
        unsafe { <C as IndexCore<Ref>>::entry_range(&self.core, self.index.value()).len() }
    }

    /// Returns `true` if the length of the list is zero.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get the value at the given index,
    /// or `None` if the index is out of bounds.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<IndexedCompound<'_, Ref, C>> {
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
    pub fn iter(&self) -> ListIter<'_, 'data, IndexedMapType, A, C> { ListIter::new(self) }
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
    pub fn iter_mut(&mut self) -> ListIter<'_, 'data, IndexedMapType, Mut, C> {
        ListIter::new(self)
    }
}

// -------------------------------------------------------------------------------------------------

impl<'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A> + 'data>
    IndexedList<'data, IndexedListType, A, C>
{
    /// Get the length of the list.
    #[must_use]
    pub fn len(&self) -> usize {
        unsafe { <C as IndexCore<Ref>>::entry_range(&self.core, self.index.value()).len() }
    }

    /// Returns `true` if the length of the list is zero.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get the value at the given index,
    /// or `None` if the index is out of bounds.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<IndexedValueList<'_, Ref, C>> {
        if index >= self.len() {
            return None;
        }

        Some(crate::types::indexed::entry::value::create_list(&self.core, self.index))
    }

    /// Get an iterator over the entries in the list.
    #[inline]
    #[must_use]
    pub fn iter(&self) -> ListIter<'_, 'data, IndexedListType, A, C> { ListIter::new(self) }
}

impl<'data, C: IndexCore<Mut> + 'data> IndexedList<'data, IndexedListType, Mut, C> {
    /// Get the value at the given index,
    /// or `None` if the index is out of bounds.
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<IndexedValueList<'_, Mut, C>> {
        if index >= self.len() {
            return None;
        }

        Some(crate::types::indexed::entry::value::create_list_mut(self.core, self.index))
    }

    /// Get an iterator over the entries in the list.
    #[inline]
    #[must_use]
    #[allow(clippy::iter_without_into_iter, reason = "Not correct")]
    pub fn iter_mut(&mut self) -> ListIter<'_, 'data, IndexedListType, Mut, C> {
        ListIter::new(self)
    }
}
