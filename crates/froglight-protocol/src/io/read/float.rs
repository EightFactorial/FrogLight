use super::{FrogRead, ReadError};

macro_rules! impl_float_read {
    ($ty1:ty, $ty2:ty) => {
        impl FrogRead for $ty1 {
            #[inline]
            fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError> {
                Ok(Self::from_bits(<$ty2>::fg_read(buf)?))
            }
        }
    };
}

impl_float_read!(f32, u32);
impl_float_read!(f64, u64);

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(512))]

    #[test]
    fn proto_read_f32(data in -32_000_000.0f32..32_000_000.0f32) {
        let bytes = data.to_be_bytes();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());
        match (data, f32::fg_read(&mut cursor)) {
            (expected, Ok(read)) => assert!((expected - read).abs() < f32::EPSILON),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }

    #[test]
    fn proto_read_f64(data in -64_000_000.0f64..64_000_000.0f64) {
        let bytes = data.to_be_bytes();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());
        match (data, f64::fg_read(&mut cursor)) {
            (expected, Ok(read)) => assert!((expected - read).abs() < f64::EPSILON),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }
}
