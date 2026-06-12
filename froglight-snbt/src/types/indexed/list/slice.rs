use core::{fmt, marker::PhantomData, range::Range};

use crate::types::indexed::{
    core::IndexCore,
    entry::{EntryIndex, ValueIndex},
    index::{IndexableValue, numeric::IntegerValue},
    list::{IndexedList, SliceIter},
    reference::Referenceable,
};

/// A slice of values indexed by an [`IndexCore`].
pub struct IndexedSlice<'data, C: IndexCore, T: Referenceable>
where
    T::Indexable: IndexableValue,
{
    core: &'data C,
    range: Range<usize>,
    _phantom: PhantomData<T>,
}

impl<'data, C: IndexCore, T: Referenceable + 'static> IndexedSlice<'data, C, T>
where
    T::Indexable: IndexableValue,
    IntegerValue: Into<T::Value<'static>>,
{
    /// Create a new [`IndexedSlice`] from a range and core.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the entry range is valid for the provided
    /// core.
    #[inline]
    #[must_use]
    pub const unsafe fn new(core: &'data C, range: Range<usize>) -> Self {
        Self { core, range, _phantom: PhantomData }
    }

    /// Get the [`EntryIndexes`](EntryIndex) of this list.
    #[inline]
    #[must_use]
    pub(crate) fn entries(&self) -> &[EntryIndex] {
        // SAFETY: `IndexedList` guarantees that this is safe.
        unsafe { self.core.get_entries(self.range) }
    }

    /// Get a value by it's index.
    ///
    /// Returns `None` if the index is out of bounds.
    pub fn get(&self, index: usize) -> Option<T::Value<'static>> {
        self.get_value(index).map(IntegerValue::into)
    }

    /// Get a value by it's index.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get_value(&self, index: usize) -> Option<IntegerValue> {
        self.entries().get(index).map(|entry| match entry.value() {
            ValueIndex::Bool(index) => unsafe {
                IntegerValue::Byte(u8::from(index.read_value(self.core.root()).as_bool()))
            },

            ValueIndex::Byte(index)
            | ValueIndex::Short(index)
            | ValueIndex::Int(index)
            | ValueIndex::Long(index) => unsafe { index.read_value(self.core.root()) },

            #[cfg(debug_assertions)]
            _ => unreachable!("Non-Integer/Bool value in IndexedSlice!"),
            #[cfg(not(debug_assertions))]
            _ => unsafe { core::hint::unreachable_unchecked() },
        })
    }

    /// Create an iterator over this slice.
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> SliceIter<'_, 'data, C, T> { SliceIter::new(self) }

    /// Get this slice as an [`IndexedList`].
    #[inline]
    #[must_use]
    pub const fn as_list(&self) -> IndexedList<'data, C> {
        unsafe { IndexedList::new(self.core, self.range) }
    }

    /// Convert this slice into an [`IndexedList`].
    #[inline]
    #[must_use]
    pub const fn into_list(self) -> IndexedList<'data, C> {
        unsafe { IndexedList::new(self.core, self.range) }
    }
}

// -------------------------------------------------------------------------------------------------

impl<C: IndexCore, T: Referenceable + 'static> fmt::Debug for IndexedSlice<'_, C, T>
where
    for<'a> T::Value<'a>: fmt::Debug,
    T::Indexable: IndexableValue,
    IntegerValue: Into<T::Value<'static>>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<C: IndexCore, T: Referenceable> Clone for IndexedSlice<'_, C, T>
where
    T::Indexable: IndexableValue,
{
    fn clone(&self) -> Self { *self }
}
impl<C: IndexCore, T: Referenceable> Copy for IndexedSlice<'_, C, T> where
    T::Indexable: IndexableValue
{
}

impl<C: IndexCore, T: Referenceable> PartialEq for IndexedSlice<'_, C, T>
where
    T::Indexable: IndexableValue,
{
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range && self.core.root() == other.core.root()
    }
}
impl<C: IndexCore, T: Referenceable> Eq for IndexedSlice<'_, C, T> where T::Indexable: IndexableValue
{}
