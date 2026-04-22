use facet::{Peek, ReflectError};

use crate::format::serialize::iterator::{IteratorStack, SerializeIterator};

/// TODO
pub struct Serializer<'mem, 'facet, C> {
    iter: SerializeIterator<'mem, 'facet>,
    core: C,
}

impl<'mem, 'facet> Serializer<'mem, 'facet, ()> {
    /// Create a new [`Serializer`] for the given type.
    #[inline]
    #[must_use]
    pub fn new(
        peek: Peek<'mem, 'facet>,
        core: impl FnMut(Peek<'mem, 'facet>),
    ) -> Serializer<'mem, 'facet, impl SerializerCore<'mem, 'facet>> {
        Serializer { iter: SerializeIterator::new(peek), core: create_core(core) }
    }
}

impl<'mem, 'facet, C: SerializerCore<'mem, 'facet>> Serializer<'mem, 'facet, C> {
    /// Returns `true` if the iterator is finished.
    #[inline]
    #[must_use]
    pub fn is_finished(&self) -> bool { self.iter.is_empty() }

    /// Returns the inner [`SerializeIterator`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> SerializeIterator<'mem, 'facet> { self.iter }
}

impl<'mem, 'facet, C: SerializerCore<'mem, 'facet>> Iterator for Serializer<'mem, 'facet, C> {
    type Item = Result<(), ReflectError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> { self.iter.next(self.core.as_fn_once()) }
}

// -------------------------------------------------------------------------------------------------

/// A trait for serializer cores.
pub trait SerializerCore<'mem, 'facet> {
    fn as_fn_once(
        &mut self,
    ) -> impl FnOnce(Peek<'mem, 'facet>, &mut IteratorStack<'mem, 'facet>) -> Result<(), ReflectError> + '_;
}

impl<'mem, 'facet, F> SerializerCore<'mem, 'facet> for F
where
    F: FnMut(Peek<'mem, 'facet>, &mut IteratorStack<'mem, 'facet>) -> Result<(), ReflectError>,
{
    #[inline]
    fn as_fn_once(
        &mut self,
    ) -> impl FnOnce(Peek<'mem, 'facet>, &mut IteratorStack<'mem, 'facet>) -> Result<(), ReflectError> + '_
    {
        self
    }
}

// -------------------------------------------------------------------------------------------------

fn create_core<'mem, 'facet>(
    _core: impl FnMut(Peek<'mem, 'facet>),
) -> impl FnMut(Peek<'mem, 'facet>, &mut IteratorStack<'mem, 'facet>) -> Result<(), ReflectError> {
    move |_peek, _stack| todo!()
}
