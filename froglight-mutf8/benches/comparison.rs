//! TODO
#![allow(clippy::match_bool, reason = "Readability")]

extern crate alloc;

use core::hint::black_box;

use criterion::Criterion;
use fearless_simd::{Level, dispatch};
use froglight_mutf8::prelude::*;
use rand::{distr::Uniform, prelude::*, rngs::Xoshiro128PlusPlus};

fn main() {
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
    ( $group:ident = $ascii:literal : $( $(@$tag:tt)? $name:ident => $fn:path),* ) => {
        fn $group(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($group));
            group.throughput(criterion::Throughput::Elements(1));

            let input = generate::<$ascii>();

            $( bench!($(@$tag)? group, input, $name => $fn); )*
        }
    };
    ( $group:expr, $input:expr, $name:ident => $fn:path ) => {
        $group.bench_with_input(stringify!($name), &$input, |b, input| {
            let mut iter = input.iter().cycle();
            b.iter(|| unsafe { black_box($fn(black_box(iter.next().unwrap_unchecked().as_ref()))) });
        });
    };
    ( @dispatch $group:expr, $input:expr, $name:ident => $fn:path ) => {
        $group.bench_with_input(stringify!($name), &$input, |b, input| {
            let mut iter = input.iter().cycle();
            dispatch!(Level::new(), simd => {
                b.iter(|| unsafe { black_box($fn(simd, black_box(iter.next().unwrap_unchecked().as_ref()))) });
            });
        });
    };
}

bench! {
    encode = false:
    froglight_mutf8 => MString::from_utf8,
    cesu8 => cesu8::to_java_cesu8,
    simd_cesu8 => simd_cesu8::mutf8::encode
}

bench! {
    encode_ascii = true:
    froglight_mutf8 => MString::from_utf8,
    cesu8 => cesu8::to_java_cesu8,
    simd_cesu8 => simd_cesu8::mutf8::encode
}

// -------------------------------------------------------------------------------------------------

fn decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode");
    group.throughput(criterion::Throughput::Elements(1));

    let input: Vec<_> = generate::<false>();
    let input: Vec<_> = input.into_iter().map(MString::from_utf8_owned).collect();

    group.bench_with_input("froglight_mutf8", &input, |b, input| {
        let mut iter = input.iter().cycle();
        dispatch!(Level::new(), simd => {
            b.iter(|| unsafe {
                black_box(MStr::to_utf8_simd(simd, black_box(iter.next().unwrap_unchecked().as_ref())));
            });
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

    group.bench_with_input("froglight_mutf8", &input, |b, input| {
        let mut iter = input.iter().cycle();
        dispatch!(Level::new(), simd => {
            b.iter(|| unsafe {
                black_box(MStr::to_utf8_simd(simd, black_box(iter.next().unwrap_unchecked().as_ref())));
            });
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

// -------------------------------------------------------------------------------------------------

bench! {
    valid_mutf8 = true:
    @dispatch froglight_simd => froglight_mutf8::operations::contains::contains_null_or_4_byte_header,
    froglight_const => froglight_mutf8::operations::contains::const_contains_null_or_4_byte_header,
    cesu8 => cesu8_is_valid,
    simd_cesu8 => simd_cesu8::implementation::active::contains_null_or_utf8_4_byte_char_header
}

fn cesu8_is_valid(bytes: &[u8]) -> bool {
    simdutf8::basic::from_utf8(bytes).is_ok_and(cesu8::is_valid_java_cesu8)
}
