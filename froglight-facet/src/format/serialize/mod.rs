//! TODO

use alloc::vec::Vec;

use facet::{Facet, Peek};

mod error;
pub use error::SerializeError;

pub mod functions;

pub(crate) mod iterator;
pub use iterator::IteratorStack;

pub(crate) mod logic;
pub use logic::{Item, Serializer};

use crate::format::writer::Writer;

/// A trait for types that can be deserialized.
#[expect(clippy::missing_errors_doc, missing_docs, reason = "WIP")]
pub trait Serialize<'facet> {
    #[inline]
    fn to_vec(value: &Self) -> Result<Vec<u8>, SerializeError> {
        let mut buffer = Vec::new();
        <Self as Serialize>::to_writer(value, Writer::new(&mut buffer)).map(|_| buffer)
    }

    fn to_writer(value: &Self, writer: Writer<'_>) -> Result<usize, SerializeError>;
}

impl<'facet, T: Facet<'facet>> Serialize<'facet> for T {
    #[inline]
    fn to_vec(value: &Self) -> Result<Vec<u8>, SerializeError> {
        let mut buffer = Vec::with_capacity(64); // TODO: Size hint
        <Self as Serialize>::to_writer(value, Writer::new(&mut buffer)).map(|_| buffer)
    }

    #[inline]
    fn to_writer(value: &Self, writer: Writer<'_>) -> Result<usize, SerializeError> {
        serialize(Peek::new(value), writer)
    }
}

// -------------------------------------------------------------------------------------------------

fn serialize(peek: Peek<'_, '_>, mut writer: Writer<'_>) -> Result<usize, SerializeError> {
    let core = |_item| {
        #[expect(clippy::no_effect_underscore_binding, reason = "|_| { ... }")]
        let _writer = &mut writer;

        todo!();
    };

    let mut ser = Serializer::new(peek, core);
    while let Some(result) = Iterator::next(&mut ser) {
        result?;
    }

    drop(ser);
    Ok(writer.total_written())
}
