//! TODO

use facet::Peek;
use froglight_facet_iter::{
    WriterError,
    serialize::{Item, SerializeError, Serializer},
};

use crate::prelude::*;

pub mod functions;

/// A trait for types that can be serialized as [`Nbt`].
pub trait SerializeNbt<'facet> {}

// -------------------------------------------------------------------------------------------------

#[inline(never)]
fn serialize(peek: Peek<'_, '_>, nbt: &mut Nbt) -> Result<(), SerializeError> {
    // Create and complete the serializer.
    let mut core = serialize_core(nbt);
    Serializer::new(peek, false, &mut core, Some("mc")).complete()
}

// -------------------------------------------------------------------------------------------------

/// The core logic behind [`serialize`], separated out for readability.
#[doc(hidden)]
#[inline(always)]
#[allow(clippy::inline_always, reason = "Performance")]
pub fn serialize_core<'mem, 'facet>(
    _nbt: &mut Nbt,
) -> impl FnMut(Item<'mem, 'facet>) -> Result<(), WriterError> {
    |item: Item<'mem, 'facet>| -> Result<(), WriterError> {
        let _item = match item {
            Item::Item(item) => item,
            Item::Size(_size) => todo!(),
        };

        todo!()
    }
}
