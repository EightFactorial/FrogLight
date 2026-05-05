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

#[test]
fn bigtest() {
    static SLICE: &[u8] = include_bytes!("nbt/bigtest.nbt");

    let nbt = IndexedNbtRef::new_named(SLICE).unwrap();
    let compound = nbt.as_compound();

    // Test getting entries by name
    if let Some(long_test) = compound.get("longTest") {
        assert_eq!(long_test.as_long().unwrap(), 9_223_372_036_854_775_807);
    } else {
        panic!("`longTest` not found!");
    }

    if let Some(int_test) = compound.get("intTest") {
        assert_eq!(int_test.as_int().unwrap(), 2_147_483_647);
    } else {
        panic!("`intTest` not found!");
    }

    if let Some(short_test) = compound.get("shortTest") {
        assert_eq!(short_test.as_short().unwrap(), 32767);
    } else {
        panic!("`shortTest` not found!");
    }

    if let Some(byte_test) = compound.get("byteTest") {
        assert_eq!(byte_test.as_byte().unwrap(), 127);
    } else {
        panic!("`byteTest` not found!");
    }

    if let Some(string_test) = compound.get("stringTest") {
        assert_eq!(
            string_test.as_string().unwrap(),
            mutf8!("HELLO WORLD THIS IS A TEST STRING ÅÄÖ!")
        );
    } else {
        panic!("`stringTest` not found!");
    }

    if let Some(float_test) = compound.get("floatTest") {
        assert_eq!(float_test.as_float(), Some(0.498_231_47));
    } else {
        panic!("`floatTest` not found!");
    }

    if let Some(double_test) = compound.get("doubleTest") {
        assert_eq!(double_test.as_double(), Some(0.493_128_713_218_231_5));
    } else {
        panic!("`doubleTest` not found!");
    }

    if let Some(nested) = compound.get("nested compound test") {
        let nested = nested.as_compound().unwrap();

        let Some(ham) = nested.get("ham") else {
            panic!("`ham` not found!");
        };
        let Some(ham) = ham.as_compound() else {
            panic!("`ham` is not a compound!");
        };
        let Some(name) = ham.get("name") else {
            panic!("`ham` does not contain `name`!");
        };

        assert_eq!(name.as_string().unwrap(), mutf8!("Hampus"));

        let Some(egg) = nested.get("egg") else {
            panic!("`egg` not found!");
        };
        let Some(egg) = egg.as_compound() else {
            panic!("`egg` is not a compound!");
        };
        let Some(name) = egg.get("name") else {
            panic!("`egg` does not contain `name`!");
        };

        assert_eq!(name.as_string().unwrap(), mutf8!("Eggbert"));
    } else {
        panic!("`nested compound test` not found!");
    }
}

#[test]
fn complex_player() {
    static SLICE: &[u8] = include_bytes!("nbt/complex_player.nbt");

    let nbt = IndexedNbtRef::new_named(SLICE).unwrap();
    let _compound = nbt.as_compound();
}
