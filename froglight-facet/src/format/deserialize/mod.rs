//! TODO

use facet::{Facet, Partial, ReflectError};

mod error;
pub use error::DeserializeError;

pub mod functions;

pub(crate) mod iterator;

pub(crate) mod logic;
use logic::Deserializer;

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
    fn from_slice_remainder(_slice: &[u8]) -> Result<(Self, &[u8]), DeserializeError>
    where
        Self: Facet<'static>,
    {
        let mut cursor = ();
        let mut de = Deserializer::new_owned::<T>(owned(&mut cursor))?;
        while let Some(result) = Iterator::next(&mut de) {
            result?;
        }
        de.build::<T>().map(|_val| todo!())
    }

    fn from_slice_borrowed(_slice: &'facet [u8]) -> Result<(Self, &'facet [u8]), DeserializeError>
    where
        Self: Facet<'facet>,
    {
        let mut cursor = ();
        let mut de = Deserializer::new_borrowed::<T>(borrowed(&mut cursor))?;
        while let Some(result) = Iterator::next(&mut de) {
            result?;
        }
        de.build::<T>().map(|_val| todo!())
    }
}

// -------------------------------------------------------------------------------------------------

fn owned(
    _cursor: &mut (),
) -> impl FnMut(Partial<'static, false>) -> Result<Partial<'static, false>, ReflectError> {
    move |_partial| todo!()
}

fn borrowed<'facet>(
    _cursor: &mut (),
) -> impl FnMut(Partial<'facet, true>) -> Result<Partial<'facet, true>, ReflectError> {
    move |_partial| todo!()
}
