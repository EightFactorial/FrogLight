//! TODO

use core::hint::black_box;

use criterion::Criterion;
use rand::{distr::Uniform, prelude::*, rngs::Xoshiro128PlusPlus};

fn main() {
    println!("SIMD Module: `{}`", froglight_mutf8::simd::mutf8::ARCH);

    let mut c = Criterion::default().configure_from_args();

    encode(&mut c);
    encode_ascii(&mut c);
    valid_mutf8(&mut c);

    c.final_summary();
}

// -------------------------------------------------------------------------------------------------

/// How many random strings to generate for tests.
const GENERATE_SIZE: usize = 512_000;

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

macro_rules! bench {
    ( $group:ident = $ascii:literal : $($name:ident => $fn:path),* ) => {
        fn $group(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($group));
            group.throughput(criterion::Throughput::Elements(1));

            let input = generate::<$ascii>();

            $(
                group.bench_with_input(stringify!($name), &input, |b, input| {
                    let mut iter = input.iter().cycle();
                    b.iter(|| unsafe { black_box($fn(black_box(iter.next().unwrap_unchecked().as_ref()))) });
                });
            )*
        }
    };
}

bench!(
    encode = false:
    froglight_simd => froglight_mutf8::simd::mutf8::utf8_to_mutf8,
    froglight_fallback => froglight_mutf8::simd::mutf8::fallback::utf8_to_mutf8,
    froglight_naive => froglight_mutf8::types::string::fallback::utf8_to_mutf8,
    cesu8 => cesu8::to_java_cesu8,
    simd_cesu8 => simd_cesu8::mutf8::encode
);

bench!(
    encode_ascii = true:
    froglight_simd => froglight_mutf8::simd::mutf8::utf8_to_mutf8,
    froglight_fallback => froglight_mutf8::simd::mutf8::fallback::utf8_to_mutf8,
    froglight_naive => froglight_mutf8::types::string::fallback::utf8_to_mutf8,
    cesu8 => cesu8::to_java_cesu8,
    simd_cesu8 => simd_cesu8::mutf8::encode
);

bench!(
    valid_mutf8 = true:
    froglight_simd => froglight_mutf8::simd::mutf8::contains_null_or_4_byte_header,
    froglight_fallback => froglight_mutf8::simd::mutf8::fallback::contains_null_or_4_byte_header,
    froglight_naive => froglight_mutf8::types::str::fallback::contains_null_or_4_byte_header,
    cesu8 => cesu8_is_valid,
    simd_cesu8 => simd_cesu8::implementation::active::contains_null_or_utf8_4_byte_char_header
);

fn cesu8_is_valid(bytes: &[u8]) -> bool {
    simdutf8::basic::from_utf8(bytes).is_ok_and(cesu8::is_valid_java_cesu8)
}
