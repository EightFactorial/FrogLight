//! TODO

use core::hint::black_box;

use criterion::Criterion;
use froglight_facet::simd::varint;
use rand::{distr::StandardUniform, prelude::*, rngs::Xoshiro128PlusPlus};

fn main() {
    println!("SIMD Module: `{}`", varint::ARCH);

    let mut c = Criterion::default().configure_from_args();

    u8(&mut c);
    u16(&mut c);
    u32(&mut c);
    u64(&mut c);
    u128(&mut c);

    c.final_summary();
}

// -------------------------------------------------------------------------------------------------

/// How many random numbers to generate for the benchmarks.
const GENERATE_SIZE: usize = 1_000_000;

/// Generate a set of random numbers using a fixed seed.
fn generate<T>() -> Vec<T>
where
    StandardUniform: Distribution<T>,
{
    let mut rand = Xoshiro128PlusPlus::seed_from_u64(0x0655_E4BA_22F5_A61C);
    let mut input = Vec::with_capacity(GENERATE_SIZE);
    for _ in 0..GENERATE_SIZE {
        input.push(rand.random::<T>());
    }
    input
}

macro_rules! bench {
    ( $group:ident = $ty:ty : $($name:ident => $fn:path),* ) => {
        fn $group(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($group));
            group.throughput(criterion::Throughput::ElementsAndBytes{ elements: 1, bytes: core::mem::size_of::<$ty>() as u64 });

            let input = generate::<$ty>();

            $(
                group.bench_with_input(stringify!($name), &input, |b, input| {
                    let mut iter = input.iter().copied().cycle();
                    b.iter(|| unsafe { $fn(black_box(iter.next().unwrap_unchecked())) });
                });
            )*
        }
    };
}

bench!(
    u8 = u8:
    arch => varint::encode,
    fallback => varint::fallback::encode,
    naive => naive_u8
);

bench!(
    u16 = u16:
    arch => varint::encode,
    fallback => varint::fallback::encode,
    naive => naive_u16
);

bench!(
    u32 = u32:
    arch => varint::encode,
    fallback => varint::fallback::encode,
    naive => naive_u32
);

bench!(
    u64 = u64:
    arch => varint::encode,
    fallback => varint::fallback::encode,
    naive => naive_u64
);

bench!(
    u128 = u128:
    arch => varint::encode,
    fallback => varint::fallback::encode,
    naive => naive_u128
);

// -------------------------------------------------------------------------------------------------

macro_rules! naive {
    ($($name:ident : $ty:ty => $size:expr),*) => {
        $(
            #[must_use]
            #[inline(never)]
            #[allow(trivial_numeric_casts, reason = "Ignored")]
            #[allow(clippy::cast_possible_truncation, reason = "Ignored")]
            fn $name(mut number: $ty) -> ([u8; $size], u8) {
                let mut output = [0u8; $size];
                let mut count = 0;
                let mut byte;

                while (number != 0 || count == 0) && count < $size {
                    byte = (number & 0b0111_1111) as u8;
                    number = (number >> 7) & (<$ty>::MAX >> 6);
                    if number != 0 {
                        byte |= 0b1000_0000;
                    }

                    output[count] = byte;
                    count += 1;
                }

                (output, count as u8)
            }
        )*
    };
}

naive!(
    naive_u8: u8 => 2,
    naive_u16: u16 => 3,
    naive_u32: u32 => 5,
    naive_u64: u64 => 10,
    naive_u128: u128 => 19
);
