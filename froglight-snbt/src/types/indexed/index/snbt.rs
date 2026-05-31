//! TODO

use crate::types::indexed::{
    core::IndexCore,
    entry::EntryIndex,
    index::{Index, Indexable, IndexableSlice},
    types::{IndexedListType, IndexedMapType},
};

impl Indexable for IndexedListType {
    type Description = ();
}

impl IndexableSlice for IndexedListType {
    #[inline]
    unsafe fn read_entries<C: IndexCore>(index: Index<Self>, core: &C) -> &[EntryIndex] {
        // SAFETY: The caller ensures that this is safe.
        unsafe { core.get_entries(index.range) }
    }
}

// -------------------------------------------------------------------------------------------------

impl Indexable for IndexedMapType {
    type Description = ();
}

impl IndexableSlice for IndexedMapType {
    #[inline]
    unsafe fn read_entries<C: IndexCore>(index: Index<Self>, core: &C) -> &[EntryIndex] {
        // SAFETY: The caller ensures that this is safe.
        unsafe { core.get_entries(index.range) }
    }
}
