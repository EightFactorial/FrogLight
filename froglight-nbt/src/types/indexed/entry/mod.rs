//! TODO

use froglight_mutf8::prelude::MStr;

use crate::types::indexed::{
    core::{IndexCore, Mut, NbtAccess, Ref},
    index::{EntryIndex, Index, ValueIndex},
    reference::IndexedReference,
};

pub(super) mod value;

/// An NBT entry that is indexed by an [`IndexCore`].
pub struct IndexedEntry<'data, A: NbtAccess, C: IndexCore<A> + 'data> {
    core: A::CORE<'data, C>,
    index: EntryIndex,
}

/// An NBT value that is indexed by an [`IndexCore`].
pub struct IndexedValue<'data, A: NbtAccess, C: IndexCore<A> + 'data> {
    core: A::CORE<'data, C>,
    index: ValueIndex,
}

impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> IndexedEntry<'data, A, C> {
    /// Create a new [`IndexedEntry`] from the given core and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `index` is a valid index for `core`.
    #[inline]
    pub unsafe fn new(core: A::CORE<'data, C>, index: EntryIndex) -> Self { Self { core, index } }

    /// Create a new [`IndexedEntry`] from the given core and name/value index
    /// pair.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the indexes in the pair are valid for
    /// `core`.
    #[inline]
    #[must_use]
    pub unsafe fn new_pair(core: A::CORE<'data, C>, name: Index<MStr>, value: ValueIndex) -> Self {
        // SAFETY: The caller ensures this is safe.
        unsafe { Self::new(core, EntryIndex::new(name, value)) }
    }
}

impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> IndexedValue<'data, A, C> {
    /// Create a new [`IndexedValue`] from the given core and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `index` is a valid index for `core`.
    #[inline]
    pub unsafe fn new(core: A::CORE<'data, C>, index: ValueIndex) -> Self { Self { core, index } }
}

// -------------------------------------------------------------------------------------------------

impl<'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A> + 'data> IndexedEntry<'data, A, C> {
    /// Get the name of this entry.
    #[inline]
    #[must_use]
    pub fn name(&self) -> IndexedReference<'_, MStr, Ref> {
        let root = <C as IndexCore<A>>::root(&self.core);
        // SAFETY: `IndexedValue` ensures this is safe.
        unsafe { IndexedReference::new(root, self.index.name()) }
    }

    /// Get the [`IndexedValue`] of this entry.
    #[inline]
    #[must_use]
    pub fn value(&self) -> IndexedValue<'_, Ref, C> {
        // SAFETY: `IndexedEntry` ensures this is safe.
        unsafe { IndexedValue::<Ref, C>::new(&self.core, self.index.value()) }
    }
}

impl<'data, C: IndexCore<Mut> + 'data> IndexedEntry<'data, Mut, C> {
    /// Get the [`IndexedValue`] of this entry.
    #[inline]
    #[must_use]
    pub fn value_mut(&mut self) -> IndexedValue<'_, Mut, C> {
        // SAFETY: `IndexedEntry` ensures this is safe.
        unsafe { IndexedValue::<Mut, C>::new(self.core, self.index.value()) }
    }
}
