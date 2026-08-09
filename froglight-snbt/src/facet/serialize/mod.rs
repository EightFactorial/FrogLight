//! TODO

use alloc::string::String;

use facet::Peek;
use froglight_facet_iter::{
    WriterError,
    serialize::{Item, SerializeError, Serializer},
};

pub mod functions;

/// A trait for types that can be serialized as [`Snbt`].
pub trait SerializeSnbt<'facet> {}

// -------------------------------------------------------------------------------------------------

#[inline(never)]
fn serialize(peek: Peek<'_, '_>, snbt: &mut String) -> Result<(), SerializeError> {
    // Create and complete the serializer.
    let mut core = serialize_core(snbt);
    Serializer::new(peek, false, &mut core, Some("snbt")).complete()
}

// -------------------------------------------------------------------------------------------------

/// The core logic behind [`serialize`], separated out for readability.
#[doc(hidden)]
#[inline(always)]
#[allow(clippy::inline_always, reason = "Performance")]
pub fn serialize_core<'mem, 'facet>(
    _snbt: &mut String,
) -> impl FnMut(Item<'mem, 'facet>) -> Result<(), WriterError> {
    |item: Item<'mem, 'facet>| -> Result<(), WriterError> {
        let _item = match item {
            Item::Item(item) => item,
            Item::Hint(_hint, _peek) => todo!(),
        };

        todo!()
    }
}
