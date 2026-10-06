use fearless_simd::prelude::*;

/// Returns `true` if the given slice contains any null bytes or 4-byte
/// UTF-8 headers.
#[must_use]
#[inline(always)]
#[fearless_simd_macros::simd]
pub fn contains_null_or_4_byte_header<S: Simd>(simd: S, mut bytes: &[u8]) -> bool {
    macro_rules! find_simd {
        (@fearless $size:literal: $simd_ty:ty, $splat:path) => {
            if bytes.len() > $size {
                let zero = $splat(simd, 0b0000_0000);
                let mask = $splat(simd, 0b1111_1000);
                let header = $splat(simd, 0b1111_0000);

                let (chunks, remainder) = bytes.as_chunks::<$size>();
                bytes = remainder;

                chunks.iter().any(|slice| {
                    let a = <$simd_ty>::load_array_ref(simd, slice);
                    a.simd_eq(zero).any_true() || (a & mask).simd_eq(header).any_true()
                })
            } else {
                false
            }
        };
    }

    find_simd!(@fearless 64: fearless_simd::u8x64<S>, S::splat_u8x64)
        || find_simd!(@fearless 32: fearless_simd::u8x32<S>, S::splat_u8x32)
        || find_simd!(@fearless 16: fearless_simd::u8x16<S>, S::splat_u8x16)
        || bytes.iter().any(|b| *b == 0b0000_0000 || (*b & 0b1111_1000) == 0b1111_0000)
}

/// Returns `true` if the given slice contains any 4-byte UTF-8 headers.
#[must_use]
#[inline(always)]
#[fearless_simd_macros::simd]
pub fn contains_4_byte_header<S: Simd>(simd: S, mut bytes: &[u8]) -> bool {
    macro_rules! find_simd {
        (@fearless $size:literal: $simd_ty:ty, $splat:path) => {
            if bytes.len() > $size {
                let mask = $splat(simd, 0b1111_1000);
                let header = $splat(simd, 0b1111_0000);

                let (chunks, remainder) = bytes.as_chunks::<$size>();
                bytes = remainder;

                chunks.iter().any(|slice| {
                    let a = <$simd_ty>::load_array_ref(simd, slice);
                    (a & mask).simd_eq(header).any_true()
                })
            } else {
                false
            }
        };
    }

    find_simd!(@fearless 64: fearless_simd::u8x64<S>, S::splat_u8x64)
        || find_simd!(@fearless 32: fearless_simd::u8x32<S>, S::splat_u8x32)
        || find_simd!(@fearless 16: fearless_simd::u8x16<S>, S::splat_u8x16)
        || bytes.iter().any(|b| (*b & 0b1111_1000) == 0b1111_0000)
}

// -------------------------------------------------------------------------------------------------

/// Returns `true` if the given slice contains any null bytes or 4-byte
/// UTF-8 headers.
#[must_use]
#[inline(always)]
pub const fn const_contains_null_or_4_byte_header(bytes: &[u8]) -> bool {
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == 0b0000_0000 || (b & 0b1111_1000) == 0b1111_0000 {
            return true;
        }
        i += 1;
    }
    false
}

/// Returns `true` if the given slice contains any 4-byte UTF-8 headers.
#[must_use]
#[inline(always)]
pub const fn const_contains_4_byte_header(bytes: &[u8]) -> bool {
    let mut i = 0;
    while i < bytes.len() {
        if (bytes[i] & 0b1111_1000) == 0b1111_0000 {
            return true;
        }
        i += 1;
    }
    false
}
