//! TODO

use alloc::vec::Vec;
use core::range::Range;

#[cfg(feature = "froglight-facet")]
#[allow(clippy::wildcard_imports, reason = "Readability")]
use froglight_facet::facet::template::*;

use crate::types::indexed::{
    core::{IndexCore, Mut, NbtAccess, Ref},
    index::EntryIndex,
};

/// An [`IndexCore`] for borrowed NBT data.
#[derive(Debug, PartialEq, Eq)]
pub struct SliceCore<'data, A: NbtAccess> {
    pub(crate) root: A::SLICE<'data>,
    pub(crate) entries: Vec<EntryIndex>,
    pub(crate) ranges: Vec<Range<usize>>,
}

impl<'data, A: NbtAccess> SliceCore<'data, A> {
    /// Create a new [`SliceCore`] with the given NBT slice, entries, and
    /// ranges.
    ///
    /// # Safety
    ///
    /// TODO
    #[inline]
    #[must_use]
    pub const unsafe fn new(
        root: A::SLICE<'data>,
        entries: Vec<EntryIndex>,
        ranges: Vec<Range<usize>>,
    ) -> Self {
        Self { root, entries, ranges }
    }
}

impl IndexCore<Ref> for SliceCore<'_, Ref> {
    #[inline]
    fn root(&self) -> &[u8] { self.root }

    #[inline]
    fn entries(&self) -> &[EntryIndex] { self.entries.as_slice() }

    #[inline]
    unsafe fn entry_range(&self, index: usize) -> &[EntryIndex] {
        // SAFETY: The caller ensures that this is safe.
        unsafe {
            let range = self.ranges.get_unchecked(index);
            self.entries.as_slice().get_unchecked(*range)
        }
    }

    #[inline]
    fn root_mut(&mut self) -> &mut [u8]
    where
        Ref: for<'a> NbtAccess<SLICE<'a> = &'a mut [u8]>,
    {
        unreachable!("Cannot get mutable access with `Ref`!")
    }

    #[inline]
    unsafe fn entries_mut(&mut self) -> &mut [EntryIndex]
    where
        Ref: for<'a> NbtAccess<SLICE<'a> = &'a mut [u8]>,
    {
        unreachable!("Cannot get mutable access with `Ref`!")
    }

    #[inline]
    unsafe fn entry_range_mut(&mut self, _index: usize) -> &mut [EntryIndex]
    where
        Ref: for<'a> NbtAccess<SLICE<'a> = &'a mut [u8]>,
    {
        unreachable!("Cannot get mutable access with `Ref`!")
    }

    #[cfg(feature = "froglight-facet")]
    fn serialize_unnamed(
        item: SerializeItem<'_, '_>,
        writer: &mut Writer<'_>,
    ) -> Result<(), WriterError> {
        use crate::prelude::IndexedNbt;

        let nbt = item.get::<IndexedNbt<'_, Ref, SliceCore<'_, Ref>>>()?;

        writer.write_bytes(nbt.as_slice())
    }

    #[cfg(feature = "froglight-facet")]
    fn serialize_named(
        item: SerializeItem<'_, '_>,
        writer: &mut Writer<'_>,
    ) -> Result<(), WriterError> {
        use crate::prelude::IndexedNbt;

        let nbt = item.get::<IndexedNbt<'_, Ref, SliceCore<'_, Ref>>>()?;

        writer.write_bytes(nbt.as_slice())
    }
}

impl IndexCore<Mut> for SliceCore<'_, Mut> {
    #[inline]
    fn root(&self) -> &[u8] { self.root }

    #[inline]
    fn entries(&self) -> &[EntryIndex] { &self.entries }

    #[inline]
    unsafe fn entry_range(&self, index: usize) -> &[EntryIndex] {
        unsafe {
            let range = self.ranges.get_unchecked(index);
            self.entries.as_slice().get_unchecked(*range)
        }
    }

    fn root_mut(&mut self) -> <Mut as NbtAccess>::SLICE<'_>
    where
        Mut: for<'a> NbtAccess<SLICE<'a> = &'a mut [u8]>,
    {
        &mut self.root
    }

    #[inline]
    unsafe fn entries_mut(&mut self) -> &mut [EntryIndex]
    where
        Mut: for<'a> NbtAccess<SLICE<'a> = &'a mut [u8]>,
    {
        &mut self.entries
    }

    #[inline]
    unsafe fn entry_range_mut(&mut self, index: usize) -> &mut [EntryIndex]
    where
        Mut: for<'a> NbtAccess<SLICE<'a> = &'a mut [u8]>,
    {
        // SAFETY: The caller ensures that this is safe.
        unsafe {
            let range = self.ranges.get_unchecked(index);
            self.entries.as_mut_slice().get_unchecked_mut(*range)
        }
    }

    #[cfg(feature = "froglight-facet")]
    fn serialize_unnamed(
        item: SerializeItem<'_, '_>,
        writer: &mut Writer<'_>,
    ) -> Result<(), WriterError> {
        use crate::prelude::IndexedNbt;

        let nbt = item.get::<IndexedNbt<'_, Mut, SliceCore<'_, Mut>>>()?;

        writer.write_bytes(nbt.as_slice())
    }

    #[cfg(feature = "froglight-facet")]
    fn serialize_named(
        item: SerializeItem<'_, '_>,
        writer: &mut Writer<'_>,
    ) -> Result<(), WriterError> {
        use crate::prelude::IndexedNbt;

        let nbt = item.get::<IndexedNbt<'_, Mut, SliceCore<'_, Mut>>>()?;

        writer.write_bytes(nbt.as_slice())
    }
}
