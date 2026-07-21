//! TODO

use alloc::{borrow::Cow, vec::Vec};
use core::range::Range;

#[cfg(feature = "froglight-facet")]
use froglight_facet::facet::prelude::*;

use crate::types::indexed::{
    core::{IndexCore, IndexCored, Mut, NbtAccess, Ref},
    index::EntryIndex,
};

/// An [`IndexCore`] for Copy-On-Write NBT data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CowCore<'data> {
    pub(crate) root: Cow<'data, [u8]>,
    pub(crate) entries: Vec<EntryIndex>,
    pub(crate) ranges: Vec<Range<usize>>,
}

impl<'data> CowCore<'data> {
    /// Create a new [`CowCore`] with the given NBT slice, entries, and ranges.
    ///
    /// # Safety
    ///
    /// TODO
    #[inline]
    #[must_use]
    pub const unsafe fn new(
        root: Cow<'data, [u8]>,
        entries: Vec<EntryIndex>,
        ranges: Vec<Range<usize>>,
    ) -> Self {
        Self { root, entries, ranges }
    }

    /// Convert this [`CowCore`] into an owned version with a `'static`
    /// lifetime.
    #[inline]
    #[must_use]
    pub fn into_owned(self) -> CowCore<'static> {
        CowCore {
            root: Cow::Owned(self.root.into_owned()),
            entries: self.entries,
            ranges: self.ranges,
        }
    }
}

impl<'data> CowCore<'data> {
    /// Create a new [`CowCore`] from the given [`SliceCore`](super::SliceCore).
    #[inline]
    #[must_use]
    pub fn from_slice<A: NbtAccess>(core: super::SliceCore<'data, A>) -> Self {
        unsafe { Self::new(Cow::Borrowed(A::into_ref(core.root)), core.entries, core.ranges) }
    }

    /// Reborrow this [`CowCore`] as a [`SliceCore`](super::SliceCore).
    ///
    /// # Note
    ///
    /// This still clones the entry and range lists.
    #[must_use]
    pub fn reborrow_ref(&self) -> super::SliceCore<'_, Ref> {
        unsafe {
            super::SliceCore::new(self.root.as_ref(), self.entries.clone(), self.ranges.clone())
        }
    }

    /// Reborrow this [`CowCore`] as a [`SliceCore`](super::SliceCore).
    ///
    /// # Note
    ///
    /// This still clones the entry and range lists.
    #[must_use]
    pub fn reborrow_mut(&mut self) -> super::SliceCore<'_, Mut> {
        unsafe {
            super::SliceCore::new(
                self.root.to_mut().as_mut_slice(),
                self.entries.clone(),
                self.ranges.clone(),
            )
        }
    }
}

impl IndexCore<Mut> for CowCore<'_> {
    #[inline]
    fn root(&self) -> &[u8] { self.root.as_ref() }

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
        self.root.to_mut()
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
}

impl IndexCored for CowCore<'_> {
    #[cfg(feature = "froglight-facet")]
    fn deserialize_named<'facet, const BORROW: bool>(
        item: DeserializeItem<'facet, BORROW>,
        reader: &mut Reader<'_>,
    ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
        use crate::{prelude::IndexedNbt, types::indexed::alloc::SliceCore};

        let nbt = IndexedNbt::<SliceCore<'_, Ref>>::new_named(reader.remaining())
            .map_err(|()| ReaderError::from_string("Failed to read IndexedNbt".into()))?;
        reader.consume(nbt.as_slice().len())?;

        item.set(nbt.into_owned())
    }

    #[cfg(feature = "froglight-facet")]
    fn deserialize_unnamed<'facet, const BORROW: bool>(
        item: DeserializeItem<'facet, BORROW>,
        reader: &mut Reader<'_>,
    ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
        use crate::{prelude::IndexedNbt, types::indexed::alloc::SliceCore};

        let nbt = IndexedNbt::<SliceCore<'_, Ref>>::new_unnamed(reader.remaining())
            .map_err(|()| ReaderError::from_string("Failed to read IndexedNbt".into()))?;
        reader.consume(nbt.as_slice().len())?;

        item.set(nbt.into_owned())
    }

    // TODO: Handle skipping the name if there is one.
    #[cfg(feature = "froglight-facet")]
    fn serialize_unnamed(
        item: SerializeItem<'_, '_>,
        writer: &mut Writer<'_>,
    ) -> Result<(), WriterError> {
        use crate::prelude::IndexedNbt;

        let nbt = item.get::<IndexedNbt<CowCore<'_>>>()?;

        writer.write_bytes(nbt.as_slice())
    }

    // TODO: Handle if the item is unnamed.
    #[cfg(feature = "froglight-facet")]
    fn serialize_named(
        item: SerializeItem<'_, '_>,
        writer: &mut Writer<'_>,
    ) -> Result<(), WriterError> {
        use crate::prelude::IndexedNbt;

        let nbt = item.get::<IndexedNbt<CowCore<'_>>>()?;

        writer.write_bytes(nbt.as_slice())
    }
}
