//! Tests for reading and writing NBT data
//!
//! Definitely not taken from `simdnbt` :)
#![allow(unused_imports)]

use std::io::{Cursor, Write};

#[cfg(feature = "io")]
use froglight_io::prelude::*;

use crate::{io::reference::*, nbt::*};

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

    // Test using `froglight-io` if the feature is enabled
    #[cfg(feature = "io")]
    {
        // Parse the NBT data using the `froglight-io` reader
        let nbt_io = NamedNbt::frog_read(&mut Cursor::new(&bytes)).unwrap();

        // Compare the NBT object names and lengths
        assert_eq!(nbt_ref.name(), nbt_io.name(), "Ref name does not match IO name!");
        assert_eq!(
            nbt_io.frog_len(),
            bytes.len(),
            "Expected IO length does not match actual length!"
        );

        // Compare the Ref and IO NBT tags
        if let Some(ref_compound) = nbt_ref.compound() {
            test_compound(ref_compound, nbt_io.compound().expect("IO is missing compound tag!"));
        }

        // Write the NBT data using the `froglight-io` writer and compare the results
        let buf_io: Vec<u8> = nbt_io.frog_to_buf().unwrap();
        assert_eq!(buf_io.len(), bytes.len(), "IO written length does not match actual length!");
        assert_eq!(buf_io, bytes, "IO written NBT does not match original data!");
    }
}

/// Test that a [`NbtCompoundRef`] matches a [`NbtCompound`]
fn test_compound(ref_compound: NbtCompoundRef, io_compound: &NbtCompound) {
    let mut tag_count = 0;
    for (ref_name, ref_tag) in &ref_compound {
        let io_tag = io_compound.get_tag_bytes(ref_name).unwrap_or_else(|| {
            panic!("Ref \"{}\" is missing tag from IO!", ref_name.to_str_lossy())
        });

        tag_count += 1;
        test_tag(ref_tag, io_tag);
    }

    assert_eq!(tag_count, io_compound.len(), "Ref is missing tags from IO!");
}

/// Test that a [`NbtTagRef`] matches a [`NbtTag`]
fn test_tag(ref_tag: NbtTagRef, io_tag: &NbtTag) {
    match ref_tag.tag_data() {
        NbtTagRefData::Byte(ref_val) => {
            assert_eq!(io_tag.as_byte().unwrap(), ref_val, "Ref byte does not match IO byte!");
        }
        NbtTagRefData::Short(ref_val) => {
            assert_eq!(io_tag.as_short().unwrap(), ref_val, "Ref short does not match IO short!");
        }
        NbtTagRefData::Int(ref_val) => {
            assert_eq!(io_tag.as_int().unwrap(), ref_val, "Ref int does not match IO int!");
        }
        NbtTagRefData::Long(ref_val) => {
            assert_eq!(io_tag.as_long().unwrap(), ref_val, "Ref long does not match IO long!");
        }
        NbtTagRefData::Float(ref_val) => {
            assert_eq!(
                io_tag.as_float().unwrap().total_cmp(&ref_val),
                std::cmp::Ordering::Equal,
                "Ref float does not match IO float!"
            );
        }
        NbtTagRefData::Double(ref_val) => {
            assert_eq!(
                io_tag.as_double().unwrap().total_cmp(&ref_val),
                std::cmp::Ordering::Equal,
                "Ref double does not match IO double!"
            );
        }
        NbtTagRefData::ByteArray(ref_val) => {
            assert_eq!(
                io_tag.as_byte_array().unwrap(),
                ref_val.into_iter().collect::<Vec<_>>(),
                "Ref byte array does not match IO byte array!"
            );
        }
        NbtTagRefData::String(ref_val) => {
            assert_eq!(
                io_tag.as_string().unwrap(),
                &ref_val.to_mutf8_string(),
                "Ref string does not match IO string!"
            );
        }
        NbtTagRefData::List(ref_val) => {
            test_list(ref_val, io_tag.as_list().expect("IO is missing list tag!"));
        }
        NbtTagRefData::Compound(ref_val) => {
            test_compound(ref_val, io_tag.as_compound().expect("IO is missing compound tag!"));
        }
        NbtTagRefData::IntArray(ref_val) => {
            assert_eq!(
                io_tag.as_int_array().unwrap(),
                ref_val.into_iter().collect::<Vec<_>>(),
                "Ref int array does not match IO int array!"
            );
        }
        NbtTagRefData::LongArray(ref_val) => {
            assert_eq!(
                io_tag.as_long_array().unwrap(),
                ref_val.into_iter().collect::<Vec<_>>(),
                "Ref long array does not match IO long array!"
            );
        }
    }
}

/// Test that a [`NbtListTagRef`] matches a [`NbtListTag`]
///
/// TODO: Implement this function
fn test_list(_ref_list: NbtListTagRef, _io_list: &NbtListTag) {}
