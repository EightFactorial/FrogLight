#![allow(unused_variables, reason = "Prefer `portable_simd` over `fearless_simd`")]

use alloc::{string::String, vec::Vec};

use fearless_simd::prelude::*;

use crate::types::{MStr, MString};

macro_rules! debug_panic {
    () => {{
        #[cfg(debug_assertions)]
        unreachable!("Invalid (M)UTF-8?!");

        // SAFETY: This should never be reachable for a valid UTF-8 string
        #[cfg(not(debug_assertions))]
        unsafe {
            core::hint::unreachable_unchecked()
        }
    }};
}

/// Convert a UTF-8 string to MUTF-8.
#[must_use]
pub fn utf8_to_mutf8<S: Simd>(simd: S, str: &str) -> MString {
    let cap = str.len().saturating_mul(3).saturating_div(2).min(isize::MAX as usize);
    let mut output = Vec::<u8>::with_capacity(cap);

    let mut iter = str.as_bytes().iter();
    while let Some(a) = iter.next() {
        match a {
            // U+0000 is encoded as [0xC0, 0x80] in MUTF-8.
            0x00 => {
                output.push(0xC0);
                output.push(0x80);
            }
            // U+0001 to U+007F are 1-byte UTF-8 sequences.
            0x00..=0x7F => {
                output.push(*a);
            }
            // U+0080 to U+07FF are 2-byte UTF-8 sequences.
            0x80..=0xDF => {
                let Some(b) = iter.next() else { debug_panic!() };

                output.push(*a);
                output.push(*b);
            }
            // U+0800 to U+FFFF are 3-byte UTF-8 sequences.
            0xE0..=0xEF => {
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

                output.extend_from_slice(&encode_surrogate_pair::<S>(simd, [*a, *b, *c, *d]));
            }
        }
    }

    // SAFETY: The output is valid MUTF-8
    unsafe { MString::from_mutf8_unchecked(output) }
}

/// Convert a UTF-8 string to MUTF-8.
#[must_use]
pub fn mutf8_to_utf8<S: Simd>(simd: S, str: &MStr) -> String {
    let cap = str.len().min(isize::MAX as usize);
    let mut output = Vec::<u8>::with_capacity(cap);

    let mut iter = str.as_bytes().iter();
    while let Some(a) = iter.next() {
        match a {
            0x01..=0x7F => {
                output.push(*a);
            }
            0xC0 => {
                let Some(0x80) = iter.next() else { debug_panic!() };

                output.push(0x00);
            }
            0xC2..=0xDF => {
                let Some(b) = iter.next() else { debug_panic!() };

                #[cfg(debug_assertions)]
                if b & 0b1100_0000 != 0b1000_0000 {
                    debug_panic!();
                }

                output.push(*a);
                output.push(*b);
            }
            0xE0..=0xEF => {
                let Some(b) = iter.next() else { debug_panic!() };

                #[cfg(debug_assertions)]
                if b & 0b1100_0000 != 0b1000_0000 {
                    debug_panic!();
                }

                match (a, b) {
                    (0xe0, 0xa0..=0xbf)
                    | (0xe1..=0xec | 0xee..=0xef, 0x80..=0xbf)
                    | (0xed, 0x80..=0x9f) => {
                        let Some(c) = iter.next() else { debug_panic!() };

                        #[cfg(debug_assertions)]
                        if c & 0b1100_0000 != 0b1000_0000 {
                            debug_panic!();
                        }

                        output.push(*a);
                        output.push(*b);
                        output.push(*c);
                    }
                    (0xed, 0xa0..=0xaf) => {
                        let Some(c) = iter.next() else { debug_panic!() };
                        #[allow(unused_variables, reason = "Used in debug mode")]
                        let Some(d) = iter.next() else { debug_panic!() };
                        let Some(e) = iter.next() else { debug_panic!() };
                        let Some(f) = iter.next() else { debug_panic!() };

                        #[cfg(debug_assertions)]
                        {
                            let value = u32::from_be_bytes([*c, *d, *e, *f]);
                            let mask = 0b1100_0000_1111_1111_1111_0000_1100_0000;
                            let desired = 0b1000_0000_1110_1101_1011_0000_1000_0000;

                            if value & mask != desired {
                                debug_panic!();
                            }
                        }

                        output
                            .extend_from_slice(&decode_surrogate_pair::<S>(simd, [*b, *c, *e, *f]));
                    }
                    _ => debug_panic!(),
                }
            }
            _ => debug_panic!(),
        }
    }

    unsafe { String::from_utf8_unchecked(output) }
}

// -------------------------------------------------------------------------------------------------

#[inline(always)]
fn encode_surrogate_pair<S: Simd>(simd: S, abcd: [u8; 4]) -> [u8; 6] {
    cfg_select! {
        feature = "nightly" => portable_encode_surrogate_pair(abcd),
        _ => fallback_encode_surrogate_pair(simd, abcd),
    }
}

#[inline(always)]
#[cfg(feature = "nightly")]
fn portable_encode_surrogate_pair(abcd: [u8; 4]) -> [u8; 6] {
    use core::simd::prelude::*;

    const CODEPOINT_AND: Simd<u8, 4> = Simd::from_array([0x07, 0x3F, 0x3F, 0x3F]);
    const CODEPOINT_SHIFT: Simd<u32, 4> = Simd::from_array([18, 12, 6, 0]);

    const SURROGATE_AND: Simd<u32, 2> = Simd::from_array([0xFFFF_FFFF, 0x0000_03FF]);
    const SURROGATE_SHIFT: Simd<u32, 2> = Simd::from_array([10, 0]);
    const SURROGATE_OR: Simd<u32, 2> = Simd::from_array([0xD800, 0xDC00]);

    const PAIR_AND: Simd<u16, 6> =
        Simd::from_array([0xF000, 0x0FC0, 0x003F, 0xF000, 0x0FC0, 0x003F]);
    const PAIR_SHIFT: Simd<u16, 6> = Simd::from_array([12, 6, 0, 12, 6, 0]);
    const PAIR_OR: Simd<u16, 6> = Simd::from_array([0xE0, 0x80, 0x80, 0xE0, 0x80, 0x80]);

    let codepoint = Simd::from_array(abcd);
    let codepoint = (codepoint & CODEPOINT_AND).cast::<u32>() << CODEPOINT_SHIFT;

    let surrogate = Simd::splat(codepoint.reduce_or() - 0x0001_0000);
    let surrogate = ((surrogate & SURROGATE_AND) >> SURROGATE_SHIFT) | SURROGATE_OR;

    let pairs = simd_swizzle!(surrogate.cast::<u16>(), [0, 0, 0, 1, 1, 1]);
    let pairs = ((pairs & PAIR_AND) >> PAIR_SHIFT) | PAIR_OR;

    pairs.cast::<u8>().to_array()
}

#[inline(always)]
#[allow(dead_code, reason = "Prefer `portable_encode_surrogate_pair`")]
fn fallback_encode_surrogate_pair<S: Simd>(_: S, [a, b, c, d]: [u8; 4]) -> [u8; 6] {
    let codepoint = (u32::from(a & 0x07) << 18)
        | (u32::from(b & 0x3F) << 12)
        | (u32::from(c & 0x3F) << 6)
        | u32::from(d & 0x3F);

    let codepoint = codepoint - 0x0001_0000;
    let high = (codepoint >> 10) | 0xD800;
    let low = (codepoint & 0x03FF) | 0xDC00;

    [
        0xE0 | ((high & 0xF000) >> 12) as u8,
        0x80 | ((high & 0x0FC0) >> 6) as u8,
        0x80 | ((high & 0x003F) as u8),
        0xE0 | ((low & 0xF000) >> 12) as u8,
        0x80 | ((low & 0x0FC0) >> 6) as u8,
        0x80 | ((low & 0x003F) as u8),
    ]
}

// -------------------------------------------------------------------------------------------------

#[inline(always)]
fn decode_surrogate_pair<S: Simd>(simd: S, bcef: [u8; 4]) -> [u8; 4] {
    cfg_select! {
        feature = "nightly" => portable_decode_surrogate_pair(bcef),
        _ => fallback_decode_surrogate_pair(simd, bcef),
    }
}

#[inline(always)]
#[cfg(feature = "nightly")]
fn portable_decode_surrogate_pair(bcef: [u8; 4]) -> [u8; 4] {
    use core::simd::prelude::*;

    const HIGHLOW_AND: Simd<u8, 4> = Simd::splat(0x003F);
    const HIGHLOW_SHIFT: Simd<u32, 4> = Simd::from_array([6, 0, 6, 0]);
    const HIGHLOW_OR: Simd<u32, 4> = Simd::splat(0xD000);

    const CODEPOINT_AND: Simd<u32, 4> =
        Simd::from_array([0x001C_0000, 0x0003_F000, 0x0000_0FC0, 0x0000_003F]);
    const CODEPOINT_SHIFT: Simd<u32, 4> = Simd::from_array([18, 12, 6, 0]);
    const CODEPOINT_OR: Simd<u32, 4> = Simd::from_array([0xF0, 0x80, 0x80, 0x80]);

    let high_low = (Simd::from_array(bcef) & HIGHLOW_AND).cast::<u32>();
    let high_low = (high_low << HIGHLOW_SHIFT) | HIGHLOW_OR;

    let (high, low) = high_low.interleave(Simd::splat(0));
    let high_low = 0x0001_0000 + (((high.reduce_or() - 0xD800) << 10) | (low.reduce_or() - 0xDC00));

    let codepoint = Simd::splat(high_low) & CODEPOINT_AND;
    let codepoint = ((codepoint) >> CODEPOINT_SHIFT) | CODEPOINT_OR;

    codepoint.cast::<u8>().to_array()
}

#[inline(always)]
#[allow(dead_code, reason = "Prefer `portable_decode_surrogate_pair`")]
fn fallback_decode_surrogate_pair<S: Simd>(_: S, [b, c, e, f]: [u8; 4]) -> [u8; 4] {
    let high = 0xD000 | u32::from(b & 0x3F) << 6 | u32::from(c & 0x3F);
    let low = 0xD000 | u32::from(e & 0x3F) << 6 | u32::from(f & 0x3F);
    let codepoint = 0x0001_0000 + ((high - 0xD800) << 10 | (low - 0xDC00));

    [
        0xF0 | ((codepoint & 0x001C_0000) >> 18) as u8,
        0x80 | ((codepoint & 0x0003_F000) >> 12) as u8,
        0x80 | ((codepoint & 0x0000_0FC0) >> 6) as u8,
        0x80 | ((codepoint & 0x0000_003F) as u8),
    ]
}
