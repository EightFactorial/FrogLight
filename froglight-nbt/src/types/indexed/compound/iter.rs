use crate::types::indexed::{
    compound::IndexedCompound,
    core::{IndexCore, NbtAccess, Ref},
    entry::IndexedEntry,
};

/// An iterator over the entries in an [`IndexedCompound`].
pub struct CompoundOwnedIter<'index, A: NbtAccess, C: IndexCore<A>>
where
    <A as NbtAccess>::CORE<'index, C>: Clone,
{
    compound: IndexedCompound<'index, A, C>,
    index: usize,
}

impl<'index, C: IndexCore<A>, A: NbtAccess> CompoundOwnedIter<'index, A, C>
where
    <A as NbtAccess>::CORE<'index, C>: Clone,
{
    /// Create a new [`CompoundOwnedIter`] from the given [`IndexedCompound`].
    #[inline]
    #[must_use]
    pub const fn new(compound: IndexedCompound<'index, A, C>) -> Self {
        CompoundOwnedIter { compound, index: 0 }
    }
}

impl<'index, A: NbtAccess, C: IndexCore<A>> IntoIterator for IndexedCompound<'index, A, C>
where
    <A as NbtAccess>::CORE<'index, C>: Clone,
{
    type IntoIter = CompoundOwnedIter<'index, A, C>;
    type Item = IndexedEntry<'index, A, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { CompoundOwnedIter::new(self) }
}

impl<'index, A: NbtAccess, C: IndexCore<A>> Iterator for CompoundOwnedIter<'index, A, C>
where
    <A as NbtAccess>::CORE<'index, C>: Clone,
{
    type Item = IndexedEntry<'index, A, C>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.compound.clone().get_index(self.index)?;
        self.index += 1;

        Some(entry)
    }
}
impl<'index, A: NbtAccess, C: IndexCore<A>> ExactSizeIterator for CompoundOwnedIter<'index, A, C>
where
    <A as NbtAccess>::CORE<'index, C>: Clone,
{
    fn len(&self) -> usize { self.compound.len() - self.index }
}

// -------------------------------------------------------------------------------------------------

/// An iterator over the entries in an [`IndexedCompound`].
pub struct CompoundIter<'iter, 'index, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> {
    compound: &'iter IndexedCompound<'index, A, C>,
    index: usize,
}

impl<'iter, 'index, C: IndexCore<Ref> + IndexCore<A>, A: NbtAccess>
    CompoundIter<'iter, 'index, A, C>
{
    /// Create a new [`CompoundIter`] from the given [`IndexedCompound`].
    #[inline]
    #[must_use]
    pub const fn new(compound: &'iter IndexedCompound<'index, A, C>) -> Self {
        CompoundIter { compound, index: 0 }
    }
}

impl<'iter, 'index, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> IntoIterator
    for &'iter IndexedCompound<'index, A, C>
{
    type IntoIter = CompoundIter<'iter, 'index, A, C>;
    type Item = IndexedEntry<'iter, Ref, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[allow(clippy::into_iter_without_iter, reason = "Ignored")]
impl<'iter, 'index, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> IntoIterator
    for &'iter mut IndexedCompound<'index, A, C>
{
    type IntoIter = CompoundIter<'iter, 'index, A, C>;
    type Item = IndexedEntry<'iter, Ref, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<'iter, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> Iterator
    for CompoundIter<'iter, '_, A, C>
{
    type Item = IndexedEntry<'iter, Ref, C>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.compound.get_index_ref(self.index)?;
        self.index += 1;

        Some(entry)
    }
}
impl<A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> ExactSizeIterator
    for CompoundIter<'_, '_, A, C>
{
    fn len(&self) -> usize { self.compound.len() - self.index }
}
