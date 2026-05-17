//! TODO

use alloc::{borrow::Cow, vec::Vec};
use core::{marker::PhantomData, range::Range};

use crate::types::indexed::{
    core::{IndexCore, Mut, NbtAccess, Ref},
    index::EntryIndex,
};

/// An [`IndexCore`] for Copy-On-Write NBT data.
#[derive(Debug, Clone)]
pub struct CowCore<'data, A: NbtAccess> {
    root: Cow<'data, [u8]>,
    entries: Vec<EntryIndex>,
    ranges: Vec<Range<usize>>,
    _phantom: PhantomData<A>,
}

impl<'data, A: NbtAccess> CowCore<'data, A> {
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
        Self { root, entries, ranges, _phantom: PhantomData }
    }

    /// Convert this [`CowCore`] into an owned version with a `'static`
    /// lifetime.
    #[inline]
    #[must_use]
    pub fn into_owned(self) -> CowCore<'static, A> {
        CowCore {
            root: Cow::Owned(self.root.into_owned()),
            entries: self.entries,
            ranges: self.ranges,
            _phantom: PhantomData,
        }
    }
}

impl<'data> CowCore<'data, Ref> {
    /// Create a new [`CowCore`] from the given [`SliceCore`].
    #[inline]
    #[must_use]
    pub fn from_slice_ref(core: super::SliceCore<'data, Ref>) -> Self {
        unsafe { Self::new(Cow::Borrowed(core.root), core.entries, core.ranges) }
    }
}
impl<'data> CowCore<'data, Mut> {
    /// Create a new [`CowCore`] from the given [`SliceCore`].
    #[inline]
    #[must_use]
    pub fn from_slice_mut(core: super::SliceCore<'data, Mut>) -> Self {
        unsafe { Self::new(Cow::Borrowed(core.root), core.entries, core.ranges) }
    }
}

impl IndexCore<Ref> for CowCore<'_, Ref> {
    #[inline]
    fn root(&self) -> &[u8] { self.root.as_ref() }

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
}

impl IndexCore<Mut> for CowCore<'_, Mut> {
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
