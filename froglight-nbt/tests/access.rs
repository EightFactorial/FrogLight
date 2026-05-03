//! TODO
#![no_std]

extern crate alloc;

use froglight_mutf8::mutf8;
use froglight_nbt::prelude::*;

#[test]
fn read() {
    static SLICE: &[u8] = &[
        0x00, 0x0A, b'e', b'n', b't', b'r', b'y', b'_', b'n', b'a', b'm', b'e', 0x00, 0x0B, b'e',
        b'n', b't', b'r', b'y', b'_', b'v', b'a', b'l', b'u', b'e', 0x00, 0x05, b'S', b'h', b'o',
        b'r', b't', 0x12, 0x34,
    ];
    static ENTRIES: &[IndexedEntry] = unsafe {
        &[
            IndexedEntry::new(BorrowedIndex::new(0), BorrowedIndex::new(12)),
            IndexedEntry::new(BorrowedIndex::new(25), BorrowedIndex::new(32)),
        ]
    };

    static NBT: IndexedCompoundRef<'static> = unsafe { IndexedCompoundRef::new(SLICE, ENTRIES) };

    if let Some(_entry) = NBT.get("entry_name") {
        // assert_eq!(entry.as_string().unwrap(), mutf8!("entry_value"));
    } else {
        panic!("`entry_name` not found!");
    }

    if let Some(_entry) = NBT.get("Short") {
        // assert_eq!(entry.as_short().unwrap(), 0x1234);
    } else {
        panic!("`Short` not found!");
    }

    for (index, entry) in NBT.iter().enumerate() {
        match index {
            0 => {
                assert_eq!(entry.name().get_ref(), mutf8!("entry_name"));
                // assert_eq!(entry.as_string().unwrap(),
                // mutf8!("entry_value"));
            }
            1 => {
                assert_eq!(entry.name().get_ref(), mutf8!("Short"));
                // assert_eq!(entry.as_short().unwrap(), 0x1234);
            }
            _ => unreachable!(),
        }
    }
}
