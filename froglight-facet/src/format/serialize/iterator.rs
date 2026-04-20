use facet::{Facet, Peek, ReflectError};
use smallvec::SmallVec;

/// TODO
pub struct SerializeIterator<'mem, 'facet> {
    stack: IteratorStack<'mem, 'facet>,
    _phantom: core::marker::PhantomData<(&'mem (), &'facet ())>,
}

/// A stack of serialization frames.
pub type IteratorStack<'mem, 'facet> = SmallVec<[(); 12]>;

impl<'mem, 'facet> SerializeIterator<'mem, 'facet> {
    /// Create a new [`SerializeIterator`] for the given type.
    #[inline]
    #[must_use]
    pub fn new<T: Facet<'facet>>(value: &'mem T) -> Self { Self::new_from(Peek::new(value)) }

    /// Create a new [`SerializeIterator`] from the given [`Peek`].
    #[must_use]
    pub fn new_from(_peek: Peek<'mem, 'facet>) -> Self {
        Self { stack: IteratorStack::new_const(), _phantom: core::marker::PhantomData }
    }

    /// TODO
    ///
    /// # Errors
    ///
    /// TODO
    pub fn next(
        &mut self,
        _f: impl FnOnce(
            Peek<'mem, 'facet>,
            &mut IteratorStack<'mem, 'facet>,
        ) -> Result<(), ReflectError>,
    ) -> Option<Result<(), ReflectError>> {
        if self.stack.is_empty() { None } else { todo!() }
    }

    /// TODO
    ///
    /// # Errors
    ///
    /// TODO
    pub fn complete(
        &mut self,
        mut f: impl FnMut(
            Peek<'mem, 'facet>,
            &mut IteratorStack<'mem, 'facet>,
        ) -> Result<(), ReflectError>,
    ) -> Option<Result<(), ReflectError>> {
        if self.stack.is_empty() {
            None
        } else {
            while !self.stack.is_empty() {
                match self.next(&mut f) {
                    Some(Ok(())) => {}
                    Some(Err(err)) => return Some(Err(err)),
                    None => break,
                }
            }
            Some(Ok(()))
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// TODO
pub struct SerIter<'mem, 'facet, F>
where
    F: FnMut(Peek<'mem, 'facet>, &mut IteratorStack<'mem, 'facet>) -> Result<(), ReflectError>,
{
    iter: SerializeIterator<'mem, 'facet>,
    f: F,
}

impl<'mem, 'facet, F> SerIter<'mem, 'facet, F>
where
    F: FnMut(Peek<'mem, 'facet>, &mut IteratorStack<'mem, 'facet>) -> Result<(), ReflectError>,
{
    /// Create a new [`SerIter`] with the given [`SerializeIterator`].
    #[inline]
    #[must_use]
    pub const fn new(iter: SerializeIterator<'mem, 'facet>, f: F) -> Self { Self { iter, f } }

    /// Returns `true` if the iterator is finished.
    #[inline]
    #[must_use]
    pub fn is_finished(&self) -> bool { self.iter.stack.is_empty() }

    /// Returns the inner [`SerializeIterator`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> SerializeIterator<'mem, 'facet> { self.iter }
}

impl<'mem, 'facet, F> Iterator for SerIter<'mem, 'facet, F>
where
    F: FnMut(Peek<'mem, 'facet>, &mut IteratorStack<'mem, 'facet>) -> Result<(), ReflectError>,
{
    type Item = Result<(), ReflectError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> { self.iter.next(&mut self.f) }
}
