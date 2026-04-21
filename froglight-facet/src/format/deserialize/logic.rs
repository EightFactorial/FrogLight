//! TODO

use facet::{AllocError, Facet, Partial, ReflectError};

use crate::format::deserialize::{
    DeserializeError,
    iterator::{DeserializeIterator, IteratorStack},
};

pub(crate) struct Deserializer<'facet, const BORROW: bool, C> {
    iter: Result<DeserializeIterator<'facet, BORROW>, DeserializeError>,
    core: C,
}

impl Deserializer<'static, false, ()> {
    /// Create a new [`Deserializer`] for the given type.
    ///
    ///
    /// # Errors
    ///
    /// Returns an error if the type cannot be allocated.
    #[inline]
    pub(crate) fn new_owned<T: Facet<'static>>(
        core: impl FnMut(Partial<'static, false>) -> Result<Partial<'static, false>, ReflectError>,
    ) -> Result<Deserializer<'static, false, impl DeserializerCore<'static, false>>, AllocError>
    {
        DeserializeIterator::<false>::new::<T>()
            .map(move |iter| Deserializer { iter: Ok(iter), core: create_core(core) })
    }

    /// Create a new [`Deserializer`] for the given type.
    ///
    ///
    /// # Errors
    ///
    /// Returns an error if the type cannot be allocated.
    #[inline]
    pub(crate) fn new_borrowed<'facet, T: Facet<'facet>>(
        core: impl FnMut(Partial<'facet>) -> Result<Partial<'facet>, ReflectError>,
    ) -> Result<Deserializer<'facet, true, impl DeserializerCore<'facet, true>>, AllocError> {
        DeserializeIterator::<true>::new::<T>()
            .map(move |iter| Deserializer { iter: Ok(iter), core: create_core(core) })
    }
}

impl<'facet, const BORROW: bool, C: DeserializerCore<'facet, BORROW>>
    Deserializer<'facet, BORROW, C>
{
    /// Returns `true` if the iterator is finished.
    #[inline]
    #[must_use]
    pub(crate) const fn is_finished(&self) -> bool {
        match &self.iter {
            Ok(iter) => iter.is_finished(),
            Err(_) => true,
        }
    }

    /// Build the final value from the deserialized data.
    ///
    /// # Errors
    ///
    /// Returns an error if some data was not initialized,
    /// or the output type does not match the input type.
    #[inline]
    pub(crate) fn build<T: Facet<'facet>>(self) -> Result<T, DeserializeError> {
        self.iter.and_then(|iter| iter.build::<T>().map_err(Into::into))
    }
}

impl<'facet, const BORROW: bool, C: DeserializerCore<'facet, BORROW>> Iterator
    for Deserializer<'facet, BORROW, C>
{
    type Item = Result<(), DeserializeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished() {
            return None;
        }

        replace_with::replace_with_and_return(
            &mut self.iter,
            || Err(DeserializeError),
            |iter| match iter.and_then(|iter| iter.next(self.core.as_fn_once())) {
                Ok(iter) => (Some(Ok(())), Ok(iter)),
                Err(err) => (Some(Err(err.clone())), Err(err)),
            },
        )
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait for deserializer cores.
#[expect(unreachable_pub, reason = "Internal trait")]
pub trait DeserializerCore<'facet, const BORROW: bool> {
    fn as_fn_once(
        &mut self,
    ) -> impl FnOnce(
        Partial<'facet, BORROW>,
        &mut IteratorStack,
    ) -> Result<Partial<'facet, BORROW>, DeserializeError>
    + '_;
}

impl<'facet, const BORROW: bool, T> DeserializerCore<'facet, BORROW> for T
where
    T: FnMut(
        Partial<'facet, BORROW>,
        &mut IteratorStack,
    ) -> Result<Partial<'facet, BORROW>, DeserializeError>,
{
    #[inline]
    fn as_fn_once(
        &mut self,
    ) -> impl FnOnce(
        Partial<'facet, BORROW>,
        &mut IteratorStack,
    ) -> Result<Partial<'facet, BORROW>, DeserializeError>
    + '_ {
        self
    }
}

// -------------------------------------------------------------------------------------------------

/// A generic [`DeserializerCore`] wrapper that only calls the provided
/// function on values to be deserialized.
fn create_core<'facet, const BORROW: bool>(
    mut core: impl FnMut(Partial<'facet, BORROW>) -> Result<Partial<'facet, BORROW>, ReflectError>,
) -> impl FnMut(
    Partial<'facet, BORROW>,
    &mut IteratorStack,
) -> Result<Partial<'facet, BORROW>, DeserializeError> {
    move |_peek, _stack| {
        let _core = &mut core;
        todo!();
    }
}
