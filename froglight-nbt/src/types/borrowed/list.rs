//! TODO
#![expect(dead_code, reason = "WIP")]

use core::{marker::PhantomData, range::Range};

use crate::types::borrowed::{
    IndexedCoreMut, IndexedCoreRef,
    reference::{BorrowedIndex, BorrowedMut, BorrowedPOD, BorrowedRef},
};

/// An NBT list tag with a reference to its data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedListRef<'data, T> {
    core: IndexedCoreRef<'data>,
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
    pub(super) const unsafe fn new(core: IndexedCoreRef<'data>, index: usize) -> Self {
        Self { core, index, _phantom: PhantomData }
    }

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
}

impl<T: BorrowedPOD> IndexedListRef<'_, T> {
    /// Get the value at the provided index, if it exists.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get_value(&self, index: usize) -> Option<T> {
        // SAFETY: `index` is always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };
        let normalized = Range { start: 0, end: range.end.saturating_sub(range.start) };

        if normalized.contains(&index) {
            let data_index = range.start + (core::mem::size_of::<T>() * index);
            Some(unsafe {
                BorrowedRef::new(self.core.root(), BorrowedIndex::new(data_index)).get_value()
            })
        } else {
            None
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// An NBT list tag with a mutable reference to its data.
#[derive(Debug, PartialEq, Eq)]
pub struct IndexedListMut<'data, T> {
    core: IndexedCoreMut<'data>,
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
    pub(super) const unsafe fn new(core: IndexedCoreMut<'data>, index: usize) -> Self {
        Self { core, index, _phantom: PhantomData }
    }

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
}

impl<T: BorrowedPOD> IndexedListMut<'_, T> {
    /// Get the value at the provided index, if it exists.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get_value(&self, index: usize) -> Option<T> {
        // SAFETY: `index` is always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };
        let normalized = Range { start: 0, end: range.end.saturating_sub(range.start) };

        if normalized.contains(&index) {
            let data_index = range.start + (core::mem::size_of::<T>() * index);
            Some(unsafe {
                BorrowedRef::new(self.core.root(), BorrowedIndex::new(data_index)).get_value()
            })
        } else {
            None
        }
    }

    /// Set the value at the provided index, if it exists.
    ///
    /// Returns `true` if the value was successfully set,
    /// or `false` if the index is out of bounds.
    pub fn set_value(&mut self, index: usize, value: T) -> bool {
        // SAFETY: `index` is always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };
        let normalized = Range { start: 0, end: range.end.saturating_sub(range.start) };

        if normalized.contains(&index) {
            let data_index = range.start + (core::mem::size_of::<T>() * index);
            unsafe {
                BorrowedMut::new(self.core.root_mut(), BorrowedIndex::new(data_index))
                    .set_value(value);
            }
            true
        } else {
            false
        }
    }
}
