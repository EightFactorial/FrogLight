//! TODO
#![allow(clippy::wildcard_imports, reason = "`aarch64` module")]

#[expect(unused_imports, reason = "WIP")]
use core::{arch::aarch64::*, simd::prelude::*};

use crate::{
    format::{Reader, ReaderError, Writer, WriterError},
    simd::varint::traits::VarIntType,
};

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
#[must_use]
pub fn encode<T: VarIntType>(value: T) -> ([u8; 31], u8) { encode_inline::<T>(value) }

/// Encode integers using SIMD.
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
    // Separate the bits into groups of 7 and shift them.
    let v = value.to_u64().to_le();

    // Build the value from each group of 7 bits.
    let mut bytes = v & 0x0000_007f;

    if T::MAX_BYTES >= 2 {
        bytes |= (v & 0x0000_3f80) << 1;
    }
    if T::MAX_BYTES >= 3 {
        bytes |= (v & 0x001f_c000) << 2;
    }
    if T::MAX_BYTES >= 5 {
        bytes |= (v & 0x0fe0_0000) << 3;
        bytes |= (v & 0xf000_0000) << 4;
    }

    // Set all but the last MSBs.
    let (bytes, length) = super::fallback::mark_bytes(Simd::from_array(bytes.to_ne_bytes()));

    (super::arr8_to_31(bytes.to_array()), length.max(1))
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
    // Separate the bits into groups of 7 and shift them.
    let v = value.to_u128().to_le();

    let mut bytes = [0u64; 3];
    bytes[0] = ((v & 0x0000_0000_0000_0000_0000_0000_0000_007f)
        | ((v & 0x0000_0000_0000_0000_0000_0000_0000_3f80) << 1)
        | ((v & 0x0000_0000_0000_0000_0000_0000_001f_c000) << 2)
        | ((v & 0x0000_0000_0000_0000_0000_0000_0fe0_0000) << 3)
        | ((v & 0x0000_0000_0000_0000_0000_0007_f000_0000) << 4)
        | ((v & 0x0000_0000_0000_0000_0000_03f8_0000_0000) << 5)
        | ((v & 0x0000_0000_0000_0000_0001_fc00_0000_0000) << 6)
        | ((v & 0x0000_0000_0000_0000_00fe_0000_0000_0000) << 7)) as u64;
    if T::MAX_BYTES >= 9 {
        bytes[1] = (((v & 0x0000_0000_0000_0000_7f00_0000_0000_0000) >> 56)
            | ((v & 0x0000_0000_0000_003f_8000_0000_0000_0000) >> 55)
            | ((v & 0x0000_0000_0000_1fc0_0000_0000_0000_0000) >> 54)
            | ((v & 0x0000_0000_000f_e000_0000_0000_0000_0000) >> 53)
            | ((v & 0x0000_0000_07f0_0000_0000_0000_0000_0000) >> 52)
            | ((v & 0x0000_0003_f800_0000_0000_0000_0000_0000) >> 51)
            | ((v & 0x0000_01fc_0000_0000_0000_0000_0000_0000) >> 50)
            | ((v & 0x0000_fe00_0000_0000_0000_0000_0000_0000) >> 49)) as u64;
    }
    if T::MAX_BYTES >= 17 {
        bytes[2] = (((v & 0x007f_0000_0000_0000_0000_0000_0000_0000) >> 112)
            | ((v & 0x3f80_0000_0000_0000_0000_0000_0000_0000) >> 111)
            | ((v & 0xc000_0000_0000_0000_0000_0000_0000_0000) >> 110)) as u64;
    }

    // Set all but the last MSBs.
    let bytes = Simd::from_array(unsafe { core::mem::transmute::<[u64; 3], [u8; 24]>(bytes) });
    let (bytes, length) = super::fallback::mark_bytes(bytes);

    (super::arr24_to_31(bytes.to_array()), length.max(1))
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
    let input = T::array_to_u64(bytes);

    // Use the MSBs to determine how many bytes there are.
    let bits = 64u32.saturating_sub((input & 0x0000_0080_8080_8080).leading_zeros());
    #[expect(clippy::cast_possible_truncation, reason = "<= 64")]
    let bytes = ((bits / 8) + 1) as u8;

    // Build the value from each group of 7 bits.
    let mut value = input & 0x0000_0000_0000_007f;

    if T::MAX_BYTES >= 2 && bytes >= 2 {
        value |= (input & 0x0000_0000_0000_7f00) >> 1;
    }
    if T::MAX_BYTES >= 3 && bytes >= 3 {
        value |= (input & 0x0000_0000_007f_0000) >> 2;
    }
    if T::MAX_BYTES >= 5 {
        if bytes >= 4 {
            value |= (input & 0x0000_0000_7f00_0000) >> 3;
        }
        if bytes >= 5 {
            value |= (input & 0x0000_007f_0000_0000) >> 4;
        }
    }

    (T::from_u64(value.to_le()), bytes)
}

/// Decode [`u64`]s and [`u128`]s using SIMD.
///
/// # Safety
///
/// TODO
#[must_use]
#[inline(always)]
#[allow(clippy::cast_lossless, reason = "Readability")]
#[allow(clippy::cast_possible_truncation, reason = "Avoids truncation")]
unsafe fn decode_large<T: VarIntType>(bytes: T::Encoded) -> (T, u8) {
    let array = unsafe { core::mem::transmute::<[u64; 3], [u8; 24]>(T::array_to_3u64(bytes)) };
    let (array, bytes) = super::fallback::unmark_bytes(Simd::from_array(array));

    let [arr_a, arr_b, arr_c] =
        unsafe { core::mem::transmute::<[u64; 24], [[u64; 8]; 3]>(array.cast::<u64>().to_array()) };
    let shift: Simd<u64, 8> = const { Simd::from_array([0, 7, 14, 21, 28, 35, 42, 49]) };

    let arr_a = Simd::from_array(arr_a);
    let value_a = (arr_a << shift).reduce_or();

    let arr_b = Simd::from_array(arr_b);
    let value_b = (arr_b << shift).reduce_or();

    if T::MAX_BYTES < 17 {
        // Build the value from each group of 56 bits.
        let value = (value_a as u128) | ((value_b as u128) << 56);

        (T::from_u128(value.to_le()), bytes.max(1))
    } else {
        // Read an additional 8 bytes if necessary.
        let arr_c = Simd::from_array(arr_c);
        let value_c = (arr_c << shift).reduce_or();

        // Build the value from each group of 56 bits.
        let value = (value_a as u128) | ((value_b as u128) << 56) | ((value_c as u128) << 112);

        (T::from_u128(value.to_le()), bytes.max(1))
    }
}
