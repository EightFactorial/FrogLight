//! TODO
#![allow(clippy::wildcard_imports, reason = "`x86_64` module")]

#[allow(unused_imports, reason = "WIP")]
use core::{arch::x86_64::*, simd::prelude::*};

/// Convert a UTF-8 string to MUTF-8.
#[must_use]
#[cfg(feature = "alloc")]
pub fn utf8_to_mutf8(str: &str) -> crate::prelude::MString {
    macro_rules! debug_panic {
        () => {{
            #[cfg(debug_assertions)]
            panic!("Invalid UTF-8, was expecting another byte!");

            // SAFETY: This should never be reachable for a valid UTF-8 string
            #[cfg(not(debug_assertions))]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        }};
    }

    let cap = str.len().saturating_mul(2).min(isize::MAX as usize);
    let mut output = alloc::vec::Vec::<u8>::with_capacity(cap);

    let mut iter = str.as_bytes().iter();
    while let Some(a) = iter.next() {
        match a {
            // U+0000 is encoded as [0xC0, 0x80] in MUTF-8.
            0x00 => {
                output.push(0xC0);
                output.push(0x80);
            }
            // U+0001 to U+007F are 1-byte UTF-8 sequences.
            ..=0x7F => {
                output.push(*a);
            }
            // U+0080 to U+07FF are 2-byte UTF-8 sequences.
            ..=0xDF => {
                let Some(b) = iter.next() else { debug_panic!() };
                output.push(*a);
                output.push(*b);
            }
            // U+0800 to U+FFFF are 3-byte UTF-8 sequences.
            ..=0xEF => {
                let Some(b) = iter.next() else { debug_panic!() };
                let Some(c) = iter.next() else { debug_panic!() };
                output.push(*a);
                output.push(*b);
                output.push(*c);
            }
            // U+10000 to U+10FFFF are 4-byte UTF-8 sequences. (UTF-8 max is U+10FFFF)
            _ => {
                let Some(b) = iter.next() else { debug_panic!() };
                let Some(c) = iter.next() else { debug_panic!() };
                let Some(d) = iter.next() else { debug_panic!() };
                output.extend_from_slice(&encode_4_byte_utf8([*a, *b, *c, *d]));
            }
        }
    }

    // SAFETY: The output is valid MUTF-8
    unsafe { crate::prelude::MString::from_mutf8_unchecked(output) }
}

#[must_use]
#[inline(always)]
#[cfg(feature = "alloc")]
#[allow(clippy::items_after_statements, reason = "Readablility")]
fn encode_4_byte_utf8(array: [u8; 4]) -> [u8; 6] {
    cfg_select! {
        // Use BMI2 instructions if available.
        all(target_arch = "x86_64", target_feature = "bmi2", not(slow_bmi2)) => {
            const CODEPOINT_AND: Simd<u8, 4> = Simd::from_array([0x07, 0x3F, 0x3F, 0x3F]);
            const CODEPOINT_SHIFT: Simd<u32, 4> = Simd::from_array([18, 12, 6, 0]);

            let codepoint = Simd::from_array(array);
            let codepoint = (codepoint & CODEPOINT_AND).cast::<u32>() << CODEPOINT_SHIFT;

            const SURROGATE_AND: Simd<u32, 2> = Simd::from_array([0xFFFF_FFFF, 0x0000_03FF]);
            const SURROGATE_SHIFT: Simd<u32, 2> = Simd::from_array([10, 0]);
            const SURROGATE_OR: Simd<u32, 2> = Simd::from_array([0xD800, 0xDC00]);

            let surrogate = Simd::splat(codepoint.reduce_or() - 0x0001_0000);
            let surrogate = ((surrogate & SURROGATE_AND) >> SURROGATE_SHIFT) | SURROGATE_OR;

            const PAIR_AND: Simd<u16, 6> = Simd::from_array([0xF000, 0x0FC0, 0x003F, 0xF000, 0x0FC0, 0x003F]);
            const PAIR_SHIFT: Simd<u16, 6> = Simd::from_array([12, 6, 0, 12, 6, 0]);
            const PAIR_OR: Simd<u16, 6> = Simd::from_array([0xE0, 0x80, 0x80, 0xE0, 0x80, 0x80]);

            let pairs = simd_swizzle!(surrogate.cast::<u16>(), [0, 0, 0, 1, 1, 1]);
            let pairs = ((pairs & PAIR_AND) >> PAIR_SHIFT) | PAIR_OR;

            pairs.cast::<u8>().to_array()
        }
        // Otherwise use the fallback implementation.
        _ => super::fallback::encode_4_byte_utf8(array)

    }
}

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
    as_simd!(bytes => 32, 16, 8, 4: {
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
    as_simd!(bytes => 32, 16, 8, 4: {
        |simd: Simd<u8, _>| {
            let mask   = Simd::<u8, _>::splat(0b1111_1000);
            let header = Simd::<u8, _>::splat(0b1111_0000);

            (simd & mask).simd_eq(header).any()
        }
    });

    bytes.iter().copied().any(|b| (b & 0b1111_1000) == 0b1111_0000)
}
