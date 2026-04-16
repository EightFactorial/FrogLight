//! A fallback implementation of variable-length encoding that should optimize
//! efficiently for platforms without a specialized implementation.

use core::simd::prelude::*;

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
/// This is the fallback implementation which should support any platform.
#[must_use]
pub fn encode<T: VarIntType>(value: T) -> ([u8; 31], u8) { encode_inline::<T>(value) }

/// Encode integers using SIMD.
///
/// This is the fallback implementation which should support any platform.
#[must_use]
#[inline(always)]
fn encode_inline<T: VarIntType>(value: T) -> ([u8; 31], u8) {
    match T::MAX_BYTES {
        0..=5 => encode_small(value),
        6..=24 => encode_large(value),
        _ => panic!("Encoding unsupported for types larger than 24 bytes!"),
    }
}

/// Set all MSBs expect the last one, and return the number of non-zero bytes.
#[inline(always)]
fn mark_bytes<const N: usize>(input: Simd<u8, N>) -> (Simd<u8, N>, u8) {
    #[inline(always)]
    #[expect(clippy::cast_possible_truncation, reason = "<= N")]
    const fn usize_to_u8(i: usize) -> u8 { i as u8 }

    // Note: Requires `const_array` and `const_trait_impl`
    // to guarantee this is a compile-time constant.
    let ascending: Simd<u8, N> = const { Simd::from_array(core::array::from_fn(usize_to_u8)) };
    let msbs: Simd<u8, N> = const { Simd::splat(0x80) };
    let zero: Simd<u8, N> = const { Simd::splat(0x00) };

    #[expect(clippy::cast_possible_truncation, reason = "<= 64")]
    let bytes = 64u32.saturating_sub(input.simd_ne(zero).to_bitmask().leading_zeros()) as u8;
    let msbmask = ascending.simd_lt(Simd::splat(bytes)).shift_elements_left::<1>(false);

    (input | (msbmask.to_simd().cast::<u8>() & msbs), bytes)
}

// -------------------------------------------------------------------------------------------------

/// Encode [`u8`]s, [`u16`]s, and [`u32`]s using SIMD.
#[must_use]
#[inline(always)]
fn encode_small<T: VarIntType>(value: T) -> ([u8; 31], u8) {
    // Separate the bits into groups of 7 and shift them.
    let v = value.to_u64().to_le();

    let bytes = (v & 0x0000_007f)
        | ((v & 0x0000_3f80) << 1)
        | ((v & 0x001f_c000) << 2)
        | ((v & 0x0fe0_0000) << 3)
        | ((v & 0xf000_0000) << 4);

    // Set all but the last MSBs.
    let (bytes, length) = mark_bytes(Simd::from_array(bytes.to_ne_bytes()));

    (super::arr8_to_31(bytes.to_array()), length.max(1))
}

// -------------------------------------------------------------------------------------------------

/// Encode [`u64`]s and [`u128`]s using SIMD.
///
/// TODO: Optimize this further
#[must_use]
#[inline(always)]
#[expect(clippy::cast_possible_truncation, reason = "Avoids truncation")]
fn encode_large<T: VarIntType>(value: T) -> ([u8; 31], u8) {
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
    bytes[1] = (((v & 0x0000_0000_0000_0000_7f00_0000_0000_0000) >> 56)
        | ((v & 0x0000_0000_0000_003f_8000_0000_0000_0000) >> 55)
        | ((v & 0x0000_0000_0000_1fc0_0000_0000_0000_0000) >> 54)
        | ((v & 0x0000_0000_000f_e000_0000_0000_0000_0000) >> 53)
        | ((v & 0x0000_0000_07f0_0000_0000_0000_0000_0000) >> 52)
        | ((v & 0x0000_0003_f800_0000_0000_0000_0000_0000) >> 51)
        | ((v & 0x0000_01fc_0000_0000_0000_0000_0000_0000) >> 50)
        | ((v & 0x0000_fe00_0000_0000_0000_0000_0000_0000) >> 49)) as u64;
    bytes[2] = (((v & 0x007f_0000_0000_0000_0000_0000_0000_0000) >> 112)
        | ((v & 0x3f80_0000_0000_0000_0000_0000_0000_0000) >> 111)
        | ((v & 0xc000_0000_0000_0000_0000_0000_0000_0000) >> 110)) as u64;

    // Set all but the last MSBs.
    let bytes = Simd::from_array(unsafe { core::mem::transmute::<[u64; 3], [u8; 24]>(bytes) });
    let (bytes, length) = mark_bytes(bytes);

    (super::arr24_to_31(bytes.to_array()), length.max(1))
}
