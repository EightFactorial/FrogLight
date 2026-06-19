//! TODO
#![allow(clippy::wildcard_imports, reason = "`x86_64` module")]

use core::{arch::x86_64::*, simd::prelude::*};

use froglight_facet_iter::{Reader, ReaderError, Writer, WriterError};

use crate::simd::varint::traits::VarIntType;

macro_rules! create_fns {
    (@$method:tt $($fn:ident & $fn_into:ident : $ty:ty => $len:literal),*) => {
        $(
            create_fns!(@$method single $fn & $fn_into : $ty => $len);
        )*
    };
    (@encode single $fn:ident & $fn_into:ident : $ty:ty => $len:literal) => {
        #[must_use]
        #[doc = concat!("Encode a [`", stringify!($ty), "`] using LEB128.")]
        pub fn $fn(value: $ty) -> ([u8; $len], u8) {
            let (enc, len) = encode_inline(value);

            // SAFETY: `len` is guaranteed to be <= $len, and is always in-bounds.
            let enc = unsafe { enc.get_unchecked(0..$len).try_into().unwrap_unchecked() };

            (enc, len)
        }

        #[doc = concat!("Encode a [`", stringify!($ty), "`] using LEB128 into the provided writer.")]
        #[doc = concat!("\n# Errors\n\nReturns an error if the [`Writer`] cannot be written to.\n")]
        #[doc = concat!("\n# Panics\n\nPanics if the buffer is not large enough to hold the encoded value.\n\nThis will never happen if the buffer is at least ", stringify!($len), " bytes long.")]
        pub fn $fn_into(value: $ty, writer: &mut Writer<'_>) -> Result<(), WriterError> {
            let (enc, len) = encode_inline(value);
            let slice = unsafe { enc.get_unchecked(0..len as usize) };
            writer.write_bytes(slice)
        }
    };
    (@decode single $fn:ident & $fn_from:ident : $ty:ty => $len:literal) => {
        #[must_use]
        #[doc = concat!("Decode a [`", stringify!($ty), "`] from a byte slice using LEB128, returning the decoded value and the number of bytes read.")]
        pub fn $fn(slice: &[u8]) -> ($ty, u8) {
            decode_inline(<$ty>::slice_to_array(slice))
        }

        #[doc = concat!("Decode a [`", stringify!($ty), "`] using LEB128 from the provided reader.")]
        #[doc = concat!("\n# Errors\n\nReturns an error if the [`Reader`] cannot be read from.\n")]
        pub fn $fn_from(reader: &mut Reader<'_>) -> Result<$ty, ReaderError> {
            let (dec, len) = decode_inline(<$ty>::slice_to_array(reader.remaining()));
            reader.consume(len as usize)?;
            Ok(dec)
        }
    };
}

create_fns!(
    @encode
    encode_u8 & encode_u8_into: u8 => 2,
    encode_u16 & encode_u16_into: u16 => 3,
    encode_u32 & encode_u32_into: u32 => 5,
    encode_u64 & encode_u64_into: u64 => 10,
    encode_u128 & encode_u128_into: u128 => 19
);

create_fns!(
    @decode
    decode_u8 & decode_u8_from: u8 => 2,
    decode_u16 & decode_u16_from: u16 => 3,
    decode_u32 & decode_u32_from: u32 => 5,
    decode_u64 & decode_u64_from: u64 => 10,
    decode_u128 & decode_u128_from: u128 => 19
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
fn encode_inline<T: VarIntType>(value: T) -> ([u8; 31], u8) {
    match T::MAX_BYTES {
        0..=5 => unsafe { encode_small(value) },
        6..=19 => unsafe { encode_large(value) },
        _ => panic!("Encoding unsupported for types larger than 19 bytes!"),
    }
}

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

// -------------------------------------------------------------------------------------------------

/// Decode integers using SIMD.
#[must_use]
pub fn decode<T: VarIntType>(slice: &[u8]) -> (T, u8) {
    decode_inline::<T>(T::slice_to_array(slice))
}

/// Decode integers using SIMD.
#[must_use]
#[inline(always)]
#[allow(clippy::missing_panics_doc, reason = "Cannot panic")]
fn decode_inline<T: VarIntType>(bytes: T::Encoded) -> (T, u8) {
    match T::MAX_BYTES {
        0..=5 => unsafe { decode_small(bytes) },
        6..=19 => unsafe { decode_large(bytes) },
        _ => panic!("Encoding unsupported for types larger than 19 bytes!"),
    }
}

/// Decode [`u8`]s, [`u16`]s, and [`u32`]s using SIMD.
///
/// # Safety
///
/// TODO
#[must_use]
#[inline(always)]
unsafe fn decode_small<T: VarIntType>(bytes: T::Encoded) -> (T, u8) {
    super::fallback::decode_inline::<T>(bytes)
}

/// Decode [`u64`]s and [`u128`]s using SIMD.
///
/// # Safety
///
/// TODO
#[must_use]
#[inline(always)]
#[allow(clippy::cast_possible_truncation, reason = "Avoids truncation")]
unsafe fn decode_large<T: VarIntType>(bytes: T::Encoded) -> (T, u8) {
    super::fallback::decode_inline::<T>(bytes)
}
