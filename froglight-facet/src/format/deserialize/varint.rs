//! TODO

macro_rules! create_decode {
    ($($fn:ident & $fn_from:ident : $ty:ty => $len:expr),*) => {
        $(
            cfg_select! {
                feature = "simd" => {
                    pub use crate::simd::varint::{$fn, $fn_from};
                }
                _ => {
                    #[must_use]
                    #[allow(trivial_numeric_casts, reason = "Ignored")]
                    #[allow(clippy::cast_possible_truncation, reason = "Ignored")]
                    #[doc = concat!("Decode a [`", stringify!($ty), "`] from a byte slice using LEB128, returning the decoded value and the number of bytes read.")]
                    pub fn $fn(slice: &[u8]) -> ($ty, u8) {
                        let mut byte: u8;
                        let mut index: usize = 0;
                        let mut number: $ty = 0;

                        while index < $len {
                            byte = slice.get(index).copied().unwrap_or(0);
                            number |= <$ty>::from(byte & 0b0111_1111) << (7 * index);
                            index += 1;
                            if byte & 0b1000_0000 == 0 {
                                break;
                            }
                        }

                        (number, index as u8)
                    }

                    #[doc = concat!("Decode a [`", stringify!($ty), "`] using LEB128 from the provided reader.")]
                    #[doc = concat!("\n# Errors\n\nReturns an error if the [`Reader`] cannot be read from.\n")]
                    pub fn $fn_from(reader: &mut crate::format::Reader<'_>) -> Result<$ty, crate::format::ReaderError> {
                        let (dec, len) = $fn(reader.remaining());
                        reader.consume(len as usize)?;
                        Ok(dec)
                    }
                }
            }
        )*
    };
}

create_decode!(
    decode_u8 & decode_u8_from: u8 => 2,
    decode_u16 & decode_u16_from: u16 => 3,
    decode_u32 & decode_u32_from: u32 => 5,
    decode_u64 & decode_u64_from: u64 => 10,
    decode_u128 & decode_u128_from: u128 => 19
);
