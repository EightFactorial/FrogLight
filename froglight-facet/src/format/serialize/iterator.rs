use facet::{Peek, Shape};
use smallvec::SmallVec;

/// TODO
pub struct SerializeIterator<'mem, 'facet> {
    shape: &'static Shape,
    stack: IteratorStack<'mem, 'facet>,
}

/// A stack of serialization frames.
pub type IteratorStack<'mem, 'facet> = SmallVec<[(Peek<'mem, 'facet>, StackItem); 12]>;

/// An item on the serializer stack
pub enum StackItem {
    /// A value to serialize.
    Value(bool),
    /// A value to process.
    Other(bool),
}

// -------------------------------------------------------------------------------------------------

impl<'mem, 'facet> SerializeIterator<'mem, 'facet> {
    /// Create a new [`SerializeIterator`] from the given [`Peek`].
    #[must_use]
    pub fn new(peek: Peek<'mem, 'facet>) -> Self {
        let mut stack = IteratorStack::new_const();
        stack.push((peek, StackItem::Other(false)));
        Self { shape: peek.shape(), stack }
    }

    /// Get the [`Shape`] that is being serialized.
    #[inline]
    #[must_use]
    pub const fn shape(&self) -> &'static Shape { self.shape }

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
        f: impl FnOnce(&mut IteratorStack<'mem, 'facet>) -> Result<(), Err>,
    ) -> Option<Result<(), Err>> {
        if self.stack.is_empty() { None } else { Some(f(&mut self.stack)) }
    }
}
