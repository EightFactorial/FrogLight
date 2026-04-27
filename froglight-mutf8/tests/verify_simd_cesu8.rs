//! TODO

use froglight_mutf8::prelude::MString;
use rand::{distr::Uniform, prelude::*, rngs::Xoshiro128PlusPlus};

#[test]
fn simd_cesu8_ascii() {
    for string in generate::<true>() {
        let mstr = MString::from_utf8(&string);
        let cesu8 = simd_cesu8::mutf8::encode(&string);

        pretty_assertions::assert_str_eq!(
            format!("{:?}", mstr.as_bytes()),
            format!("{:?}", cesu8.as_ref()),
            "Froglight and SIMD-CESU-8 outputs differ for string: {string:?}"
        );
    }
}

#[test]
fn simd_cesu8_utf8() {
    for string in generate::<false>() {
        let mstr = MString::from_utf8(&string);
        let cesu8 = simd_cesu8::mutf8::encode(&string);

        pretty_assertions::assert_str_eq!(
            format!("{:?}", mstr.as_bytes()),
            format!("{:?}", cesu8.as_ref()),
            "Froglight and SIMD-CESU-8 outputs differ for string: {string:?}"
        );
    }
}

// -------------------------------------------------------------------------------------------------

/// How many random strings to generate for tests.
const GENERATE_SIZE: usize = 128_000;

/// Generate a set of random strings using a fixed seed.
fn generate<const ASCII: bool>() -> Vec<String> {
    let mut rand = Xoshiro128PlusPlus::seed_from_u64(0x0655_E4BA_22F5_A61D);
    let ascii = Uniform::<char>::new_inclusive(0x01 as char, 0x7F as char).unwrap();

    let mut input = Vec::with_capacity(GENERATE_SIZE);

    for _ in 0..GENERATE_SIZE {
        let length = rand.next_u32() % 512;
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
