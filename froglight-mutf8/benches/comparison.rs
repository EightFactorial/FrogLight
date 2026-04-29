//! TODO
#![allow(clippy::match_bool, reason = "Readability")]

extern crate alloc;

use alloc::borrow::Cow;
use core::hint::black_box;

use criterion::Criterion;
use froglight_mutf8::prelude::*;
use rand::{distr::Uniform, prelude::*, rngs::Xoshiro128PlusPlus};

fn main() {
    println!("SIMD Module: `{}`", froglight_mutf8::simd::mutf8::ARCH);

    let mut c = Criterion::default().configure_from_args();

    encode(&mut c);
    encode_ascii(&mut c);
    decode(&mut c);
    decode_ascii(&mut c);
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
    froglight_simd => simd_utf8_to_mutf8,
    froglight_fallback => fallback_utf8_to_mutf8,
    froglight_naive => naive_utf8_to_mutf8,
    cesu8 => cesu8::to_java_cesu8,
    simd_cesu8 => simd_cesu8::mutf8::encode
);

bench!(
    encode_ascii = true:
    froglight_simd => simd_utf8_to_mutf8,
    froglight_fallback => fallback_utf8_to_mutf8,
    froglight_naive => naive_utf8_to_mutf8,
    cesu8 => cesu8::to_java_cesu8,
    simd_cesu8 => simd_cesu8::mutf8::encode
);

fn simd_utf8_to_mutf8(str: &str) -> Cow<'_, MStr> {
    match froglight_mutf8::simd::mutf8::contains_null_or_4_byte_header(str.as_bytes()) {
        // SAFETY: `true` means the input was valid MUTF-8.
        true => Cow::Borrowed(unsafe { MStr::from_mutf8_unchecked(str.as_bytes()) }),
        false => Cow::Owned(froglight_mutf8::simd::mutf8::utf8_to_mutf8(str)),
    }
}

fn fallback_utf8_to_mutf8(str: &str) -> Cow<'_, MStr> {
    match froglight_mutf8::simd::mutf8::fallback::contains_null_or_4_byte_header(str.as_bytes()) {
        // SAFETY: `true` means the input was valid MUTF-8.
        true => Cow::Borrowed(unsafe { MStr::from_mutf8_unchecked(str.as_bytes()) }),
        false => Cow::Owned(froglight_mutf8::simd::mutf8::fallback::utf8_to_mutf8(str)),
    }
}

fn naive_utf8_to_mutf8(str: &str) -> Cow<'_, MStr> {
    match froglight_mutf8::types::str::fallback::contains_null_or_4_byte_header(str.as_bytes()) {
        // SAFETY: `true` means the input was valid MUTF-8.
        true => Cow::Borrowed(unsafe { MStr::from_mutf8_unchecked(str.as_bytes()) }),
        false => Cow::Owned(froglight_mutf8::types::string::fallback::utf8_to_mutf8(str)),
    }
}

// -------------------------------------------------------------------------------------------------

fn decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode");
    group.throughput(criterion::Throughput::Elements(1));

    let input: Vec<_> = generate::<false>();
    let input: Vec<_> = input.into_iter().map(MString::from_utf8_owned).collect();

    group.bench_with_input("froglight_simd", &input, |b, input| {
        let mut iter = input.iter().cycle();
        b.iter(|| unsafe {
            black_box(simd_mutf8_to_utf8(black_box(iter.next().unwrap_unchecked().as_ref())));
        });
    });
    group.bench_with_input("froglight_fallback", &input, |b, input| {
        let mut iter = input.iter().cycle();
        b.iter(|| unsafe {
            black_box(fallback_mutf8_to_utf8(black_box(iter.next().unwrap_unchecked().as_ref())));
        });
    });
    group.bench_with_input("froglight_naive", &input, |b, input| {
        let mut iter = input.iter().cycle();
        b.iter(|| unsafe {
            black_box(naive_mutf8_to_utf8(black_box(iter.next().unwrap_unchecked().as_ref())));
        });
    });

    group.bench_with_input("cesu8", &input, |b, input| {
        let mut iter = input.iter().cycle();
        b.iter(|| unsafe {
            black_box(cesu8::from_java_cesu8(black_box(iter.next().unwrap_unchecked().as_ref())))
        });
    });
    group.bench_with_input("simd_cesu8", &input, |b, input| {
        let mut iter = input.iter().cycle();
        b.iter(|| unsafe {
            black_box(simd_cesu8::mutf8::decode(black_box(iter.next().unwrap_unchecked().as_ref())))
        });
    });
}

fn decode_ascii(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_ascii");
    group.throughput(criterion::Throughput::Elements(1));

    let input: Vec<_> = generate::<true>();
    let input: Vec<_> = input.into_iter().map(MString::from_utf8_owned).collect();

    group.bench_with_input("froglight_simd", &input, |b, input| {
        let mut iter = input.iter().cycle();
        b.iter(|| unsafe {
            black_box(simd_mutf8_to_utf8(black_box(iter.next().unwrap_unchecked().as_ref())));
        });
    });
    group.bench_with_input("froglight_fallback", &input, |b, input| {
        let mut iter = input.iter().cycle();
        b.iter(|| unsafe {
            black_box(fallback_mutf8_to_utf8(black_box(iter.next().unwrap_unchecked().as_ref())));
        });
    });
    group.bench_with_input("froglight_naive", &input, |b, input| {
        let mut iter = input.iter().cycle();
        b.iter(|| unsafe {
            black_box(naive_mutf8_to_utf8(black_box(iter.next().unwrap_unchecked().as_ref())));
        });
    });

    group.bench_with_input("cesu8", &input, |b, input| {
        let mut iter = input.iter().cycle();
        b.iter(|| unsafe {
            black_box(cesu8::from_java_cesu8(black_box(iter.next().unwrap_unchecked().as_ref())))
        });
    });
    group.bench_with_input("simd_cesu8", &input, |b, input| {
        let mut iter = input.iter().cycle();
        b.iter(|| unsafe {
            black_box(simd_cesu8::mutf8::decode(black_box(iter.next().unwrap_unchecked().as_ref())))
        });
    });
}

fn simd_mutf8_to_utf8(str: &MStr) -> Cow<'_, str> {
    match froglight_mutf8::simd::mutf8::contains_null_or_4_byte_header(str.as_bytes()) {
        // SAFETY: `true` means the input was valid UTF-8.
        true => Cow::Borrowed(unsafe { str::from_utf8_unchecked(str.as_bytes()) }),
        false => Cow::Owned(froglight_mutf8::simd::mutf8::mutf8_to_utf8(str)),
    }
}

fn fallback_mutf8_to_utf8(str: &MStr) -> Cow<'_, str> {
    match froglight_mutf8::simd::mutf8::fallback::contains_null_or_4_byte_header(str.as_bytes()) {
        // SAFETY: `true` means the input was valid UTF-8.
        true => Cow::Borrowed(unsafe { str::from_utf8_unchecked(str.as_bytes()) }),
        false => Cow::Owned(froglight_mutf8::simd::mutf8::fallback::mutf8_to_utf8(str)),
    }
}

fn naive_mutf8_to_utf8(str: &MStr) -> Cow<'_, str> {
    match froglight_mutf8::types::str::fallback::contains_null_or_4_byte_header(str.as_bytes()) {
        // SAFETY: `true` means the input was valid UTF-8.
        true => Cow::Borrowed(unsafe { str::from_utf8_unchecked(str.as_bytes()) }),
        false => Cow::Owned(froglight_mutf8::types::string::fallback::mutf8_to_utf8(str)),
    }
}

// -------------------------------------------------------------------------------------------------

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
