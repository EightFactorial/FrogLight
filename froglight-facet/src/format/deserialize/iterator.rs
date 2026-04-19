use facet::{AllocError, Facet, Partial, ReflectError, ReflectErrorKind};
use facet_path::Path;
use smallvec::SmallVec;

/// TODO
pub struct DeserializeIterator<const BORROW: bool> {
    partial: Partial<'static, BORROW>,
    stack: IteratorStack,
}

/// A stack of deserialization frames.
pub type IteratorStack = SmallVec<[(); 12]>;

impl DeserializeIterator<false> {
    /// Create a new [`DeserializeIterator`] for the given type.
    ///
    /// # Errors
    ///
    /// Returns an error if the type cannot be allocated.
    pub fn new<T: Facet<'static>>() -> Result<Self, AllocError> {
        Ok(Self { partial: Partial::alloc_owned::<T>()?, stack: SmallVec::new_const() })
    }
}

impl DeserializeIterator<true> {
    /// Create a new [`DeserializeIterator`] for the given type.
    ///
    /// # Errors
    ///
    /// Returns an error if the type cannot be allocated.
    pub fn new<T: Facet<'static>>() -> Result<Self, AllocError> {
        Ok(Self { partial: Partial::alloc::<T>()?, stack: SmallVec::new_const() })
    }
}

impl<const BORROW: bool> DeserializeIterator<BORROW> {
    /// TODO
    ///
    /// # Errors
    ///
    /// TODO
    pub fn next(
        mut self,
        f: impl FnOnce(
            Partial<'static, BORROW>,
            &mut IteratorStack,
        ) -> Result<Partial<'static, BORROW>, ReflectError>,
    ) -> Result<Self, ReflectError> {
        self.partial = f(self.partial, &mut self.stack)?;
        Ok(self)
    }

    /// TODO
    ///
    /// # Errors
    ///
    /// TODO
    pub fn complete(
        mut self,
        mut f: impl FnMut(
            Partial<'static, BORROW>,
            &mut IteratorStack,
        ) -> Result<Partial<'static, BORROW>, ReflectError>,
    ) -> Result<Self, ReflectError> {
        while self.partial.frame_count() > 1 {
            self.partial = f(self.partial, &mut self.stack)?;
        }
        Ok(self)
    }

    /// Build the final value from the deserialized data.
    ///
    /// # Errors
    ///
    /// Returns an error if some data was not initialized,
    /// or the output type does not match the input type.
    pub fn build<T: Facet<'static>>(mut self) -> Result<T, ReflectError> {
        while self.partial.frame_count() > 1 {
            self.partial = self.partial.end()?;
        }

        self.partial.build()?.materialize().map_err(|err| ReflectError {
            kind: ReflectErrorKind::WrongShape { expected: err.expected, actual: err.actual },
            path: Path::new(T::SHAPE),
        })
    }
}
