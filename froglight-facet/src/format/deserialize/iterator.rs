use facet::{AllocError, Facet, Partial, ReflectError, ReflectErrorKind};
use facet_path::Path;
use smallvec::SmallVec;

/// TODO
pub struct DeserializeIterator<'facet, const BORROW: bool> {
    partial: Partial<'facet, BORROW>,
    stack: IteratorStack,
}

/// A stack of deserialization frames.
pub type IteratorStack = SmallVec<[(); 12]>;

impl DeserializeIterator<'static, false> {
    /// Create a new [`DeserializeIterator`] for the given type.
    ///
    /// # Errors
    ///
    /// Returns an error if the type cannot be allocated.
    pub fn new<T: Facet<'static>>() -> Result<Self, AllocError> {
        Ok(Self { partial: Partial::alloc_owned::<T>()?, stack: SmallVec::new_const() })
    }
}

impl<'facet> DeserializeIterator<'facet, true> {
    /// Create a new [`DeserializeIterator`] for the given type.
    ///
    /// # Errors
    ///
    /// Returns an error if the type cannot be allocated.
    pub fn new<T: Facet<'facet>>() -> Result<Self, AllocError> {
        Ok(Self { partial: Partial::alloc::<T>()?, stack: SmallVec::new_const() })
    }
}

impl<'facet, const BORROW: bool> DeserializeIterator<'facet, BORROW> {
    /// Returns `true` if the deserialization process is finished.
    #[inline]
    #[must_use]
    pub const fn is_finished(&self) -> bool { self.partial.frame_count() == 1 }

    /// TODO
    ///
    /// # Errors
    ///
    /// TODO
    pub fn next(
        mut self,
        f: impl FnOnce(
            Partial<'facet, BORROW>,
            &mut IteratorStack,
        ) -> Result<Partial<'facet, BORROW>, ReflectError>,
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
            Partial<'facet, BORROW>,
            &mut IteratorStack,
        ) -> Result<Partial<'facet, BORROW>, ReflectError>,
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
    pub fn build<T: Facet<'facet>>(mut self) -> Result<T, ReflectError> {
        while self.partial.frame_count() > 1 {
            self.partial = self.partial.end()?;
        }

        self.partial.build()?.materialize().map_err(|err| ReflectError {
            kind: ReflectErrorKind::WrongShape { expected: err.expected, actual: err.actual },
            path: Path::new(T::SHAPE),
        })
    }
}

// -------------------------------------------------------------------------------------------------

/// TODO
pub struct DeserIter<'facet, const BORROW: bool, F>
where
    F: FnMut(
        Partial<'facet, BORROW>,
        &mut IteratorStack,
    ) -> Result<Partial<'facet, BORROW>, ReflectError>,
{
    iter: Result<DeserializeIterator<'facet, BORROW>, ReflectError>,
    f: F,
}

impl<'facet, const BORROW: bool, F> DeserIter<'facet, BORROW, F>
where
    F: FnMut(
        Partial<'facet, BORROW>,
        &mut IteratorStack,
    ) -> Result<Partial<'facet, BORROW>, ReflectError>,
{
    /// Create a new [`DeserIter`] from the given [`DeserializeIterator`].
    #[inline]
    #[must_use]
    pub const fn new(iter: DeserializeIterator<'facet, BORROW>, f: F) -> Self {
        Self { iter: Ok(iter), f }
    }

    /// Returns `true` if the iterator is okay.
    #[inline]
    #[must_use]
    pub const fn is_ok(&self) -> bool { self.iter.is_ok() }

    /// Returns `true` if the iterator is finished.
    #[inline]
    #[must_use]
    pub const fn is_finished(&self) -> bool {
        match &self.iter {
            Ok(iter) => iter.is_finished(),
            Err(_) => true,
        }
    }

    /// Returns the inner [`DeserializeIterator`] if the iterator is okay.
    ///
    /// # Errors
    ///
    /// Returns an error if the iterator encountered an error.
    #[inline]
    pub fn into_inner(self) -> Result<DeserializeIterator<'facet, BORROW>, ReflectError> {
        self.iter
    }

    /// Build the final value from the deserialized data.
    ///
    /// # Errors
    ///
    /// Returns an error if some data was not initialized,
    /// or the output type does not match the input type.
    #[inline]
    pub fn build<T: Facet<'facet>>(self) -> Result<T, ReflectError> {
        self.into_inner().and_then(DeserializeIterator::build::<T>)
    }
}

impl<'facet, const BORROW: bool, F> Iterator for DeserIter<'facet, BORROW, F>
where
    F: FnMut(
        Partial<'facet, BORROW>,
        &mut IteratorStack,
    ) -> Result<Partial<'facet, BORROW>, ReflectError>,
{
    type Item = Result<(), ReflectError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished() {
            return None;
        }

        replace_with::replace_with_and_return(
            &mut self.iter,
            || Err(ReflectError { kind: ReflectErrorKind::Unknown, path: Path::new(<()>::SHAPE) }),
            |iter| match iter.and_then(|iter| iter.next(&mut self.f)) {
                Ok(iter) => (Some(Ok(())), Ok(iter)),
                Err(err) => (Some(Err(err.clone())), Err(err)),
            },
        )
    }
}
