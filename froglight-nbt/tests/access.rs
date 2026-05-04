//! TODO
#![no_std]

extern crate alloc;
extern crate std;

use froglight_mutf8::mutf8;
use froglight_nbt::prelude::*;

#[test]
fn read() {
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
