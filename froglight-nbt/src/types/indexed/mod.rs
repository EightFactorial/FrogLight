//! TODO
#![expect(clippy::result_unit_err, reason = "WIP")]

use ::core::marker::PhantomData;
use froglight_mutf8::prelude::MStr;

pub mod alloc;

pub mod compound;
use compound::IndexedCompound;

pub mod core;
use core::{IndexCore, Mut, NbtAccess, Ref};

pub mod entry;

pub mod index;
use index::Index;

pub mod list;

pub mod reference;
use reference::IndexedReference;

pub mod types;

/// An indexed NBT structure for borrowed NBT data.
pub struct IndexedNbt<'data, A: NbtAccess, C: IndexCore<A> + 'data> {
    core: C,
    name: Option<Index<MStr>>,
    _phantom: PhantomData<&'data A>,
}

impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> IndexedNbt<'data, A, C> {
    /// Create a new [`IndexedNbt`] from the given [`IndexCore`].
    ///
    /// # Safety
    ///
    /// TODO
    #[inline]
    #[must_use]
    pub const unsafe fn new_core(core: C, name: Option<Index<MStr>>) -> Self {
        Self { core, name, _phantom: PhantomData }
    }

    /// Get a reference to the underlying [`IndexCore`] of this NBT structure.
    #[inline]
    #[must_use]
    pub const fn core(&self) -> &C { &self.core }

    /// Get a mutable reference to the underlying [`IndexCore`] of this NBT
    /// structure.
    ///
    /// # Safety
    ///
    /// The caller must ensure that core is still valid if it is modified.
    #[inline]
    #[must_use]
    pub const unsafe fn core_mut(&mut self) -> &mut C { &mut self.core }

    /// Get the raw NBT data as a byte slice.
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[u8] { self.core.root() }
}

impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> AsRef<[u8]> for IndexedNbt<'data, A, C> {
    #[inline]
    fn as_ref(&self) -> &[u8] { self.as_slice() }
}

// -------------------------------------------------------------------------------------------------

impl<'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A> + 'data> IndexedNbt<'data, A, C> {
    /// Get the name of this NBT structure, if it has one.
    #[inline]
    #[must_use]
    pub fn name(&self) -> Option<IndexedReference<'_, MStr, Ref>> {
        self.name.as_ref().map(|index| unsafe {
            IndexedReference::<MStr, Ref>::new(<C as IndexCore<Ref>>::root(&self.core), *index)
        })
    }

    /// Get the root [`IndexedCompound`] of this NBT structure.
    #[inline]
    #[must_use]
    pub fn as_compound(&self) -> IndexedCompound<'_, Ref, C> {
        // SAFETY: `0` is always a valid index.
        unsafe { IndexedCompound::new(&self.core, 0) }
    }

    /// Convert this NBT structure into a read-only version.
    #[inline]
    #[must_use]
    pub fn into_ref(self) -> IndexedNbt<'data, Ref, C> {
        // SAFETY: `IndexedNbt` is still valid.
        unsafe { IndexedNbt::new_core(self.core, self.name) }
    }
}

impl<'data, A: NbtAccess, C: IndexCore<Mut> + IndexCore<A> + 'data> IndexedNbt<'data, A, C> {
    /// Get the root [`IndexedCompound`] of this NBT structure.
    #[inline]
    #[must_use]
    pub fn as_compound_mut(&mut self) -> IndexedCompound<'_, Mut, C> {
        // SAFETY: `0` is always a valid index.
        unsafe { IndexedCompound::new(&mut self.core, 0) }
    }

    /// Get the raw NBT data as a mutable byte slice.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the slice is still valid for it's core.
    #[inline]
    #[must_use]
    pub unsafe fn as_slice_mut(&mut self) -> &mut [u8] {
        <C as IndexCore<Mut>>::root_mut(&mut self.core)
    }

    /// Convert this NBT structure into a mutable version.
    #[inline]
    #[must_use]
    pub fn into_mut(self) -> IndexedNbt<'data, Mut, C> {
        // SAFETY: `IndexedNbt` is still valid.
        unsafe { IndexedNbt::new_core(self.core, self.name) }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'data> IndexedNbt<'data, Ref, alloc::SliceCore<'data, Ref>> {
    /// Parse an unnamed NBT structure from the given byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte slice is not valid NBT data.
    #[inline]
    pub fn new_unnamed_ref(data: &'data [u8]) -> Result<Self, ()> {
        alloc::parse::parse_nbt_ref(data, false)
    }

    /// Parse a named NBT structure from the given byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte slice is not valid NBT data.
    #[inline]
    pub fn new_named_ref(data: &'data [u8]) -> Result<Self, ()> {
        alloc::parse::parse_nbt_ref(data, true)
    }

    /// Take ownership of the underlying NBT data,
    /// returning a new [`IndexedNbt`] with an owned core.
    #[must_use]
    pub fn into_owned_ref(self) -> IndexedNbt<'static, Ref, alloc::CowCore<'static, Ref>> {
        // SAFETY: `self.core` is valid for the lifetime of `self`.
        unsafe {
            let core = alloc::CowCore::from_slice_ref(self.core).into_owned();
            IndexedNbt::<Ref, alloc::CowCore<'static, Ref>>::new_core(core, self.name)
        }
    }
}

impl<'data> IndexedNbt<'data, Mut, alloc::SliceCore<'data, Mut>> {
    /// Parse an unnamed NBT structure from the given byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte slice is not valid NBT data.
    #[inline]
    pub fn new_unnamed_mut(data: &'data mut [u8]) -> Result<Self, ()> {
        alloc::parse::parse_nbt_mut(data, false)
    }

    /// Parse a named NBT structure from the given byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte slice is not valid NBT data.
    #[inline]
    pub fn new_named_mut(data: &'data mut [u8]) -> Result<Self, ()> {
        alloc::parse::parse_nbt_mut(data, true)
    }

    /// Take ownership of the underlying NBT data,
    /// returning a new [`IndexedNbt`] with an owned core.
    #[must_use]
    pub fn into_owned_mut(self) -> IndexedNbt<'static, Mut, alloc::CowCore<'static, Mut>> {
        // SAFETY: `self.core` is valid for the lifetime of `self`.
        unsafe {
            let core = alloc::CowCore::from_slice_mut(self.core).into_owned();
            IndexedNbt::<Mut, alloc::CowCore<'static, Mut>>::new_core(core, self.name)
        }
    }
}
