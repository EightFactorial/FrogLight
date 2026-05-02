//! TODO
#![no_std]

extern crate alloc;

use alloc::borrow::Cow;

use froglight_mutf8::mutf8;
use froglight_nbt::{
    prelude::*,
    types::borrowed::{IndexedEntry, IndexedTag, NbtIndex, Ref},
};

#[test]
fn read() {
    static SLICE: &[u8] = &[
        0x00, 0x0A, b'e', b'n', b't', b'r', b'y', b'_', b'n', b'a', b'm', b'e', 0x00, 0x0B, b'e',
        b'n', b't', b'r', b'y', b'_', b'v', b'a', b'l', b'u', b'e', 0x00, 0x05, b'S', b'h', b'o',
        b'r', b't', 0x12, 0x34,
    ];
    static ENTRIES: &[IndexedEntry<'static, Ref>] = unsafe {
        &[
            IndexedEntry::new_unchecked(NbtIndex::new(0), IndexedTag::String(NbtIndex::new(12))),
            IndexedEntry::new_unchecked(NbtIndex::new(25), IndexedTag::Short(NbtIndex::new(32))),
        ]
    };

    static NBT: IndexedCompound<'static, Ref> =
        unsafe { IndexedCompound::new_unchecked(SLICE, Cow::Borrowed(ENTRIES)) };

    if let Some(entry) = NBT.get("entry_name") {
        assert_eq!(entry.as_string().unwrap(), mutf8!("entry_value"));
    } else {
        panic!("`entry_name` not found!");
    }

    if let Some(entry) = NBT.get("Short") {
        assert_eq!(entry.as_short().unwrap(), 0x1234);
    } else {
        panic!("`Short` not found!");
    }

    for (index, (name, entry)) in NBT.iter().enumerate() {
        match index {
            0 => {
                assert_eq!(name.get(), mutf8!("entry_name"));
                assert_eq!(entry.as_string().unwrap(), mutf8!("entry_value"));
            }
            1 => {
                assert_eq!(name.get(), mutf8!("Short"));
                assert_eq!(entry.as_short().unwrap(), 0x1234);
            }
            _ => unreachable!(),
        }
    }
}
