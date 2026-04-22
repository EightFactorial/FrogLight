//! TODO

use facet::{Facet, HeapValue, Partial};

mod error;
pub use error::DeserializeError;

pub mod functions;

pub(crate) mod iterator;
pub use iterator::IteratorStack;

pub(crate) mod logic;
pub use logic::Deserializer;

use crate::format::reader::Reader;

/// A trait for types that can be deserialized.
#[expect(clippy::missing_errors_doc, missing_docs, reason = "WIP")]
pub trait Deserialize<'facet>: Sized {
    #[inline]
    fn from_slice(slice: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Facet<'static>,
    {
        <Self as Deserialize>::from_slice_remainder(slice).map(|(val, _)| val)
    }

    fn from_slice_remainder(slice: &[u8]) -> Result<(Self, &[u8]), DeserializeError>
    where
        Self: Facet<'static>;

    fn from_slice_borrowed(slice: &'facet [u8]) -> Result<(Self, &'facet [u8]), DeserializeError>
    where
        Self: Facet<'facet>;
}

impl<'facet, T: Sized> Deserialize<'facet> for T {
    fn from_slice_remainder(slice: &[u8]) -> Result<(Self, &[u8]), DeserializeError>
    where
        Self: Facet<'static>,
    {
        let mut cursor = Reader::new(slice);
        let value = deserialize_owned(Partial::alloc_owned::<T>()?, &mut cursor)?;
        Ok((value.materialize::<T>()?, cursor.remaining()))
    }

    fn from_slice_borrowed(slice: &'facet [u8]) -> Result<(Self, &'facet [u8]), DeserializeError>
    where
        Self: Facet<'facet>,
    {
        let mut cursor = Reader::new(slice);
        let value = deserialize_borrowed(Partial::alloc::<T>()?, &mut cursor)?;
        Ok((value.materialize::<T>()?, cursor.remaining()))
    }
}

// -------------------------------------------------------------------------------------------------

fn deserialize_owned(
    partial: Partial<'static, false>,
    reader: &mut Reader<'_>,
) -> Result<HeapValue<'static, false>, DeserializeError> {
    let core = move |_partial| {
        #[expect(clippy::no_effect_underscore_binding, reason = "move |_| { ... }")]
        let _reader = &mut *reader;

        todo!();
    };

    let mut de = Deserializer::new(partial, core);
    while let Some(result) = Iterator::next(&mut de) {
        result?;
    }
    de.into_partial().and_then(|part| part.build().map_err(|_err| todo!()))
}

fn deserialize_borrowed<'facet>(
    partial: Partial<'facet, true>,
    reader: &mut Reader<'facet>,
) -> Result<HeapValue<'facet, true>, DeserializeError> {
    let core = move |_partial| {
        #[expect(clippy::no_effect_underscore_binding, reason = "move |_| { ... }")]
        let _reader = &mut *reader;

        todo!();
    };

    let mut de = Deserializer::new(partial, core);
    while let Some(result) = Iterator::next(&mut de) {
        result?;
    }
    de.into_partial().and_then(|part| part.build().map_err(|_err| todo!()))
}
