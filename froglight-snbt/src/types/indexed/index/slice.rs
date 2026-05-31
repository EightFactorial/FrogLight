//! TODO

use crate::types::indexed::{
    core::IndexCore,
    entry::EntryIndex,
    index::{Index, Indexable, IndexableSlice, numeric::Integer},
};

impl Indexable for [Integer] {
    type Description = ();
}

impl IndexableSlice for [Integer] {
    #[inline]
    unsafe fn read_entries<C: IndexCore>(index: Index<Self>, core: &C) -> &[EntryIndex] {
        // SAFETY: The caller ensures that this is safe.
        unsafe { core.get_entries(index.range) }
    }
}
