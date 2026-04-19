//! TODO
#![allow(clippy::wildcard_imports, reason = "`x86_64` module")]

use core::{arch::x86_64::*, simd::prelude::*};

use crate::simd::varint::traits::VarIntType;

macro_rules! encode_fns {
    ($($fn:ident & $fn_into:ident : $ty:ty => $len:literal),*) => {
        $(
            encode_fns!(@single $fn & $fn_into : $ty => $len);
        )*
    };
    (@single $fn:ident & $fn_into:ident : $ty:ty => $len:literal) => {
        #[doc = concat!("Encode a [`", stringify!($ty), "`] using LEB128.")]
        #[must_use]
        pub fn $fn(value: $ty) -> ([u8; $len], u8) {
            let (enc, len) = encode_inline(value);

            // SAFETY: `len` is guaranteed to be <= $len, and is always in-bounds.
            let enc = unsafe { enc.get_unchecked(0..$len).try_into().unwrap_unchecked() };

            (enc, len)
        }

        #[doc = concat!("Encode a [`", stringify!($ty), "`] using LEB128 into the provided buffer, returning the number of bytes written.")]
        #[doc = ""]
        #[doc = concat!("# Panics\n\nPanics if the buffer is not large enough to hold the encoded value.\n\nThis will never happen if the buffer is at least ", stringify!($len), " bytes long.")]
        #[must_use]
        pub fn $fn_into(value: $ty, buffer: &mut [u8]) -> usize {
            let (enc, len) = encode_inline(value);
            let len = len as usize;

            // SAFETY: `len` is guaranteed to be <= $len, and is always in-bounds.
            let src = unsafe { enc.get_unchecked(0..len) };
            let dst = buffer.get_mut(0..len).expect(concat!("Buffer is too small to hold the encoded value! Requires at most ", stringify!($len), " bytes."));
            dst.copy_from_slice(src);

            len
        }

    };
}

encode_fns!(
    encode_u8 & encode_u8_into: u8 => 2,
    encode_u16 & encode_u16_into: u16 => 3,
    encode_u32 & encode_u32_into: u32 => 5,
    encode_u64 & encode_u64_into: u64 => 10,
    encode_u128 & encode_u128_into: u128 => 19
);

// -------------------------------------------------------------------------------------------------

/// Encode integers using SIMD.
///
/// # Safety
///
/// TODO
#[must_use]
pub fn encode<T: VarIntType>(value: T) -> ([u8; 31], u8) { encode_inline::<T>(value) }

/// Encode integers using SIMD.
///
/// # Safety
///
/// TODO
#[must_use]
#[inline(always)]
#[allow(clippy::missing_panics_doc, reason = "Cannot panic")]
pub fn encode_inline<T: VarIntType>(value: T) -> ([u8; 31], u8) {
    match T::MAX_BYTES {
        0..=5 => unsafe { encode_small(value) },
        6..=19 => unsafe { encode_large(value) },
        _ => panic!("Encoding unsupported for types larger than 19 bytes!"),
    }
}

// -------------------------------------------------------------------------------------------------

/// Encode [`u8`]s, [`u16`]s, and [`u32`]s using SIMD.
///
/// # Safety
///
/// TODO
#[must_use]
#[inline(always)]
unsafe fn encode_small<T: VarIntType>(value: T) -> ([u8; 31], u8) {
    cfg_select! {
        // Use BMI2 instructions if available.
        all(target_arch = "x86_64", target_feature = "bmi2", not(slow_bmi2)) => {
            // Separate the bits into groups of 7 and shift them.
            let v = value.to_u64().to_le();

            // SAFETY: The target has `bmi2` support.
            let bytes = unsafe { _pdep_u64(v, 0x0000_000f_7f7f_7f7f) };

            // Set all but the last MSBs.
            let (bytes, length) = super::fallback::mark_bytes(Simd::from_array(bytes.to_ne_bytes()));

            (super::arr8_to_31(bytes.to_array()), length.max(1))
        }
        // Otherwise use the fallback implementation.
        _ => super::fallback::encode_small(value)
    }
}

// -------------------------------------------------------------------------------------------------

/// Encode [`u64`]s and [`u128`]s using SIMD.
///
/// # Safety
///
/// TODO
#[must_use]
#[inline(always)]
#[expect(clippy::cast_possible_truncation, reason = "Avoids truncation")]
unsafe fn encode_large<T: VarIntType>(value: T) -> ([u8; 31], u8) {
    cfg_select! {
        // Use BMI2 instructions if available.
        all(target_arch = "x86_64", target_feature = "bmi2", not(slow_bmi2)) => {
            // Separate the bits into groups of 7 and shift them.
            let v = value.to_u128().to_le();

            // SAFETY: The target has `bmi2` support.
            let mut bytes = [0u64; 3];
            bytes[0] = unsafe { _pdep_u64(v as u64, 0x7f7f_7f7f_7f7f_7f7f) };
            bytes[1] = unsafe { _pdep_u64((v >> 56) as u64, 0x7f7f_7f7f_7f7f_7f7f) };
            bytes[2] = unsafe { _pdep_u64((v >> 112) as u64, 0x7f7f_7f7f_7f7f_7f7f) };

            // Set all but the last MSBs.
            let bytes = Simd::from_array(unsafe { core::mem::transmute::<[u64; 3], [u8; 24]>(bytes) });
            let (bytes, length) = super::fallback::mark_bytes(bytes);

            (super::arr24_to_31(bytes.to_array()), length.max(1))
        }
        // Otherwise use the fallback implementation.
        _ => super::fallback::encode_large(value)
    }
}
