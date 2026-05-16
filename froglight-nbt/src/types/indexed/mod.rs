//! TODO

use ::core::marker::PhantomData;
use froglight_mutf8::prelude::MStr;

#[cfg(feature = "alloc")]
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

cfg_select! {
    feature = "alloc" => {
        /// An indexed NBT structure for borrowed NBT data.
        pub struct IndexedNbt<'data, A: NbtAccess, C: IndexCore<A> + 'data = alloc::SliceCore<'data, A>> {
            core: C,
            name: Option<Index<MStr>>,
            _phantom: PhantomData<(&'data (), A)>,
        }
    }
    _ => {
        /// An indexed NBT structure for borrowed NBT data.
        pub struct IndexedNbt<'data, A: NbtAccess, C: IndexCore<A> + 'data> {
            core: C,
            name: Option<Index<MStr>>,
            _phantom: PhantomData<(&'data (), A)>,
        }
    }
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

    /// Get the raw NBT data as a byte slice.
    #[inline]
    #[must_use]
    pub fn raw(&self) -> &[u8] { self.core.root() }
}

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
}

impl<'data, A: NbtAccess, C: IndexCore<Mut> + IndexCore<A> + 'data> IndexedNbt<'data, A, C> {
    /// Get the root [`IndexedCompound`] of this NBT structure.
    #[inline]
    #[must_use]
    pub fn as_compound_mut(&mut self) -> IndexedCompound<'_, Mut, C> {
        // SAFETY: `0` is always a valid index.
        unsafe { IndexedCompound::new(&mut self.core, 0) }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "alloc")]
impl<'data> IndexedNbt<'data, Ref, alloc::SliceCore<'data, Ref>> {
    /// Parse an unnamed NBT structure from the given byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte slice is not valid NBT data.
    #[inline]
    #[expect(clippy::result_unit_err, reason = "WIP")]
    pub fn new_unnamed_ref(data: &'data [u8]) -> Result<Self, ()> {
        alloc::parse::parse_nbt_ref(data, false)
    }

    /// Parse a named NBT structure from the given byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte slice is not valid NBT data.
    #[inline]
    #[expect(clippy::result_unit_err, reason = "WIP")]
    pub fn new_named_ref(data: &'data [u8]) -> Result<Self, ()> {
        alloc::parse::parse_nbt_ref(data, true)
    }
}

#[cfg(feature = "alloc")]
impl<'data> IndexedNbt<'data, Mut, alloc::SliceCore<'data, Mut>> {
    /// Parse an unnamed NBT structure from the given byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte slice is not valid NBT data.
    #[inline]
    #[expect(clippy::result_unit_err, reason = "WIP")]
    pub fn new_unnamed_mut(data: &'data mut [u8]) -> Result<Self, ()> {
        alloc::parse::parse_nbt_mut(data, false)
    }

    /// Parse a named NBT structure from the given byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte slice is not valid NBT data.
    #[inline]
    #[expect(clippy::result_unit_err, reason = "WIP")]
    pub fn new_named_mut(data: &'data mut [u8]) -> Result<Self, ()> {
        alloc::parse::parse_nbt_mut(data, true)
    }
}
