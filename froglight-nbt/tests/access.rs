//! TODO
#![no_std]

extern crate alloc;
extern crate std;

use froglight_mutf8::mutf8;
use froglight_nbt::prelude::*;

#[test]
fn hello_world() {
    static SLICE: &[u8] = include_bytes!("nbt/hello_world.nbt");

    // Parse `SLICE`
    let nbt = IndexedNbtRef::new_named(SLICE).unwrap();
    let compound = nbt.as_compound();

    // Test getting entries by name
    if let Some(value) = compound.get("name") {
        assert_eq!(value.as_string().unwrap(), mutf8!("Bananrama"));
    } else {
        panic!("`hello world` not found!");
    }

    // Test iterating over entries
    for entry in compound.iter() {
        assert_eq!(entry.name().get_ref(), mutf8!("name"));
        assert_eq!(entry.value().as_string().unwrap(), mutf8!("Bananrama"));
    }
}

#[test]
fn short() {
    static SLICE: &[u8] = &[
        0x0A, 0x08, 0x00, 0x0A, b'e', b'n', b't', b'r', b'y', b'_', b'n', b'a', b'm', b'e', 0x00,
        0x0B, b'e', b'n', b't', b'r', b'y', b'_', b'v', b'a', b'l', b'u', b'e', 0x02, 0x00, 0x05,
        b'S', b'h', b'o', b'r', b't', 0x12, 0x34,
    ];

    // Parse `SLICE`
    let nbt = IndexedNbtRef::new_unnamed(SLICE).unwrap();
    let compound = nbt.as_compound();

    // Test getting entries by name
    if let Some(value) = compound.get("entry_name") {
        assert_eq!(value.as_string().unwrap(), mutf8!("entry_value"));
    } else {
        panic!("`entry_name` not found!");
    }

    if let Some(value) = compound.get("Short") {
        assert_eq!(value.as_short().unwrap(), 0x1234);
    } else {
        panic!("`Short` not found!");
    }

    // Test iterating over entries
    for (index, entry) in compound.iter().enumerate() {
        match index {
            0 => {
                assert_eq!(entry.name().get_ref(), mutf8!("entry_name"));
                assert_eq!(entry.value().as_string().unwrap(), mutf8!("entry_value"));
            }
            1 => {
                assert_eq!(entry.name().get_ref(), mutf8!("Short"));
                assert_eq!(entry.value().as_short().unwrap(), 0x1234);
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn inttest1023() {
    static SLICE: &[u8] = include_bytes!("nbt/inttest1023.nbt");

    let nbt = IndexedNbtRef::new_named(SLICE).unwrap();
    let compound = nbt.as_compound();

    let value = compound.get("").unwrap();
    let list = value.as_list().unwrap();
    assert_eq!(list.len(), 1023);

    #[expect(clippy::cast_possible_truncation, reason = "Only 1024 entries")]
    for (index, entry) in list.iter().enumerate() {
        assert_eq!(entry.as_int(), Some(index as u32), "Entry {entry:?} is not {index}?");
    }
}

// #[test]
// fn bigtest() {
//     static SLICE: &[u8] = include_bytes!("nbt/bigtest.nbt");
//
//     std::println!("Slice: {:?}", &SLICE[..64]);
//     let nbt = IndexedNbtRef::new_named(SLICE).unwrap();
//     let compound = nbt.as_compound();
//
//     std::println!("Compound: {compound:?}");
// }

// #[test]
// fn complex_player() {
//     static SLICE: &[u8] = include_bytes!("nbt/complex_player.nbt");
//
//     std::println!("Slice: {:?}", &SLICE[..64]);
//     let nbt = IndexedNbtRef::new_named(SLICE).unwrap();
//     let compound = nbt.as_compound();
//
//     std::println!("Compound: {compound:?}");
// }
