//! TODO

use facet::{Partial, ReflectError};

use crate::format::deserialize::{
    DeserializeError,
    iterator::{DeserializeIterator, IteratorStack},
};

/// TODO
pub struct Deserializer<'facet, const BORROW: bool, C> {
    iter: Result<DeserializeIterator<'facet, BORROW>, DeserializeError>,
    core: C,
}

/// A deserializer item.
pub enum Item<'facet, const BORROW: bool> {
    /// A size to be deserialized.
    Size(u32),
    /// A value to be deserialized.
    Partial(Partial<'facet, BORROW>),
}

impl<'facet, const BORROW: bool> Deserializer<'facet, BORROW, ()> {
    /// Create a new [`Deserializer`] for the given type.
    #[inline]
    pub(crate) fn new(
        partial: Partial<'facet, BORROW>,
        core: impl FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReflectError>,
    ) -> Deserializer<'facet, BORROW, impl DeserializerCore<'facet, BORROW>> {
        Deserializer {
            iter: Ok(DeserializeIterator::new_partial(partial)),
            core: create_core(core),
        }
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
    pub(crate) fn into_partial(self) -> Result<Partial<'facet, BORROW>, DeserializeError> {
        self.iter.map(DeserializeIterator::into_partial)
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
    mut core: impl FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReflectError>,
) -> impl FnMut(
    Partial<'facet, BORROW>,
    &mut IteratorStack,
) -> Result<Partial<'facet, BORROW>, DeserializeError> {
    move |_peek, _stack| {
        let _core = &mut core;
        todo!();
    }
}
