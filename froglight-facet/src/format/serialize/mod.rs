//! TODO

use alloc::vec::Vec;

use facet::Facet;

mod error;
pub use error::SerializeError;

pub mod functions;

pub(crate) mod iterator;
pub use iterator::IteratorStack;

pub(crate) mod logic;
pub use logic::Serializer;

/// A trait for types that can be deserialized.
#[expect(clippy::result_unit_err, clippy::missing_errors_doc, missing_docs, reason = "WIP")]
pub trait Serialize<'facet> {
    #[inline]
    fn to_vec(value: &Self) -> Result<Vec<u8>, ()> {
        let buffer = Vec::new();
        <Self as Serialize>::to_buffer(value, &mut ()).map(|_| buffer)
    }

    fn to_buffer(value: &Self, buffer: &mut ()) -> Result<usize, ()>;
}

impl<'facet, T: Facet<'facet>> Serialize<'facet> for T {
    #[inline]
    fn to_vec(value: &Self) -> Result<Vec<u8>, ()> {
        let buffer = Vec::with_capacity(64); // TODO: Size hint
        <Self as Serialize>::to_buffer(value, &mut ()).map(|_| buffer)
    }

    fn to_buffer(_value: &Self, _buffer: &mut ()) -> Result<usize, ()> { todo!() }
}
