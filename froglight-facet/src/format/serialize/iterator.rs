use facet::{Facet, Peek};
use smallvec::SmallVec;

/// TODO
pub struct SerializeIterator<'mem, 'facet> {
    stack: IteratorStack<'mem, 'facet>,
    _phantom: core::marker::PhantomData<(&'mem (), &'facet ())>,
}

/// A stack of serialization frames.
pub type IteratorStack<'mem, 'facet> = SmallVec<[SerItem; 12]>;

/// An item on the serializer stack
#[derive(Debug, Clone)]
pub enum SerItem {}

// -------------------------------------------------------------------------------------------------

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

    /// Returns `true` if the iterator is finished.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.stack.is_empty() }

    /// TODO
    ///
    /// # Errors
    ///
    /// TODO
    pub fn next<Err>(
        &mut self,
        _f: impl FnOnce(Peek<'mem, 'facet>, &mut IteratorStack<'mem, 'facet>) -> Result<(), Err>,
    ) -> Option<Result<(), Err>> {
        if self.stack.is_empty() { None } else { todo!() }
    }
}
