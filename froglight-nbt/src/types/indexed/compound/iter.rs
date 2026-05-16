use crate::types::indexed::{
    compound::IndexedCompound,
    core::{IndexCore, NbtAccess, Ref},
    entry::IndexedEntry,
};

/// An iterator over the entries in an [`IndexedCompound`].
pub struct CompoundIter<'iter, 'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> {
    compound: &'iter IndexedCompound<'data, A, C>,
    index: usize,
}

impl<'iter, 'data, C: IndexCore<Ref> + IndexCore<A>, A: NbtAccess>
    CompoundIter<'iter, 'data, A, C>
{
    /// Create a new [`CompoundIter`] from the given [`IndexedCompound`].
    #[inline]
    #[must_use]
    pub const fn new(compound: &'iter IndexedCompound<'data, A, C>) -> Self {
        CompoundIter { compound, index: 0 }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'iter, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> Iterator
    for CompoundIter<'iter, '_, A, C>
{
    type Item = IndexedEntry<'iter, Ref, C>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.compound.entries().get(self.index).copied()?;
        self.index += 1;

        // SAFETY: `IndexedCompound` guarantees that `entry` has valid indexes.
        Some(unsafe { IndexedEntry::<Ref, C>::new(&self.compound.core, entry) })
    }
}

impl<'iter, 'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> IntoIterator
    for &'iter IndexedCompound<'data, A, C>
{
    type IntoIter = CompoundIter<'iter, 'data, A, C>;
    type Item = IndexedEntry<'iter, Ref, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
