//! TODO

use core::simd::prelude::*;

pub use super::fallback::{mutf8_to_utf8, utf8_to_mutf8};

// -------------------------------------------------------------------------------------------------

macro_rules! as_simd {
    ($input:expr => $($n:literal),* : $fn:expr) => {
        $(
            if $input.len() >= $n {
                let (chunks, remainder) = $input.as_chunks::<$n>();
                $input = remainder;

                if chunks.into_iter().copied().any(|arr| ($fn)(Simd::from_array(arr))) {
                    return true;
                }
            }
        )*
    };
}

/// Returns `true` if the given slice contains any null bytes or 4-byte UTF-8
/// headers.
#[must_use]
#[inline(always)]
pub fn contains_null_or_4_byte_header(mut bytes: &[u8]) -> bool {
    as_simd!(bytes => 32, 16, 8: {
        |simd: Simd<u8, _>| {
            let zero   = Simd::<u8, _>::splat(0b0000_0000);
            let mask   = Simd::<u8, _>::splat(0b1111_1000);
            let header = Simd::<u8, _>::splat(0b1111_0000);

            simd.simd_eq(zero).any() || (simd & mask).simd_eq(header).any()
        }
    });

    bytes.iter().copied().any(|b| b == 0b0000_0000 || (b & 0b1111_1000) == 0b1111_0000)
}

/// Returns `true` if the given slice contains any 4-byte UTF-8 headers.
#[must_use]
#[inline(always)]
pub fn contains_4_byte_header(mut bytes: &[u8]) -> bool {
    as_simd!(bytes => 32, 16, 8: {
        |simd: Simd<u8, _>| {
            let mask   = Simd::<u8, _>::splat(0b1111_1000);
            let header = Simd::<u8, _>::splat(0b1111_0000);

            (simd & mask).simd_eq(header).any()
        }
    });

    bytes.iter().copied().any(|b| (b & 0b1111_1000) == 0b1111_0000)
}
