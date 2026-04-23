//! TODO

macro_rules! create_encode {
    ($($fn:ident & $fn_into:ident : $ty:ty => $len:expr),*) => {
        $(
            cfg_select! {
                feature = "simd" => {
                    pub use crate::simd::varint::{$fn, $fn_into};
                }
                _ => {
                    #[allow(trivial_numeric_casts, reason = "Ignored")]
                    #[allow(clippy::cast_possible_truncation, reason = "Ignored")]
                    #[doc = concat!("Encode a [`", stringify!($ty), "`] using LEB128.")]
                    #[must_use]
                    pub fn $fn(mut value: $ty) -> ([u8; $len], u8) {
                        let mut output = [0u8; $len];
                        let mut count = 0;
                        let mut byte;

                        while (value != 0 || count == 0) && count < $len {
                            byte = (value & 0b0111_1111) as u8;
                            value = (value >> 7) & (<$ty>::MAX >> 6);
                            if value != 0 {
                                byte |= 0b1000_0000;
                            }

                            output[count] = byte;
                            count += 1;
                        }

                        (output, count as u8)
                    }

                    #[doc = concat!("Encode a [`", stringify!($ty), "`] using LEB128 into the provided buffer, returning the number of bytes written.")]
                    #[doc = ""]
                    #[doc = concat!("# Panics\n\nPanics if the buffer is not large enough to hold the encoded value.\n\nThis will never happen if the buffer is at least ", stringify!($len), " bytes long.")]
                    #[must_use]
                    pub fn $fn_into(value: $ty, buffer: &mut [u8]) -> usize {
                        let (enc, len) = $fn(value);
                        let len = len as usize;

                        // SAFETY: `len` is guaranteed to be <= $len, and is always in-bounds.
                        let src = unsafe { enc.get_unchecked(0..len) };
                        let dst = buffer.get_mut(0..len).expect(concat!("Buffer is too small to hold the encoded value! Requires at most ", stringify!($len), " bytes."));
                        dst.copy_from_slice(src);

                        len
                    }
                }
            }
        )*
    };
}

create_encode!(
    encode_u8 & encode_u8_into: u8 => 2,
    encode_u16 & encode_u16_into: u16 => 3,
    encode_u32 & encode_u32_into: u32 => 5,
    encode_u64 & encode_u64_into: u64 => 10,
    encode_u128 & encode_u128_into: u128 => 19
);
