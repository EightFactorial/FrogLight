//! TODO
#![expect(clippy::result_unit_err, reason = "WIP")]

use ::core::{borrow::Borrow, fmt};
#[cfg(feature = "froglight-facet")]
use froglight_facet::{self as mc, facet::WithFnAttr};
use froglight_mutf8::prelude::MStr;

pub mod compound;
use compound::IndexedCompound;

pub mod core;
use core::{CowCore, IndexCore, IndexCored, Mut, NbtAccess, Ref, SliceCore};

pub mod entry;

pub mod index;
use index::Index;

pub mod list;

pub mod reference;
use reference::{IndexedReference, ValueReference};

mod parse;

pub mod types;

/// An indexed NBT structure for borrowed NBT data.
///
/// # Note
///
/// If the `froglight-facet` feature is enabled, by default the
/// [`IndexedNbt::WITH_UNNAMED`] ser/de functions will be used.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "facet", derive(facet::Facet), facet(opaque))]
#[cfg_attr(feature = "froglight-facet", facet(mc::with = Self::WITH_UNNAMED))]
pub struct IndexedNbt<C: IndexCored> {
    core: C,
    name: Option<Index<MStr>>,
}

impl<C: IndexCored> IndexedNbt<C> {
    /// A [`WithFnAttr`] for reading named NBT.
    ///
    /// See [`FacetTemplate`](froglight_facet::facet::FacetTemplate) for how
    /// this is used.
    #[cfg(feature = "froglight-facet")]
    pub const WITH_NAMED: WithFnAttr = C::WITH_NAMED;
    /// A [`WithFnAttr`] for reading optional named NBT.
    ///
    /// See [`FacetTemplate`](froglight_facet::facet::FacetTemplate) for how
    /// this is used.
    #[cfg(feature = "froglight-facet")]
    pub const WITH_OPT_NAMED: WithFnAttr = froglight_facet::option_with!(Self::WITH_NAMED);
    /// A [`WithFnAttr`] for reading optional unnamed NBT.
    ///
    /// See [`FacetTemplate`](froglight_facet::facet::FacetTemplate) for how
    /// this is used.
    #[cfg(feature = "froglight-facet")]
    pub const WITH_OPT_UNNAMED: WithFnAttr = froglight_facet::option_with!(Self::WITH_UNNAMED);
    /// A [`WithFnAttr`] for reading punnamed NBT.
    ///
    /// See [`FacetTemplate`](froglight_facet::facet::FacetTemplate) for how
    /// this is used.
    #[cfg(feature = "froglight-facet")]
    pub const WITH_UNNAMED: WithFnAttr = C::WITH_UNNAMED;

    /// Create a new [`IndexedNbt`] from the given [`IndexCore`].
    ///
    /// # Safety
    ///
    /// TODO
    #[inline]
    #[must_use]
    pub const unsafe fn new_core(core: C, name: Option<Index<MStr>>) -> Self { Self { core, name } }

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
    pub fn as_slice(&self) -> &[u8]
    where
        C: IndexCore<Ref>,
    {
        self.core.root()
    }
}

// -------------------------------------------------------------------------------------------------

impl<C: IndexCored> IndexedNbt<C> {
    /// Get the name of this NBT structure, if it has one.
    #[inline]
    #[must_use]
    pub fn name(&self) -> Option<IndexedReference<'_, MStr, Ref>>
    where
        C: IndexCore<Ref>,
    {
        self.name.as_ref().map(|index| unsafe {
            IndexedReference::<MStr, Ref>::new(<C as IndexCore<Ref>>::root(&self.core), *index)
        })
    }

    /// Get the root [`IndexedCompound`] of this NBT structure.
    #[inline]
    #[must_use]
    pub fn as_compound(&self) -> IndexedCompound<'_, Ref, C>
    where
        C: IndexCore<Ref>,
    {
        // SAFETY: `0` is always a valid index.
        unsafe { IndexedCompound::new(&self.core, 0) }
    }

    /// Get the root [`IndexedCompound`] of this NBT structure.
    #[inline]
    #[must_use]
    pub fn as_compound_mut(&mut self) -> IndexedCompound<'_, Mut, C>
    where
        C: IndexCore<Mut>,
    {
        // SAFETY: `0` is always a valid index.
        unsafe { IndexedCompound::new(&mut self.core, 0) }
    }

    /// Get the root [`IndexedValueReference`] of this NBT structure.
    ///
    /// # Note
    ///
    /// This is always a [`IndexedValueReference::Compound`].
    #[inline]
    #[must_use]
    pub fn as_value(&self) -> ValueReference<'_, Ref, C>
    where
        C: IndexCore<Ref>,
    {
        ValueReference::Compound(self.as_compound())
    }

    /// Get the root [`IndexedValueReference`] of this NBT structure.
    ///
    /// # Note
    ///
    /// This is always a [`IndexedValueReference::Compound`].
    #[inline]
    #[must_use]
    pub fn as_value_mut(&mut self) -> ValueReference<'_, Mut, C>
    where
        C: IndexCore<Mut>,
    {
        ValueReference::Compound(self.as_compound_mut())
    }

    /// Get the raw NBT data as a mutable byte slice.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the slice is still valid for it's core.
    #[inline]
    #[must_use]
    pub unsafe fn as_slice_mut(&mut self) -> &mut [u8]
    where
        C: IndexCore<Mut>,
    {
        <C as IndexCore<Mut>>::root_mut(&mut self.core)
    }
}

// -------------------------------------------------------------------------------------------------

impl<'data, A: NbtAccess> IndexedNbt<SliceCore<'data, A>>
where
    SliceCore<'data, A>: IndexCored,
{
    /// Parse an unnamed NBT structure from the given byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte slice is not valid NBT data.
    #[inline]
    pub fn new_unnamed(data: A::SLICE<'data>) -> Result<Self, ()> { parse::parse_nbt(data, false) }

    /// Parse a named NBT structure from the given byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte slice is not valid NBT data.
    #[inline]
    pub fn new_named(data: A::SLICE<'data>) -> Result<Self, ()> { parse::parse_nbt(data, true) }

    /// Convert this [`IndexedNbt`]'s core into a borrowed core.
    #[must_use]
    pub fn into_ref(self) -> IndexedNbt<SliceCore<'data, Ref>> {
        unsafe { IndexedNbt::new_core(self.core.into_ref(), self.name) }
    }

    /// Take ownership of the underlying NBT data,
    /// returning a new [`IndexedNbt`] with a [`CowCore`] core.
    #[must_use]
    pub fn into_cow(self) -> IndexedNbt<CowCore<'data>> {
        // SAFETY: `self.core` is valid for the lifetime of `self`.
        unsafe {
            let core = CowCore::from_slice(self.core);
            IndexedNbt::<CowCore<'data>>::new_core(core, self.name)
        }
    }

    /// Take ownership of the underlying NBT data,
    /// returning a new [`IndexedNbt`] with an owned [`CowCore`] core.
    #[must_use]
    pub fn into_owned(self) -> IndexedNbt<CowCore<'static>> {
        // SAFETY: `self.core` is valid for the lifetime of `self`.
        unsafe {
            let core = CowCore::from_slice(self.core).into_owned();
            IndexedNbt::<CowCore<'static>>::new_core(core, self.name)
        }
    }
}

impl IndexedNbt<SliceCore<'_, Ref>> {
    /// Create a new empty NBT structure.
    #[must_use]
    pub fn empty_slice() -> Self {
        use ::alloc::vec;
        use ::core::range::Range;
        use index::{EntryIndex, ValueIndex};

        unsafe {
            IndexedNbt::new_core(
                SliceCore::new(
                    [10u8, 0u8].as_slice(),
                    vec![EntryIndex::new(Index::new(0), ValueIndex::Compound(Index::new(0)))],
                    vec![Range { start: 0, end: 0 }],
                ),
                None,
            )
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl IndexedNbt<CowCore<'_>> {
    /// Create a new empty NBT structure.
    #[inline]
    #[must_use]
    pub fn empty_cow() -> Self {
        use ::core::range::Range;

        use crate::types::indexed::index::{EntryIndex, ValueIndex};

        unsafe {
            IndexedNbt::new_core(
                CowCore::new(
                    ::alloc::borrow::Cow::Borrowed([10u8, 0u8].as_slice()),
                    ::alloc::vec![EntryIndex::new(
                        Index::new(0),
                        ValueIndex::Compound(Index::new(0))
                    )],
                    ::alloc::vec![Range { start: 0, end: 0 }],
                ),
                None,
            )
        }
    }

    /// Take ownership of the underlying NBT data,
    /// returning a new [`IndexedNbt`] with an owned [`CowCore`] core.
    #[must_use]
    pub fn into_owned(self) -> IndexedNbt<CowCore<'static>> {
        unsafe { IndexedNbt::new_core(self.core.into_owned(), self.name) }
    }
}

// -------------------------------------------------------------------------------------------------

impl<C: IndexCore<Ref>> fmt::Debug for IndexedNbt<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ::alloc::borrow::Cow;
        let name = self.name().map_or(Cow::Borrowed(""), |str| str.get().to_utf8());
        f.debug_map().entry(&name.as_ref(), &self.as_compound()).finish()
    }
}

impl Default for IndexedNbt<SliceCore<'_, Ref>> {
    #[inline]
    fn default() -> Self { Self::empty_slice() }
}
impl Default for IndexedNbt<CowCore<'_>> {
    #[inline]
    fn default() -> Self { Self::empty_cow() }
}

impl<C: IndexCore<Ref>> PartialEq for IndexedNbt<C> {
    fn eq(&self, other: &Self) -> bool { self.as_slice() == other.as_slice() }
}
impl<C: IndexCore<Ref>> Eq for IndexedNbt<C> {}

impl<C: IndexCore<Ref>> AsRef<[u8]> for IndexedNbt<C> {
    #[inline]
    fn as_ref(&self) -> &[u8] { self.as_slice() }
}
impl<C: IndexCore<Ref>> Borrow<[u8]> for IndexedNbt<C> {
    #[inline]
    fn borrow(&self) -> &[u8] { self.as_slice() }
}
