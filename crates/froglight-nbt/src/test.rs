//! Tests for reading and writing NBT data
//!
//! Definitely not taken from `simdnbt` :)

use std::io::{Cursor, Write};

#[cfg(feature = "io")]
use froglight_io::prelude::*;

use crate::prelude::*;

/// A macro for generating tests
macro_rules! test {
    ($test:ident, $file:expr) => {
        test!($test, data, $file);
    };
    ($test:ident, $read:tt, $file:expr) => {
        #[test]
        #[cfg(feature = "io")]
        fn $test() {
            #[cfg(feature = "debug")]
            *LOG;

            // Read the NBT data from the file
            let bytes = test!(@$read $file);
            let data = NamedNbt::frog_read(&mut Cursor::new(&bytes)).unwrap();

            // Write the NBT data to a buffer and compare it to the original
            // let buffer: Vec<u8> = data.frog_to_buf().unwrap();
            // assert_eq!(buffer, bytes, "Written NBT does not match original data!");
            assert_eq!(data.frog_len(), bytes.len(), "Predicted length does not match actual length!");
        }
    };

    // Helpers for specifying how to read test data
    (@data $file:expr) => {{
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/test/", $file)).as_slice()
    }};
    (@gzip $file:expr) => {{
        let mut decoder = flate2::write::GzDecoder::new(Vec::new());
        decoder.write_all(test!(@data $file)).unwrap();
        decoder.finish().unwrap()
    }};
}

/// A reusable logger for debugging that is only initialized once
#[cfg(feature = "debug")]
static LOG: std::sync::LazyLock<()> = std::sync::LazyLock::new(|| {
    use tracing_subscriber::{EnvFilter, fmt};
    let _ = fmt().with_env_filter(EnvFilter::from_default_env()).try_init();
});

test!(hello_world, "hello_world.nbt");
test!(hypixel, "hypixel.nbt");
test!(inttest1023, "inttest1023.nbt");

test!(bigtest, gzip, "bigtest.nbt");
test!(level, gzip, "level.dat");
test!(simple_player, gzip, "simple_player.dat");
test!(complex_player, gzip, "complex_player.dat");
