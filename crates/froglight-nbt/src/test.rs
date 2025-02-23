//! Tests for reading and writing NBT data
//!
//! Definitely not taken from `simdnbt` :)
#![allow(unused_imports)]

use std::io::{Cursor, Write};

#[cfg(feature = "io")]
use froglight_io::prelude::*;

use crate::prelude::*;

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
            let bytes = test!(@$reader $file);

            // Parse the NBT data using the slice-based reader
            let (nbt_ref, remainder) = match crate::io::NamedNbtRef::try_new(&bytes) {
                Ok((nbt_ref, remainder)) => (nbt_ref, remainder),
                Err(err) => panic!("Failed to parse NBT data: {err:?}"),
            };
            assert_eq!(nbt_ref.len() + remainder.len(), bytes.len(), "Ref parsed length does not match actual length!");

            // Test using `froglight-io` if the feature is enabled
            #[cfg(feature = "io")]
            {
                // Parse the NBT data using the `froglight-io` reader
                let nbt_io = NamedNbt::frog_read(&mut Cursor::new(&bytes)).unwrap();

                // Compare the expected length to the actual length
                assert_eq!(nbt_io.frog_len(), bytes.len(), "Expected IO length does not match actual length!");

                // Write the NBT data using the `froglight-io` writer and compare the results
                let buf_io: Vec<u8>  = nbt_io.frog_to_buf().unwrap();
                assert_eq!(buf_io.len(), bytes.len(), "IO written length does not match actual length!");
                assert_eq!(buf_io, bytes, "IO written NBT does not match original data!");
            }
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

// Raw NBT
test!(hello_world, "hello_world.nbt");
test!(hypixel, "hypixel.nbt");
test!(inttest1023, "inttest1023.nbt");

// Gzip-compressed NBT
test!(bigtest, gzip, "bigtest.nbt");
test!(level, gzip, "level.dat");
test!(simple_player, gzip, "simple_player.dat");
test!(complex_player, gzip, "complex_player.dat");
