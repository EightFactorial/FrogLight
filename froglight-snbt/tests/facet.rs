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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Facet)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Facet)]
struct BasicNested<'a> {
    first: u32,
    second: u32,
    third: BasicEnum<'a>,
}

test! {
    BasicNested:
    "{first:0,second:0,third:{a:0b}}" => BasicNested { first: 0, second: 0, third: BasicEnum::Byte { a: 0 }},
    "{first:1i,second:2i,third:{a:3b}}" => BasicNested { first: 1, second: 2, third: BasicEnum::Byte { a: 3 }},
    "{first:255ui,\"second\":0si,third:{a:255ub,b:bool(0b)}}" => BasicNested { first: 255, second: 0, third: BasicEnum::Multiple { a: 255, b: false }}
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Facet)]
struct BasicRename<'a> {
    second: u32,
    first: u32,
    #[facet(rename = "third")]
    c: BasicEnum<'a>,
}

test! {
    BasicRename:
    "{first:0,second:0,third:{a:0b}}" => BasicRename { first: 0, second: 0, c: BasicEnum::Byte { a: 0 }},
    "{first:1i,second:2i,third:{a:3b}}" => BasicRename { first: 1, second: 2, c: BasicEnum::Byte { a: 3 }},
    "{first:255ui,\"second\":0si,third:{a:255ub,b:bool(0b)}}" => BasicRename { first: 255, second: 0, c: BasicEnum::Multiple { a: 255, b: false }}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Facet)]
#[facet(rename_all = "UPPERCASE")]
struct BasicRenameAll<'a> {
    second: u32,
    first: u32,
    third: BasicEnum<'a>,
}

test! {
    BasicRenameAll:
    "{FIRST:0,SECOND:0,THIRD:{a:0b}}" => BasicRenameAll { first: 0, second: 0, third: BasicEnum::Byte { a: 0 }},
    "{FIRST:1i,SECOND:2i,THIRD:{a:3b}}" => BasicRenameAll { first: 1, second: 2, third: BasicEnum::Byte { a: 3 }},
    "{FIRST:255ui,\"SECOND\":0si,THIRD:{a:255ub,b:bool(0b)}}" => BasicRenameAll { first: 255, second: 0, third: BasicEnum::Multiple { a: 255, b: false }}
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Facet)]
struct BasicFlattened<'a> {
    first: u32,
    second: u32,
    #[facet(flatten)]
    third: BasicEnum<'a>,
}

test! {
    BasicFlattened:
    "{first:0,second:0,a:0b}" => BasicFlattened { first: 0, second: 0, third: BasicEnum::Byte { a: 0 }},
    "{first:1i,second:2i,a:3b}" => BasicFlattened { first: 1, second: 2, third: BasicEnum::Byte { a: 3 }},
    "{first:255ui,\"second\":0si,a:255ub,b:bool(0b)}" => BasicFlattened { first: 255, second: 0, third: BasicEnum::Multiple { a: 255, b: false }}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Facet)]
struct MultipleFlattened {
    first: u32,
    #[facet(flatten)]
    second: BasicStruct,
    #[facet(flatten)]
    third: BasicStruct,
}

test! {
    MultipleFlattened:
    "{first:0,a:0b,b:0b,c:0l,d:false}" => MultipleFlattened { first: 0, second: BasicStruct { a: 0, b: 0, c: 0, d: false }, third: BasicStruct { a: 0, b: 0, c: 0, d: false } },
    "{first:123,a:255b,b:0b,c:1l,'d':bool(1l)}" => MultipleFlattened { first: 123, second: BasicStruct { a: 255, b: 0, c: 1, d: true }, third: BasicStruct { a: 255, b: 0, c: 1, d: true } }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Facet)]
struct BasicAmbiguous {
    a: u32,
    b: AmbiguousEnum,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Facet)]
enum AmbiguousEnum {
    A { second: u32 },
    B { second: u32 },
    C { second: u32, third: u32 },
}

test! {
    BasicAmbiguous:
    "{a:0,b:{second:0}}" => BasicAmbiguous { a: 0, b: AmbiguousEnum::A { second: 0 }},
    "{a:1,b:{second:2}}" => BasicAmbiguous { a: 1, b: AmbiguousEnum::A { second: 2 }},
    "{a:255,b:{\"second\":0}}" => BasicAmbiguous { a: 255, b: AmbiguousEnum::A { second: 0 }},
    "{a:255,b:{'second':0,'third':0}}" => BasicAmbiguous { a: 255, b: AmbiguousEnum::C { second: 0, third: 0 }}
}
