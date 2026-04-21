use facet::{Facet, Peek, ReflectError};

use crate::format::serialize::iterator::{IteratorStack, SerializeIterator};

/// TODO
pub struct Serializer<'mem, 'facet, F>
where
    F: FnMut(Peek<'mem, 'facet>, &mut IteratorStack<'mem, 'facet>) -> Result<(), ReflectError>,
{
    iter: SerializeIterator<'mem, 'facet>,
    f: F,
}

impl<'mem, 'facet, F> Serializer<'mem, 'facet, F>
where
    F: FnMut(Peek<'mem, 'facet>, &mut IteratorStack<'mem, 'facet>) -> Result<(), ReflectError>,
{
    /// Create a new [`Serializer`] for the given type.
    #[inline]
    #[must_use]
    pub fn new<T: Facet<'facet>>(
        value: &'mem T,
        core: impl FnMut(Peek<'_, '_>),
    ) -> Serializer<
        'mem,
        'facet,
        impl FnMut(Peek<'mem, 'facet>, &mut IteratorStack<'mem, 'facet>) -> Result<(), ReflectError>,
    > {
        Serializer { iter: SerializeIterator::new(value), f: serializer(core) }
    }

    /// Returns `true` if the iterator is finished.
    #[inline]
    #[must_use]
    pub fn is_finished(&self) -> bool { self.iter.is_empty() }

    /// Returns the inner [`SerializeIterator`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> SerializeIterator<'mem, 'facet> { self.iter }
}

impl<'mem, 'facet, F> Iterator for Serializer<'mem, 'facet, F>
where
    F: FnMut(Peek<'mem, 'facet>, &mut IteratorStack<'mem, 'facet>) -> Result<(), ReflectError>,
{
    type Item = Result<(), ReflectError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> { self.iter.next(&mut self.f) }
}

// -------------------------------------------------------------------------------------------------

fn serializer<'mem, 'facet>(
    _core: impl FnMut(Peek<'mem, 'facet>),
) -> impl FnMut(Peek<'mem, 'facet>, &mut IteratorStack<'mem, 'facet>) -> Result<(), ReflectError> {
    move |_peek, _stack| todo!()
}
