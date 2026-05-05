//! TODO

use core::{fmt, marker::PhantomData};

use crate::types::borrowed::{
    IndexedCore,
    reference::{BorrowedIndex, BorrowedMut, BorrowedPOD, BorrowedRef},
    value::{BorrowedValueMut, BorrowedValueRef, IndexedList},
};

/// An NBT list tag with a reference to its data.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct IndexedListRef<'data, T> {
    root: &'data [u8],
    core: &'data IndexedCore,
    index: usize,
    _phantom: PhantomData<T>,
}

impl<'data, T> IndexedListRef<'data, T> {
    /// Create a new [`IndexedListRef`] with the given core and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `index` is a valid index of `core`.
    #[must_use]
    pub(super) const unsafe fn new(
        root: &'data [u8],
        core: &'data IndexedCore,
        index: usize,
    ) -> Self {
        Self { root, core, index, _phantom: PhantomData }
    }
}

impl<T: BorrowedPOD> IndexedListRef<'_, T> {
    /// Get the number of entries in this [`IndexedListRef`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        unsafe { BorrowedRef::new(self.root, BorrowedIndex::<[u8]>::new(self.index)).len() }
    }

    /// Returns `true` if the list is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get the value at the provided index, if it exists.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get_value(&self, index: usize) -> Option<T> {
        if index < self.len() {
            let index = self.index + (core::mem::size_of::<T>() * index);

            // SAFETY: `index` is valid for `core` and is of type `T`
            Some(unsafe { BorrowedRef::new(self.root, BorrowedIndex::new(index)).get_value() })
        } else {
            None
        }
    }

    /// Get an iterator over all entries in this list.
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        (0..self.len()).map(move |index| {
            let index = self.index + (core::mem::size_of::<T>() * index);

            // SAFETY: `index` is valid for `core` and is of type `T`
            unsafe { BorrowedRef::new(self.root, BorrowedIndex::new(index)).get_value() }
        })
    }
}

impl IndexedListRef<'_, IndexedList> {
    /// Get the number of entries in this [`IndexedListRef`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        // SAFETY: `index` is always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };

        range.end.saturating_sub(range.start)
    }

    /// Returns `true` if the list is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get the value at the provided index, if it exists.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<BorrowedValueRef<'_>> {
        // SAFETY: `index` and `range` are always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };
        let entries = unsafe { self.core.entries().get_unchecked(*range) };

        entries
            .get(index)
            .map(|entry| unsafe { BorrowedValueRef::new(self.root, self.core, entry.value()) })
    }

    /// Get an iterator over all entries in this list.
    pub fn iter(&self) -> impl Iterator<Item = BorrowedValueRef<'_>> + '_ {
        // SAFETY: `index` and `range` are always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };
        let entries = unsafe { self.core.entries().get_unchecked(*range) };

        entries
            .iter()
            .map(move |entry| unsafe { BorrowedValueRef::new(self.root, self.core, entry.value()) })
    }
}

impl<T: fmt::Debug + BorrowedPOD> fmt::Debug for IndexedListRef<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl fmt::Debug for IndexedListRef<'_, IndexedList> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

// -------------------------------------------------------------------------------------------------

/// An NBT list tag with a mutable reference to its data.
#[derive(PartialEq, Eq)]
pub struct IndexedListMut<'data, T> {
    root: &'data mut [u8],
    core: &'data IndexedCore,
    index: usize,
    _phantom: PhantomData<T>,
}

impl<'data, T> IndexedListMut<'data, T> {
    /// Create a new [`IndexedListMut`] with the given core and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `index` is a valid index of `core`.
    #[must_use]
    pub(super) const unsafe fn new(
        root: &'data mut [u8],
        core: &'data IndexedCore,
        index: usize,
    ) -> Self {
        Self { root, core, index, _phantom: PhantomData }
    }

    /// Get this [`IndexedListMut`] as an [`IndexedListRef`].
    #[inline]
    #[must_use]
    pub fn as_ref(&self) -> IndexedListRef<'_, T> {
        // SAFETY: `index` is always guaranteed to be within bounds
        unsafe { IndexedListRef::new(self.root, self.core, self.index) }
    }

    /// Convert this [`IndexedListMut`] into an [`IndexedListRef`].
    #[inline]
    #[must_use]
    pub fn into_ref(self) -> IndexedListRef<'data, T> {
        // SAFETY: `index` is always guaranteed to be within bounds
        unsafe { IndexedListRef::new(self.root, self.core, self.index) }
    }
}

impl<T: BorrowedPOD> IndexedListMut<'_, T> {
    /// Get the number of entries in this [`IndexedListMut`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        unsafe { BorrowedRef::new(self.root, BorrowedIndex::<[u8]>::new(self.index)).len() }
    }

    /// Returns `true` if the list is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get the value at the provided index, if it exists.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get_value(&self, index: usize) -> Option<T> {
        // SAFETY: `index` and `range` are always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };
        let entries = unsafe { self.core.entries().get_unchecked(*range) };

        entries.get(index).map(|entry| {
            // SAFETY: `index` is valid for `core` and is of type `T`
            unsafe {
                BorrowedRef::new(self.root, BorrowedIndex::new(entry.value().index())).get_value()
            }
        })
    }

    /// Set the value at the provided index, if it exists.
    ///
    /// Returns `true` if the value was successfully set,
    /// or `false` if the index is out of bounds.
    pub fn set_value(&mut self, index: usize, value: T) -> bool {
        // SAFETY: `index` and `range` are always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };
        let entries = unsafe { self.core.entries().get_unchecked(*range) };

        if let Some(entry) = entries.get(index) {
            // SAFETY: `index` is valid for `core` and is of type `T`
            unsafe {
                BorrowedMut::new(self.root, BorrowedIndex::new(entry.value().index()))
                    .set_value(value);
            }
            true
        } else {
            false
        }
    }
}

impl IndexedListMut<'_, IndexedList> {
    /// Get the number of entries in this [`IndexedListMut`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        // SAFETY: `index` is always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };

        range.end.saturating_sub(range.start)
    }

    /// Returns `true` if the list is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get the value at the provided index, if it exists.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<BorrowedValueRef<'_>> {
        // SAFETY: `index` and `range` are always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };
        let entries = unsafe { self.core.entries().get_unchecked(*range) };

        entries
            .get(index)
            .map(|entry| unsafe { BorrowedValueRef::new(self.root, self.core, entry.value()) })
    }

    /// Get the value at the provided index mutably, if it exists.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<BorrowedValueMut<'_>> {
        // SAFETY: `index` and `range` are always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };
        let entries = unsafe { self.core.entries().get_unchecked(*range) };

        entries
            .get(index)
            .map(|entry| unsafe { BorrowedValueMut::new(self.root, self.core, entry.value()) })
    }
}

impl<T: fmt::Debug + BorrowedPOD> fmt::Debug for IndexedListMut<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(&self.as_ref(), f) }
}

impl fmt::Debug for IndexedListMut<'_, IndexedList> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(&self.as_ref(), f) }
}
