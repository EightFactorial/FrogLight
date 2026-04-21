use facet::{AllocError, Facet, Partial, ReflectError, ReflectErrorKind};
use facet_path::Path;
use smallvec::SmallVec;

/// TODO
pub(super) struct DeserializeIterator<'facet, const BORROW: bool> {
    partial: Partial<'facet, BORROW>,
    stack: IteratorStack,
}

/// A stack of deserialization frames.
#[expect(unreachable_pub, reason = "Internal type")]
pub type IteratorStack = SmallVec<[DeserItem; 12]>;

/// An item on the deserializer stack
#[derive(Debug, Clone)]
#[expect(unreachable_pub, reason = "Internal type")]
pub enum DeserItem {}

// -------------------------------------------------------------------------------------------------

impl DeserializeIterator<'static, false> {
    /// Create a new [`DeserializeIterator`] for the given type.
    ///
    /// # Errors
    ///
    /// Returns an error if the type cannot be allocated.
    #[inline]
    pub(crate) fn new<T: Facet<'static>>() -> Result<Self, AllocError> {
        Partial::alloc_owned::<T>().map(Self::new_partial)
    }
}

impl<'facet> DeserializeIterator<'facet, true> {
    /// Create a new [`DeserializeIterator`] for the given type.
    ///
    /// # Errors
    ///
    /// Returns an error if the type cannot be allocated.
    #[inline]
    pub(crate) fn new<T: Facet<'facet>>() -> Result<Self, AllocError> {
        Partial::alloc::<T>().map(Self::new_partial)
    }
}

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

    /// Build the final value from the deserialized data.
    ///
    /// # Errors
    ///
    /// Returns an error if some data was not initialized,
    /// or the output type does not match the input type.
    pub(crate) fn build<T: Facet<'facet>>(mut self) -> Result<T, ReflectError> {
        while self.partial.frame_count() > 1 {
            self.partial = self.partial.end()?;
        }

        self.partial.build()?.materialize().map_err(|err| ReflectError {
            kind: ReflectErrorKind::WrongShape { expected: err.expected, actual: err.actual },
            path: Path::new(T::SHAPE),
        })
    }
}
