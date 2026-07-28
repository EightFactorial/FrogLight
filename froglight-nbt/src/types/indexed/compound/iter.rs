use crate::types::indexed::{
    compound::IndexedCompound,
    core::{IndexCore, NbtAccess, Ref},
    entry::IndexedEntry,
};

/// An iterator over the entries in an [`IndexedCompound`].
pub struct CompoundOwnedIter<'data, A: NbtAccess, C: IndexCore<A>>
where
    <A as NbtAccess>::CORE<'data, C>: Clone,
{
    compound: IndexedCompound<'data, A, C>,
    index: usize,
}

impl<'data, C: IndexCore<A>, A: NbtAccess> CompoundOwnedIter<'data, A, C>
where
    <A as NbtAccess>::CORE<'data, C>: Clone,
{
    /// Create a new [`CompoundOwnedIter`] from the given [`IndexedCompound`].
    #[inline]
    #[must_use]
    pub const fn new(compound: IndexedCompound<'data, A, C>) -> Self {
        CompoundOwnedIter { compound, index: 0 }
    }
}

impl<'data, A: NbtAccess, C: IndexCore<A>> IntoIterator for IndexedCompound<'data, A, C>
where
    <A as NbtAccess>::CORE<'data, C>: Clone,
{
    type IntoIter = CompoundOwnedIter<'data, A, C>;
    type Item = IndexedEntry<'data, A, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { CompoundOwnedIter::new(self) }
}

impl<'data, A: NbtAccess, C: IndexCore<A>> Iterator for CompoundOwnedIter<'data, A, C>
where
    <A as NbtAccess>::CORE<'data, C>: Clone,
{
    type Item = IndexedEntry<'data, A, C>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.compound.clone().get_index(self.index)?;
        self.index += 1;

        Some(entry)
    }
}
impl<'data, A: NbtAccess, C: IndexCore<A>> ExactSizeIterator for CompoundOwnedIter<'data, A, C>
where
    <A as NbtAccess>::CORE<'data, C>: Clone,
{
    fn len(&self) -> usize { self.compound.len() - self.index }
}

// -------------------------------------------------------------------------------------------------

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

impl<'iter, 'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> IntoIterator
    for &'iter IndexedCompound<'data, A, C>
{
    type IntoIter = CompoundIter<'iter, 'data, A, C>;
    type Item = IndexedEntry<'iter, Ref, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[allow(clippy::into_iter_without_iter, reason = "Ignored")]
impl<'iter, 'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> IntoIterator
    for &'iter mut IndexedCompound<'data, A, C>
{
    type IntoIter = CompoundIter<'iter, 'data, A, C>;
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
