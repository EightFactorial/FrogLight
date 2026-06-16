//! [`VarInt`] trait implementations
#![expect(clippy::cast_lossless, reason = "Intended behavior")]
#![expect(clippy::cast_possible_truncation, reason = "Intended behavior")]
#![expect(trivial_numeric_casts, reason = "Intended behavior")]

/// A trait for variable-length types.
pub trait VarIntType: sealed::Sealed + Sized {
    /// The maximum number of bytes required to encode this type.
    const MAX_BYTES: usize;

    /// The output type, which is an array with a length of [`Self::MAX_BYTES`]
    type Encoded: AsRef<[u8]>;

    /// Create a [`Self::Encoded`] array from a byte slice.
    fn slice_to_array(bytes: &[u8]) -> Self::Encoded;
    /// Create a [`u64`] from a [`Self::Encoded`] array.
    fn array_to_u64(array: Self::Encoded) -> u64;
    /// Create a [`[u64; 3]`] from a [`Self::Encoded`] array.
    fn array_to_3u64(array: Self::Encoded) -> [u64; 3];

    /// Encode the value into a byte array and return the number of bytes used.
    fn encode(self) -> (Self::Encoded, u8);
    /// Decode the value from a byte array and return the number of bytes read.
    fn decode(bytes: &[u8]) -> (Self, u8);

    /// Create this type from a [`u8`].
    fn from_u8(value: u8) -> Self;
    /// Create this type from a [`u32`].
    fn from_u32(value: u32) -> Self;
    /// Create this type from a [`u64`].
    fn from_u64(value: u64) -> Self;
    /// Create this type from a [`u128`].
    fn from_u128(value: u128) -> Self;

    /// Convert this type to a [`u8`].
    fn to_u8(self) -> u8;
    /// Convert this type to a [`u32`].
    fn to_u32(self) -> u32;
    /// Convert this type to a [`u64`].
    fn to_u64(self) -> u64;
    /// Convert this type to a [`u128`].
    fn to_u128(self) -> u128;
}

macro_rules! implement {
    ($ty:ty, $N:expr, $encode:path, $decode:path) => {
        impl VarIntType for $ty {
            const MAX_BYTES: usize = $N;

            type Encoded = [u8; $N];

            #[inline(always)]
            fn slice_to_array(bytes: &[u8]) -> Self::Encoded {
                core::array::from_fn(|i| bytes.get(i).copied().unwrap_or(0))
            }
            #[inline(always)]
            fn array_to_u64(array: Self::Encoded) -> u64 {
                u64::from_le_bytes(core::array::from_fn(|i| array.get(i).copied().unwrap_or(0)))
            }
            #[inline(always)]
            fn array_to_3u64(array: Self::Encoded) -> [u64; 3] {
                let arr: [u8; 24] = core::array::from_fn(|i| array.get(i).copied().unwrap_or(0));
                [
                    u64::from_le_bytes(unsafe { arr.get_unchecked(0..8).try_into().unwrap_unchecked() }),
                    u64::from_le_bytes(unsafe { arr.get_unchecked(8..16).try_into().unwrap_unchecked() }),
                    u64::from_le_bytes(unsafe { arr.get_unchecked(16..24).try_into().unwrap_unchecked() }),
                ]
            }

            #[inline(always)]
            fn encode(self) -> (Self::Encoded, u8) { $encode(self) }
            #[inline(always)]
            fn decode(bytes: &[u8]) -> (Self, u8) { $decode(bytes) }

            #[inline(always)]
            fn from_u8(value: u8) -> Self { value as Self }

            #[inline(always)]
            fn from_u32(value: u32) -> Self { value as Self }

            #[inline(always)]
            fn from_u64(value: u64) -> Self { value as Self }

            #[inline(always)]
            fn from_u128(value: u128) -> Self { value as Self }

            #[inline(always)]
            fn to_u8(self) -> u8 { self as u8 }

            #[inline(always)]
            fn to_u32(self) -> u32 { self as u32 }

            #[inline(always)]
            fn to_u64(self) -> u64 { self as u64 }

            #[inline(always)]
            fn to_u128(self) -> u128 { self as u128 }
        }
    };
    ($($ty:ty, $N:expr, $encode:path, $decode:path),*) => {
        $( implement!($ty, $N, $encode, $decode); )*
    };
}

implement!(u8, 2, super::encode_u8, super::decode_u8);
implement!(u16, 3, super::encode_u16, super::decode_u16);
implement!(u32, 5, super::encode_u32, super::decode_u32);
implement!(u64, 10, super::encode_u64, super::decode_u64);
implement!(u128, 19, super::encode_u128, super::decode_u128);

// -------------------------------------------------------------------------------------------------

mod sealed {
    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for u128 {}
}
