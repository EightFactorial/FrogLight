//! TODO
#![allow(dead_code, reason = "WIP")]

use facet::{HeapValue, Partial};
use froglight_facet_iter::{
    ReaderError,
    deserialize::{DeserializeError, Deserializer, Item},
};
use smallvec::SmallVec;

use crate::types::indexed::{
    alloc::{IndexedNbtSlice, SliceCore},
    compound::IndexedCompound,
    core::Ref,
    reference::IndexedValueReference,
};

#[inline(never)]
fn deserialize_owned(
    partial: Partial<'static, false>,
    nbt: &IndexedNbtSlice,
) -> Result<HeapValue<'static, false>, DeserializeError> {
    // Create and complete the deserializer.
    let mut core = deserialize_owned_core(nbt);
    let de = Deserializer::new(partial, false, &mut core, Some("mc"));
    de.complete()?.build().map_err(DeserializeError::from)
}

/// The core logic behind [`deserialize_owned`], separated out for readability.
#[doc(hidden)]
#[inline(always)]
#[allow(clippy::inline_always, reason = "Performance")]
pub fn deserialize_owned_core(
    nbt: &IndexedNbtSlice,
) -> impl FnMut(Item<'static, false>) -> Result<Item<'static, false>, ReaderError> {
    let mut cache = SmallVec::<[IndexedCompound<'_, Ref, SliceCore<'_, Ref>>; 8]>::new_const();
    cache.push(nbt.as_compound());

    move |item: Item<'static, false>| {
        let Item::Item(item) = item else { return Ok(Item::Size(0)) };

        let field = item.partial().nearest_field().unwrap();
        if let Some(field) = cache.iter().rev().find_map(|c| c.get(field.name)) {
            match field.as_value() {
                IndexedValueReference::Byte(v) => {
                    return item.set(v.into_value()).map(Item::Item);
                }
                IndexedValueReference::Short(v) => {
                    return item.set(v.into_value()).map(Item::Item);
                }
                IndexedValueReference::Int(v) => {
                    return item.set(v.into_value()).map(Item::Item);
                }
                IndexedValueReference::Long(v) => {
                    return item.set(v.into_value()).map(Item::Item);
                }
                IndexedValueReference::Float(v) => {
                    return item.set(v.into_value()).map(Item::Item);
                }
                IndexedValueReference::Double(v) => {
                    return item.set(v.into_value()).map(Item::Item);
                }
                IndexedValueReference::ByteArray(v) => {
                    return item.set(v.into_value().to_vec()).map(Item::Item);
                }
                IndexedValueReference::String(v) => {
                    return item.set(v.into_value().to_utf8().into_owned()).map(Item::Item);
                }
                IndexedValueReference::IntArray(v) => {
                    return item.set(v.into_value().to_vec()).map(Item::Item);
                }
                IndexedValueReference::LongArray(v) => {
                    return item.set(v.into_value().to_vec()).map(Item::Item);
                }
                IndexedValueReference::List(_v) => todo!(),
                IndexedValueReference::Compound(_v) => todo!(),
            }
        }

        todo!("{field:?}: \n\n {item:?}")
    }
}

#[test]
fn test() {
    use alloc::string::String;

    use crate::prelude::IndexedNbt;

    #[derive(Debug, facet::Facet)]
    #[allow(non_snake_case, reason = "WIP")]
    struct TestStruct {
        entry_name: String,
        Short: u16,
    }

    static SLICE: &[u8] = &[
        0x0A, 0x08, 0x00, 0x0A, b'e', b'n', b't', b'r', b'y', b'_', b'n', b'a', b'm', b'e', 0x00,
        0x0B, b'e', b'n', b't', b'r', b'y', b'_', b'v', b'a', b'l', b'u', b'e', 0x02, 0x00, 0x05,
        b'S', b'h', b'o', b'r', b't', 0x12, 0x34, 0x00,
    ];

    // Parse `SLICE`
    let nbt = IndexedNbt::new_unnamed_ref(SLICE).unwrap();
    std::println!("NBT: {nbt:#?}");

    let partial = Partial::alloc_owned::<TestStruct>().unwrap();
    let value = deserialize_owned(partial, &nbt).unwrap();
    let test = value.materialize::<TestStruct>().unwrap();

    std::println!("Struct: {test:#?}");

    assert_eq!(test.entry_name, "entry_value");
    assert_eq!(test.Short, 4660);
}
