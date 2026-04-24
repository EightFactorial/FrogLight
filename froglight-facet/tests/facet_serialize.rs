//! TODO
#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use core::num::ParseIntError;

use facet::Facet;
use froglight_facet::{to_vec, to_writer, to_writer_variable};

macro_rules! assert_eq_bytes {
    ($ident:ident: $($input:expr),*) => {
        #[test]
        fn $ident() {
            let mut bytes = Vec::new();
            $(
                bytes.clear();
                to_writer(&$input, &mut bytes).unwrap();
                assert_eq!(bytes, $input.to_le_bytes());

                bytes.clear();
                to_writer_variable(&$input, &mut bytes).unwrap();
            )*
        }
    };
}

#[test]
fn bool() {
    assert_eq!(to_vec(&true).unwrap(), [1]);
    assert_eq!(to_vec(&false).unwrap(), [0]);
}

assert_eq_bytes!(u8: 0u8, 1u8, 2u8, 254u8, u8::MAX);
assert_eq_bytes!(u16: 0u16, 1u16, 2u16, 256u16, 257u16, 65534u16, u16::MAX);
assert_eq_bytes!(u32: 0u32, 1u32, 2u32, 256u32, 257u32, 65534u32, u32::MAX);
assert_eq_bytes!(u64: 0u64, 1u64, 2u64, 256u64, 257u64, 65534u64, u64::MAX);
assert_eq_bytes!(u128: 0u128, 1u128, 2u128, 256u128, 257u128, 65536u128, u128::MAX);

assert_eq_bytes!(i8: i8::MIN, -2i8, -1i8, 0i8, 1i8, 2i8, i8::MAX);
assert_eq_bytes!(i16: i16::MIN, -2i16, -1i16, 0i16, 1i16, 2i16, i16::MAX);
assert_eq_bytes!(i32: i32::MIN, -2i32, -1i32, 0i32, 1i32, 2i32, i32::MAX);
assert_eq_bytes!(i64: i64::MIN, -2i64, -1i64, 0i64, 1i64, 2i64, i64::MAX);
assert_eq_bytes!(i128: i128::MIN, -2i128, -1i128, 0i128, 1i128, 2i128, i128::MAX);

// -------------------------------------------------------------------------------------------------

#[test]
#[allow(dead_code, reason = "WIP")]
fn proxy() {
    #[derive(Facet)]
    #[facet(mc::proxy = ByteString)]
    struct Bytes(u8, u8, u8, u8);

    #[derive(Facet)]
    struct ByteString(String);

    impl From<&Bytes> for ByteString {
        fn from(bytes: &Bytes) -> Self {
            Self(alloc::format!("{},{},{},{}", bytes.0, bytes.1, bytes.2, bytes.3))
        }
    }
    impl TryFrom<ByteString> for Bytes {
        type Error = String;

        fn try_from(value: ByteString) -> Result<Self, Self::Error> {
            let f = |res: Result<u8, ParseIntError>| {
                res.map_err(|err| alloc::format!("Failed to parse byte: {err}"))
            };

            let mut iter = value.0.split(',').map(str::parse::<u8>);
            Ok(Self(
                iter.next().map_or_else(|| Err(String::from("Missing byte 0")), f)?,
                iter.next().map_or_else(|| Err(String::from("Missing byte 1")), f)?,
                iter.next().map_or_else(|| Err(String::from("Missing byte 2")), f)?,
                iter.next().map_or_else(|| Err(String::from("Missing byte 3")), f)?,
            ))
        }
    }

    // // Serialize `Bytes`
    // let bytes = Bytes(1, 2, 3, 4);
    // let serialized = to_vec(&bytes).unwrap();
    //
    // // Check that `ByteString` was serialized
    // let (length, content) = serialized.split_first().unwrap();
    // assert_eq!(*length, 7, "Expected length to be `7`, got `{length}`");
    // assert_eq!(content, b"1,2,3,4");
}
