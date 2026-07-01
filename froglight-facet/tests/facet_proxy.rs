//! TODO
#![no_std]

extern crate alloc;

use alloc::string::String;
use core::num::ParseIntError;

use facet::Facet;
use froglight_facet::{from_slice, to_vec};

#[test]
fn proxy() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Facet)]
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
                res.map_err(|err| alloc::format!("Failed to parse byte, {err}"))
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

    // Serialize `Bytes`
    let bytes = Bytes(1, 2, 3, 4);
    let serialized = to_vec(&bytes, 0).unwrap();

    // Check that `ByteString` was serialized
    let (length, content) = serialized.split_first().unwrap();
    assert_eq!(*length, 7, "Expected length to be `7`, got `{length}`");
    assert_eq!(content, b"1,2,3,4");

    // Check that `Bytes` can be deserialized
    let deserialized: Bytes = from_slice(&serialized, 0).unwrap();
    assert_eq!(deserialized, bytes);
}
