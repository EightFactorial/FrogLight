//! TODO
#![no_std]

extern crate alloc;

use alloc::borrow::Cow;
use core::range::Range;

use froglight_mutf8::mutf8;
use froglight_nbt::{
    prelude::*,
    types::borrowed::{reference::BorrowedIndex, value::IndexedValue},
};

#[test]
fn read() {
    static SLICE: &[u8] = &[
        0x00, 0x0A, b'e', b'n', b't', b'r', b'y', b'_', b'n', b'a', b'm', b'e', 0x00, 0x0B, b'e',
        b'n', b't', b'r', b'y', b'_', b'v', b'a', b'l', b'u', b'e', 0x00, 0x05, b'S', b'h', b'o',
        b'r', b't', 0x12, 0x34,
    ];
    static ENTRIES: &[IndexedEntry] = unsafe {
        &[
            IndexedEntry::new(BorrowedIndex::new(0), IndexedValue::String(BorrowedIndex::new(12))),
            IndexedEntry::new(BorrowedIndex::new(25), IndexedValue::Short(BorrowedIndex::new(32))),
        ]
    };
    static INDEXES: &[Range<usize>] = &[Range { start: 0, end: ENTRIES.len() }];

    static NBT: IndexedNbtRef<'static> = unsafe {
        IndexedNbtRef::new_unchecked(SLICE, None, Cow::Borrowed(ENTRIES), Cow::Borrowed(INDEXES))
    };

    let compound = NBT.as_compound();

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
