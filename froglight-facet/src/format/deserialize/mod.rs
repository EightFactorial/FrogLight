//! TODO

use facet::Facet;

mod error;
// pub use error::DeserializeError;

pub mod functions;

mod iterator;
pub use iterator::{DeserItem, DeserIter, DeserializeIterator, IteratorStack};

/// A trait for types that can be deserialized.
#[expect(clippy::result_unit_err, clippy::missing_errors_doc, missing_docs, reason = "WIP")]
pub trait Deserialize<'facet>: Sized {
    #[inline]
    fn from_slice(slice: &[u8]) -> Result<Self, ()> {
        <Self as Deserialize>::from_slice_remainder(slice).map(|(val, _)| val)
    }

    fn from_slice_remainder(slice: &[u8]) -> Result<(Self, &[u8]), ()>;

    fn from_slice_borrowed(slice: &'facet [u8]) -> Result<(Self, &'facet [u8]), ()>;
}

impl<'facet, T: Facet<'facet> + Sized> Deserialize<'facet> for T {
    fn from_slice_remainder(_slice: &[u8]) -> Result<(Self, &[u8]), ()> { todo!() }

    fn from_slice_borrowed(_slice: &'facet [u8]) -> Result<(Self, &'facet [u8]), ()> { todo!() }
}
