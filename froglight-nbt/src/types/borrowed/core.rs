//! TODO

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::ops::Deref;
#[cfg(feature = "alloc")]
use core::range::Range;

use crate::types::borrowed::index::EntryIndex;

/// An [`IndexCore`] for borrowed NBT data.
#[cfg(feature = "alloc")]
pub struct BorrowedCore<'data, A: NbtAccess> {
    root: A::SLICE<'data>,
    entries: Vec<EntryIndex>,
    ranges: Vec<Range<usize>>,
}

#[cfg(feature = "alloc")]
impl<'data, A: NbtAccess> BorrowedCore<'data, A> {
    /// Create a new [`BorrowedCore`] with the given NBT slice, entries, and
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

#[cfg(feature = "alloc")]
impl IndexCore<Mut> for BorrowedCore<'_, Mut> {
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
}

// -------------------------------------------------------------------------------------------------

/// A trait for an index of NBT entries.
///
/// If the `alloc` feature is enabled,
/// [`BorrowedCore`] is provided as the default implementation.
pub trait IndexCore<A: NbtAccess> {
    /// Get a reference to the root NBT data slice.
    #[must_use]
    fn root(&self) -> &[u8];

    /// Get a reference to the [`EntryIndex`]es of this NBT structure.
    #[must_use]
    fn entries(&self) -> &[EntryIndex];

    /// Get the [`EntryIndex`]es for the given compound/list index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the given index is valid.
    #[must_use]
    unsafe fn entry_range(&self, index: usize) -> &[EntryIndex];

    /// Get a mutable reference to the root NBT data slice.
    #[must_use]
    fn root_mut(&mut self) -> &mut [u8]
    where
        A: for<'a> NbtAccess<SLICE<'a> = &'a mut [u8]>;

    /// Get a mutable reference to the [`EntryIndex`]es of this NBT structure.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the list of entries is still valid
    /// if it is modified.
    #[must_use]
    unsafe fn entries_mut(&mut self) -> &mut [EntryIndex]
    where
        A: for<'a> NbtAccess<SLICE<'a> = &'a mut [u8]>;

    /// Get a mutable reference to the [`EntryIndex`]es for the given
    /// compound/list index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the given index is valid and that the list
    /// of entries is still valid if it is modified.
    #[must_use]
    unsafe fn entry_range_mut(&mut self, index: usize) -> &mut [EntryIndex]
    where
        A: for<'a> NbtAccess<SLICE<'a> = &'a mut [u8]>;
}

impl<T: IndexCore<Mut> + ?Sized> IndexCore<Ref> for T {
    #[inline]
    fn root(&self) -> &[u8] { self.root() }

    #[inline]
    fn entries(&self) -> &[EntryIndex] { self.entries() }

    #[inline]
    unsafe fn entry_range(&self, index: usize) -> &[EntryIndex] {
        // SAFETY: The caller ensures that this is safe.
        unsafe { self.entry_range(index) }
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

// -------------------------------------------------------------------------------------------------

/// A trait for either [`Ref`] or [`Mut`] access.
pub trait NbtAccess: sealed::Sealed + 'static {
    /// The type of slice that NBT data is accessed through.
    type SLICE<'data>: Deref<Target = [u8]> + 'data;
    /// The type of reference that the core is accessed through.
    type CORE<'a, C: ?Sized + 'a>: Deref<Target = C> + Sized + 'a;
}

/// A marker type for read-only access.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct Ref;
impl NbtAccess for Ref {
    type CORE<'a, C: ?Sized + 'a> = &'a C;
    type SLICE<'data> = &'data [u8];
}

/// A marker type for mutable access.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct Mut;
impl NbtAccess for Mut {
    type CORE<'a, C: ?Sized + 'a> = &'a mut C;
    type SLICE<'data> = &'data mut [u8];
}

// -------------------------------------------------------------------------------------------------

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::Ref {}
    impl Sealed for super::Mut {}
}
