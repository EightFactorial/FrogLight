//! Early benchmarks
//!
//! Batch Size: 512000
//!
//! Ryzen 5800X with `nightly`:
//!
//! Encode UTF8:
//!  - MString::from_utf8_simd : 508.133649ms
//!  - simd_cesu8::mutf8::encode : 591.457994ms
//!  - cesu8::to_java_cesu8 : 765.776418ms
//!
//! Encode ASCII:
//!  - MString::from_utf8_simd : 10.781467ms
//!  - simd_cesu8::mutf8::encode : 11.615085ms
//!  - cesu8::to_java_cesu8 : 69.587603ms
//!
//! Decode UTF8:
//!  - MString::to_utf8_simd : 698.068462ms
//!  - simd_cesu8::mutf8::decode : 620.230279ms
//!  - cesu8::from_java_cesu8 : 659.970129ms
//!
//! Decode ASCII:
//!  - MString::to_utf8_simd : 9.820759ms
//!  - simd_cesu8::mutf8::decode : 11.594746ms
//!  - cesu8::from_java_cesu8 : 11.352276ms
//!
//! Ryzen 5800X without `nightly`:
//!
//! Encode UTF8:
//!  - MString::from_utf8_simd : 537.819819ms
//!  - simd_cesu8::mutf8::encode : 592.939366ms
//!  - cesu8::to_java_cesu8 : 775.57945ms
//!
//! Encode ASCII:
//!  - MString::from_utf8_simd : 10.855477ms
//!  - simd_cesu8::mutf8::encode : 16.61744ms
//!  - cesu8::to_java_cesu8 : 42.104611ms
//!
//! Decode UTF8:
//!  - MString::to_utf8_simd : 729.294534ms
//!  - simd_cesu8::mutf8::decode : 633.969376ms
//!  - cesu8::from_java_cesu8 : 655.613611ms
//!
//! Decode ASCII:
//!  - MString::to_utf8_simd : 9.714378ms
//!  - simd_cesu8::mutf8::decode : 11.628525ms
//!  - cesu8::from_java_cesu8 : 11.165155ms

use core::hint::black_box;
use std::time::Instant;

use froglight_mutf8::prelude::*;
use rand::{distr::Uniform, prelude::*, rngs::Xoshiro128PlusPlus};

macro_rules! time {
    ($fn:path $([ $($arg:expr),* ])? => $input:expr) => {{
        let start = Instant::now();
        for input in &$input {
            let _value = black_box($fn( $($($arg),* ,)? black_box(input)));
        }
        println!(" - {} : {:?}", stringify!($fn), start.elapsed());
    }};
    (@ref $fn:path as $ty:ty $([ $($arg:expr),* ])? => $input:expr) => {{
        let start = Instant::now();
        for input in &$input {
            let input: &$ty = input.as_ref();
            let _value = black_box($fn( $($($arg),* ,)? black_box(input)));
        }
        println!(" - {} : {:?}", stringify!($fn), start.elapsed());
    }};
    (@dispatch $fn:path => $input:expr) => {{
        let start = Instant::now();
        fearless_simd::dispatch!(fearless_simd::Level::new(), simd => {
            for input in &$input {
                let _value = black_box($fn(simd, black_box(input)));
            }
        });
        println!(" - {} : {:?}", stringify!($fn), start.elapsed());
    }};
}

fn main() {
    println!("Batch Size: {GENERATE_SIZE}\n");

    {
        println!("Encode UTF8:");
        let input = generate::<false>();
        time!(@dispatch MString::from_utf8_simd => input);
        time!(simd_cesu8::mutf8::encode => input);
        time!(cesu8::to_java_cesu8 => input);
    }

    {
        println!("Encode ASCII:");
        let input = generate::<true>();
        time!(@dispatch MString::from_utf8_simd => input);
        time!(simd_cesu8::mutf8::encode => input);
        time!(cesu8::to_java_cesu8 => input);
    }

    {
        println!("Decode UTF8:");
        let input =
            generate::<false>().into_iter().map(MString::from_utf8_owned).collect::<Vec<_>>();
        time!(@dispatch MString::to_utf8_simd => input);
        time!(@ref simd_cesu8::mutf8::decode as [u8] => input);
        time!(@ref cesu8::from_java_cesu8 as [u8] => input);
    }

    {
        println!("Decode ASCII:");
        let input =
            generate::<true>().into_iter().map(MString::from_utf8_owned).collect::<Vec<_>>();
        time!(@dispatch MString::to_utf8_simd => input);
        time!(@ref simd_cesu8::mutf8::decode as [u8] => input);
        time!(@ref cesu8::from_java_cesu8 as [u8] => input);
    }
}

// -------------------------------------------------------------------------------------------------

/// How many random strings to generate for tests.
const GENERATE_SIZE: usize = 512_000;
const STRING_LENGTH: u32 = 512;

/// Generate a set of random strings using a fixed seed.
fn generate<const ASCII: bool>() -> Vec<String> {
    let mut rand = Xoshiro128PlusPlus::seed_from_u64(0x0655_E4BA_22F5_A61E);
    let ascii = Uniform::<char>::new_inclusive(0x01 as char, 0x7F as char).unwrap();

    let mut input = Vec::with_capacity(GENERATE_SIZE);

    for _ in 0..GENERATE_SIZE {
        let length = rand.next_u32() % STRING_LENGTH;
        let mut string = String::with_capacity(length as usize);

        for _ in 0..length {
            if ASCII {
                string.push(rand.sample(ascii));
            } else {
                string.push(rand.random::<char>());
            }
        }

        input.push(string);
    }

    input
}
