//! TODO

use core::hint::black_box;

use criterion::Criterion;
use froglight_facet::simd::varint::{self, VarIntType};
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

/// Generate a set of encoded numbers using a fixed seed.
fn generate_encoded<T: VarIntType>() -> Vec<T::Encoded>
where
    StandardUniform: Distribution<T>,
{
    generate::<T>().into_iter().map(|v| v.encode().0).collect()
}

macro_rules! bench {
    ($group:ident = $ty:ty : $(@$bench:ident $name:ident => $fn:path),* ) => {
        fn $group(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($group));
            group.throughput(criterion::Throughput::ElementsAndBytes{ elements: 1, bytes: core::mem::size_of::<$ty>() as u64 });

            let numbers = generate::<$ty>();
            let encoded = generate_encoded::<$ty>();

            $(
                bench!(
                    @$bench
                    group, &numbers, &encoded, $name => $fn
                );
            )*
        }
    };
    ( @encode $group:expr, $input:expr, $ignored:expr, $name:ident => $fn:path ) => {
        $group.bench_with_input(stringify!($name), $input, |b, input| {
            let mut iter = input.iter().copied().cycle();
            b.iter(|| unsafe { $fn(black_box(iter.next().unwrap_unchecked())) });
        });
    };
    ( @decode $group:expr, $ignored:expr, $input:expr, $name:ident => $fn:path ) => {
        $group.bench_with_input(stringify!($name), $input, |b, input| {
            let mut iter = input.iter().copied().cycle();
            b.iter(|| unsafe { $fn(black_box(&iter.next().unwrap_unchecked())) });
        });
    };
}

bench!(
    u8 = u8:
    @encode arch_encode => varint::encode::<u8>,
    @decode arch_decode => varint::decode::<u8>,
    @encode fallback_encode => varint::fallback::encode::<u8>,
    @decode fallback_decode => varint::fallback::decode::<u8>,
    @encode naive_encode => naive_encode_u8,
    @decode naive_decode => naive_decode_u8
);

bench!(
    u16 = u16:
    @encode arch_encode => varint::encode::<u16>,
    @decode arch_decode => varint::decode::<u16>,
    @encode fallback_encode => varint::fallback::encode::<u16>,
    @decode fallback_decode => varint::fallback::decode::<u16>,
    @encode naive_encode => naive_encode_u16,
    @decode naive_decode => naive_decode_u16
);

bench!(
    u32 = u32:
    @encode arch_encode => varint::encode::<u32>,
    @decode arch_decode => varint::decode::<u32>,
    @encode fallback_encode => varint::fallback::encode::<u32>,
    @decode fallback_decode => varint::fallback::decode::<u32>,
    @encode naive_encode => naive_encode_u32,
    @decode naive_decode => naive_decode_u32
);

bench!(
    u64 = u64:
    @encode arch_encode => varint::encode::<u64>,
    @decode arch_decode => varint::decode::<u64>,
    @encode fallback_encode => varint::fallback::encode::<u64>,
    @decode fallback_decode => varint::fallback::decode::<u64>,
    @encode naive_encode => naive_encode_u64,
    @decode naive_decode => naive_decode_u64
);

bench!(
    u128 = u128:
    @encode arch_encode => varint::encode::<u128>,
    @decode arch_decode => varint::decode::<u128>,
    @encode fallback_encode => varint::fallback::encode::<u128>,
    @decode fallback_decode => varint::fallback::decode::<u128>,
    @encode naive_encode => naive_encode_u128,
    @decode naive_decode => naive_decode_u128
);

// -------------------------------------------------------------------------------------------------

macro_rules! naive {
    (@encode $($name:ident : $ty:ty => $size:expr),*) => {
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
    (@decode $($name:ident : $ty:ty => $size:expr),*) => {
        $(
            #[must_use]
            #[inline(never)]
            #[allow(trivial_numeric_casts, reason = "Ignored")]
            #[allow(clippy::cast_possible_truncation, reason = "Ignored")]
            fn $name(bytes: &[u8]) -> ($ty, u8) {
                let mut byte: u8;
                let mut index: usize = 0;
                let mut number: $ty = 0;

                while index < $size {
                    byte = *bytes.get(index).unwrap_or(&0);
                    number |= <$ty>::from(byte & 0b0111_1111) << (7 * index);
                    index += 1;
                    if byte & 0b1000_0000 == 0 {
                        break;
                    }
                }

                (number, index as u8)
            }
        )*
    };
}

naive!(
    @encode
    naive_encode_u8: u8 => 2,
    naive_encode_u16: u16 => 3,
    naive_encode_u32: u32 => 5,
    naive_encode_u64: u64 => 10,
    naive_encode_u128: u128 => 19
);

naive!(
    @decode
    naive_decode_u8: u8 => 2,
    naive_decode_u16: u16 => 3,
    naive_decode_u32: u32 => 5,
    naive_decode_u64: u64 => 10,
    naive_decode_u128: u128 => 19
);
