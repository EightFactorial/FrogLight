use facet::Partial;
use smallvec::SmallVec;

/// TODO
pub(super) struct DeserializeIterator<'facet, const BORROW: bool> {
    partial: Partial<'facet, BORROW>,
    stack: IteratorStack,
}

/// A stack of deserialization frames.
pub type IteratorStack = SmallVec<[StackItem; 12]>;

/// An item on the deserializer stack
#[derive(Debug, Clone)]
pub enum StackItem {}

// -------------------------------------------------------------------------------------------------

impl<'facet, const BORROW: bool> DeserializeIterator<'facet, BORROW> {
    /// Create a new [`DeserializeIterator`] for the given [`Partial`].
    #[inline]
    #[must_use]
    pub(crate) const fn new_partial(partial: Partial<'facet, BORROW>) -> Self {
        Self { partial, stack: SmallVec::new_const() }
    }

    /// Returns `true` if the deserialization process is finished.
    #[inline]
    #[must_use]
    pub(crate) const fn is_finished(&self) -> bool { self.partial.frame_count() == 1 }

    /// TODO
    ///
    /// # Errors
    ///
    /// TODO
    pub(crate) fn next<Err>(
        mut self,
        f: impl FnOnce(
            Partial<'facet, BORROW>,
            &mut IteratorStack,
        ) -> Result<Partial<'facet, BORROW>, Err>,
    ) -> Result<Self, Err> {
        self.partial = f(self.partial, &mut self.stack)?;
        Ok(self)
    }

    /// Get the [`Partial`] from the iterator.
    #[inline]
    #[must_use]
    pub(crate) fn into_partial(self) -> Partial<'facet, BORROW> { self.partial }
}
