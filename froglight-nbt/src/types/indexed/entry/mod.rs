//! TODO

use core::fmt;

use froglight_mutf8::prelude::MStr;

use crate::types::indexed::{
    core::{IndexCore, Mut, NbtAccess, Ref},
    index::{EntryIndex, Index, ValueIndex},
    reference::IndexedReference,
};

pub(super) mod value;

/// An NBT entry that is indexed by an [`IndexCore`].
pub struct IndexedEntry<'index, A: NbtAccess, C: IndexCore<A> + 'index> {
    core: A::CORE<'index, C>,
    index: EntryIndex,
}

/// An NBT value that is indexed by an [`IndexCore`].
pub struct IndexedValue<'index, A: NbtAccess, C: IndexCore<A> + 'index> {
    core: A::CORE<'index, C>,
    index: ValueIndex,
}

impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> IndexedEntry<'index, A, C> {
    /// Create a new [`IndexedEntry`] from the given core and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `index` is a valid index for `core`.
    #[inline]
    pub const unsafe fn new(core: A::CORE<'index, C>, index: EntryIndex) -> Self {
        Self { core, index }
    }

    /// Create a new [`IndexedEntry`] from the given core and name/value index
    /// pair.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the indexes in the pair are valid for
    /// `core`.
    #[inline]
    #[must_use]
    pub const unsafe fn new_pair(
        core: A::CORE<'index, C>,
        name: Index<MStr>,
        value: ValueIndex,
    ) -> Self {
        // SAFETY: The caller ensures this is safe.
        unsafe { Self::new(core, EntryIndex::new(name, value)) }
    }

    /// Get the name of this entry.
    #[inline]
    #[must_use]
    pub fn name(self) -> IndexedReference<'index, MStr, Ref>
    where
        C: IndexCore<Ref>,
    {
        let root = <C as IndexCore<A>>::root(A::into_core(self.core));
        // SAFETY: `IndexedValue` ensures this is safe.
        unsafe { IndexedReference::new(root, self.index.name()) }
    }

    /// Get a reference to the name of this entry.
    #[inline]
    #[must_use]
    pub fn name_ref(&self) -> IndexedReference<'_, MStr, Ref>
    where
        C: IndexCore<Ref>,
    {
        let root = <C as IndexCore<A>>::root(&self.core);
        // SAFETY: `IndexedValue` ensures this is safe.
        unsafe { IndexedReference::new(root, self.index.name()) }
    }

    /// Get the [`IndexedValue`] of this entry.
    #[inline]
    #[must_use]
    pub fn value(self) -> IndexedValue<'index, A, C> {
        // SAFETY: `IndexedEntry` ensures this is safe.
        unsafe { IndexedValue::<A, C>::new(self.core, self.index.value()) }
    }

    /// Get a reference to the [`IndexedValue`] of this entry.
    #[inline]
    #[must_use]
    pub fn value_ref(&self) -> IndexedValue<'_, Ref, C>
    where
        C: IndexCore<Ref>,
    {
        // SAFETY: `IndexedEntry` ensures this is safe.
        unsafe { IndexedValue::<Ref, C>::new(&self.core, self.index.value()) }
    }

    /// Get the name-value pair of this entry.
    #[inline]
    #[must_use]
    pub fn pair(self) -> (IndexedReference<'index, MStr, Ref>, IndexedValue<'index, Ref, C>)
    where
        C: IndexCore<Ref>,
    {
        let core = A::into_core(self.core);
        let root = <C as IndexCore<A>>::root(core);

        // SAFETY: `IndexedValue` ensures this is safe.
        unsafe {
            (
                IndexedReference::new(root, self.index.name()),
                IndexedValue::<Ref, C>::new(core, self.index.value()),
            )
        }
    }

    /// Get a reference to the name-value pair of this entry.
    #[inline]
    #[must_use]
    pub fn pair_ref(&self) -> (IndexedReference<'_, MStr, Ref>, IndexedValue<'_, Ref, C>)
    where
        C: IndexCore<Ref>,
    {
        let root = <C as IndexCore<A>>::root(&self.core);

        // SAFETY: `IndexedValue` ensures this is safe.
        unsafe {
            (
                IndexedReference::new(root, self.index.name()),
                IndexedValue::<Ref, C>::new(&self.core, self.index.value()),
            )
        }
    }

    /// Convert this [`IndexedEntry`] into one that uses [`Ref`] access.
    #[inline]
    #[must_use]
    pub fn into_ref(self) -> IndexedEntry<'index, Ref, C>
    where
        C: IndexCore<Ref>,
    {
        let core = A::into_core(self.core);
        // SAFETY: `IndexedEntry` ensures this is safe.
        unsafe { IndexedEntry::<Ref, C>::new(core, self.index) }
    }
}

impl<'index, C: IndexCore<Mut> + 'index> IndexedEntry<'index, Mut, C> {
    /// Get the [`IndexedValue`] of this entry.
    #[inline]
    #[must_use]
    pub fn value_mut(&mut self) -> IndexedValue<'_, Mut, C> {
        // SAFETY: `IndexedEntry` ensures this is safe.
        unsafe { IndexedValue::<Mut, C>::new(self.core, self.index.value()) }
    }
}

impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> IndexedValue<'index, A, C> {
    /// Create a new [`IndexedValue`] from the given core and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `index` is a valid index for `core`.
    #[inline]
    pub const unsafe fn new(core: A::CORE<'index, C>, index: ValueIndex) -> Self {
        Self { core, index }
    }
}

// -------------------------------------------------------------------------------------------------

impl<A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> fmt::Debug for IndexedValue<'_, A, C> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_value(), f)
    }
}
impl<A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> fmt::Debug for IndexedEntry<'_, A, C> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (name, value) = self.pair_ref();
        f.debug_struct("IndexedEntry").field("name", &name).field("value", &value).finish()
    }
}

impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> Clone for IndexedEntry<'index, A, C>
where
    <A as NbtAccess>::CORE<'index, C>: Clone,
{
    fn clone(&self) -> Self { Self { core: self.core.clone(), index: self.index } }
}
impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> Copy for IndexedEntry<'index, A, C> where
    <A as NbtAccess>::CORE<'index, C>: Copy
{
}

impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> Clone for IndexedValue<'index, A, C>
where
    <A as NbtAccess>::CORE<'index, C>: Clone,
{
    fn clone(&self) -> Self { Self { core: self.core.clone(), index: self.index } }
}
impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> Copy for IndexedValue<'index, A, C> where
    <A as NbtAccess>::CORE<'index, C>: Copy
{
}

impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> PartialEq for IndexedEntry<'index, A, C> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
            && <C as IndexCore<A>>::root(&self.core) == <C as IndexCore<A>>::root(&other.core)
    }
}
impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> Eq for IndexedEntry<'index, A, C> where
    <A as NbtAccess>::CORE<'index, C>: PartialEq
{
}

impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> PartialEq for IndexedValue<'index, A, C> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
            && <C as IndexCore<A>>::root(&self.core) == <C as IndexCore<A>>::root(&other.core)
    }
}
impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> Eq for IndexedValue<'index, A, C> where
    <A as NbtAccess>::CORE<'index, C>: PartialEq
{
}
