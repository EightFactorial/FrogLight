use facet::{Field, Peek, Shape};
use smallvec::SmallVec;

/// TODO
pub struct SerializeIterator<'mem, 'facet> {
    shape: &'static Shape,
    stack: IteratorStack<'mem, 'facet>,
}

/// A stack of serialization frames.
pub type IteratorStack<'mem, 'facet> = SmallVec<[StackItem<'mem, 'facet>; 12]>;

#[derive(Debug, Clone)]
pub struct StackItem<'mem, 'facet> {
    pub peek: Peek<'mem, 'facet>,
    pub ty: ItemType,
    pub variable: bool,
    pub field: Option<Field>,
}

/// An item on the serializer stack
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemType {
    /// A value to serialize.
    Value,
    /// A value to process.
    Other,
}

impl<'mem, 'facet> StackItem<'mem, 'facet> {
    /// Create a new [`StackItem`].
    #[inline]
    #[must_use]
    pub const fn new(peek: Peek<'mem, 'facet>, ty: ItemType, var: bool) -> Self {
        Self { peek, ty, variable: var, field: None }
    }

    /// Set the [`Field`] this [`StackItem`] came from.
    #[inline]
    #[must_use]
    pub const fn with_field(mut self, field: Option<Field>) -> Self {
        self.field = field;
        self
    }
}

// -------------------------------------------------------------------------------------------------

impl<'mem, 'facet> SerializeIterator<'mem, 'facet> {
    /// Create a new [`SerializeIterator`] from the given [`Peek`].
    #[must_use]
    pub fn new(peek: Peek<'mem, 'facet>) -> Self {
        let mut stack = IteratorStack::new_const();
        stack.push(StackItem::new(peek, ItemType::Other, false));
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
