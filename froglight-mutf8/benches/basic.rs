//! TODO

use core::hint::black_box;
use std::time::Instant;

use froglight_mutf8::prelude::*;
use rand::{distr::Uniform, prelude::*, rngs::Xoshiro128PlusPlus};

macro_rules! time {
    ($fn:path => $input:expr) => {{
        let start = Instant::now();
        for string in &$input {
            black_box($fn(string));
        }
        println!(" - {}: {:?}", stringify!($fn), start.elapsed());
    }};
}

fn main() {
    println!("Batch Size: {GENERATE_SIZE}\n");

    {
        println!("Encode UTF8:");
        let input = generate::<false>();
        time!(MString::from_utf8 => input);
        time!(simd_cesu8::mutf8::encode => input);
        time!(cesu8::to_java_cesu8 => input);
    }

    {
        println!("Encode ASCII:");
        let input = generate::<true>();
        time!(MString::from_utf8 => input);
        time!(simd_cesu8::mutf8::encode => input);
        time!(cesu8::to_java_cesu8 => input);
    }
}

// -------------------------------------------------------------------------------------------------

/// How many random strings to generate for tests.
const GENERATE_SIZE: usize = 512_000;

/// Generate a set of random strings using a fixed seed.
fn generate<const ASCII: bool>() -> Vec<String> {
    let mut rand = Xoshiro128PlusPlus::seed_from_u64(0x0655_E4BA_22F5_A61E);
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
