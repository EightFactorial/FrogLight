//! TODO
#![expect(dead_code, reason = "WIP")]

use ::core::marker::PhantomData;
use froglight_mutf8::prelude::MStr;

pub mod compound;
use compound::IndexedCompound;

pub mod core;
use core::{IndexCore, Mut, NbtAccess, Ref};

pub mod index;
use index::Index;

pub mod list;
pub mod reference;
pub mod value;

cfg_select! {
    feature = "alloc" => {
        /// An indexed NBT structure for borrowed NBT data.
        pub struct IndexedNbt<'data, A: NbtAccess, C: IndexCore<A> + 'data = core::BorrowedCore<'data, A>> {
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

    /// Get the raw NBT data as a byte slice.
    #[inline]
    #[must_use]
    pub fn raw(&self) -> &[u8] { self.core.root() }
}

impl<'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A> + 'data> IndexedNbt<'data, A, C> {
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
