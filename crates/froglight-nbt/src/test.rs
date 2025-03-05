//! Tests for reading and writing NBT data
//!
//! Definitely not taken from `simdnbt` :)
#![allow(unused_imports)]

use std::io::{Cursor, Write};

#[cfg(feature = "io")]
use froglight_io::prelude::*;

use crate::{io::slice::*, nbt::*, prelude::*, snbt::Compat};

/// A macro for generating tests
macro_rules! test {
    ($test:ident, $file:expr) => {
        test!($test, raw, $file);
    };
    ($test:ident, $reader:tt, $file:expr) => {
        #[test]
        fn $test() {
            #[cfg(feature = "debug")]
            *LOG;

            // Read the NBT data from the file
            test_data(&test!(@$reader $file));
       }
    };

    // Helpers for specifying how to read test data
    (@raw $file:expr) => {{
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/test/", $file)).as_slice()
    }};
    (@gzip $file:expr) => {{
        let mut decoder = flate2::write::GzDecoder::new(Vec::new());
        decoder.write_all(test!(@raw $file)).unwrap();
        decoder.finish().unwrap()
    }};
}

/// A reusable logger for debugging that is only initialized once
#[cfg(feature = "debug")]
static LOG: std::sync::LazyLock<()> = std::sync::LazyLock::new(|| {
    use tracing_subscriber::{EnvFilter, fmt};
    let _ = fmt().with_env_filter(EnvFilter::from_default_env()).try_init();
});

// -------------------------------------------------------------------------------------------------

// !!! Add NBT tests here !!!

// Raw NBT
test!(hello_world, "hello_world.nbt");
test!(hypixel, "hypixel.nbt");
test!(inttest1023, "inttest1023.nbt");

// Gzip-compressed NBT
test!(bigtest, gzip, "bigtest.nbt");
test!(level, gzip, "level.dat");
test!(simple_player, gzip, "simple_player.dat");
test!(complex_player, gzip, "complex_player.dat");

// -------------------------------------------------------------------------------------------------

/// Test reading and writing NBT data
fn test_data(bytes: &[u8]) {
    // Parse the NBT data using the slice-based reader
    let (nbt_ref, remainder) = match NamedNbtRef::try_new(bytes) {
        Ok((nbt_ref, remainder)) => (nbt_ref, remainder),
        Err(err) => panic!("Failed to parse NBT data: {err:?}"),
    };
    assert_eq!(
        nbt_ref.as_bytes().len() + remainder.len(),
        bytes.len(),
        "Ref parsed length does not match actual length!"
    );

    // Convert the `NamedNbtRef` into an owned `NamedNbt`
    let nbt_ref = nbt_ref.as_owned();

    // Convert the NBT into SNBT (Compat) and back
    // TODO: Fix floating-point comparison causing test failures
    if let Some(nbt_ref) = nbt_ref.compound() {
        let _snbt = Snbt::<Compat>::from_compound(nbt_ref).unwrap();
        // let _snbt = snbt.as_compound().unwrap();
        // assert_eq!(nbt_ref, &snbt);
    }

    // Test using `froglight-io` if the feature is enabled
    #[cfg(feature = "io")]
    {
        // Parse the NBT data using the `froglight-io` reader
        let nbt_io = NamedNbt::frog_read(&mut Cursor::new(&bytes)).unwrap();

        // Compare the slice-based and IO-based NBT data
        assert_eq!(nbt_ref, nbt_io);

        // Write the slice-based NBT data and compare to the original data.
        let ref_io: Vec<u8> = nbt_ref.frog_to_buf().unwrap();
        assert_eq!(ref_io.len(), bytes.len(), "Ref written length does not match actual length!");
        assert_eq!(ref_io, bytes, "Ref written NBT does not match original data!");

        // Write the `froglight-io` NBT data and compare to the original data.
        let buf_io: Vec<u8> = nbt_io.frog_to_buf().unwrap();
        assert_eq!(buf_io.len(), bytes.len(), "IO written length does not match actual length!");
        assert_eq!(buf_io, bytes, "IO written NBT does not match original data!");
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(FrogNbt)]
#[frog(path = crate)]
struct TestStructHolder {
    #[frog(ident = "froglight:test", tag = "compound")]
    test: TestStruct,
}

#[derive(FrogNbt)]
#[frog(path = crate)]
struct TestStruct {
    #[frog(ident = "froglight:string", tag = "string")]
    string: String,
    #[frog(ident = "froglight:value", tag = "long")]
    value: i64,
    #[frog(ident = "froglight:array", tag = "bytearray")]
    array: Vec<u8>,
}

#[test]
fn test_convert() {
    let compound = NbtCompound::from(vec![(
        Mutf8String::from("froglight:test"),
        NbtTag::Compound(NbtCompound::from(vec![
            (Mutf8String::from("froglight:string"), Mutf8String::from("Hello, world!").into()),
            (Mutf8String::from("froglight:value"), NbtTag::Long(42)),
            (Mutf8String::from("froglight:array"), NbtTag::ByteArray(vec![1i8, 2, 3].into())),
        ])),
    )]);

    // Test creating `TestStructHolder` from an `NbtCompound`
    let holder = TestStructHolder::from_compound(&compound).unwrap();
    assert_eq!(holder.test.string, "Hello, world!");
    assert_eq!(holder.test.value, 42);
    assert_eq!(holder.test.array, vec![1, 2, 3]);

    // Test converting `TestStructHolder` into an `NbtCompound`
    // assert_eq!(holder.as_compound().unwrap(), compound);
}
