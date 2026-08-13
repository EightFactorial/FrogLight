//! TODO
#![no_std]

extern crate alloc;

use facet::*;
use froglight_snbt::prelude::*;

macro_rules! test {
    ($ty:ident: $($lit:literal => $val:expr),+) => {
        #[test]
        #[expect(non_snake_case, reason = "Using the type as the test name")]
        fn $ty() {
            $({
                static SNBT: &str = $lit;
                let snbt = IndexedSnbtSlice::new_ref(SNBT).unwrap();
                let value = from_snbt_borrowed::<$ty>(&snbt).unwrap();
                assert_eq!(value, $val);
            })+
        }
    };
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Facet)]
struct BasicStruct {
    a: u8,
    b: i8,
    c: u64,
    d: bool,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Facet)]
enum BasicEnum<'a> {
    Byte { a: u8 },
    Short { b: u16 },
    Multiple { a: u8, b: bool },
    Nameless(u32, u32, &'a str),
}

test! {
    BasicStruct:
    "{a:0b,b:255ub,c:0l,d:false}" => BasicStruct { a: 0, b: -1, c: 0, d: false },
    "{a:255b,b:-128sb,c:1l,d:bool(2)}" => BasicStruct { a: 255, b: -128, c: 1, d: true }
}

test! {
    BasicEnum:
    "{a:0b}" => BasicEnum::Byte { a: 0 },
    "{b:0s}" => BasicEnum::Short { b: 0 },
    "{a:0b,b:true}" => BasicEnum::Multiple { a: 0, b: true },
    "{'0':0,'1':1,'2':borrowed}" => BasicEnum::Nameless(0, 1, "borrowed")
}

// -------------------------------------------------------------------------------------------------
