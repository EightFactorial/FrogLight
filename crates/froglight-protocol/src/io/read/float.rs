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
    fn proto_read_f32(data in proptest::collection::vec(-32_000_000.0f32..32_000_000.0f32, 0..512)) {
        // Convert the data to bytes
        let data = data.iter().flat_map(|f| f.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(data.as_slice());

        for (expected, result) in data.chunks_exact(4).map(|chunk|
            (f32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]), f32::fg_read(&mut cursor))
        ) {
            match result {
                Ok(read) => assert!((expected - read).abs() < f32::EPSILON),
                Err(err) => panic!("Expected: `{expected}`, Error: `{err}`"),
            }
        }
    }

    #[test]
    fn proto_read_f64(data in proptest::collection::vec(-64_000_000.0f64..64_000_000.0f64, 0..512)) {
        // Convert the data to bytes
        let data = data.iter().flat_map(|f| f.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(data.as_slice());

        for (expected, result) in data.chunks_exact(8).map(|chunk|
            (f64::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7]]), f64::fg_read(&mut cursor))
        ) {
            match result {
                Ok(read) => assert!((expected - read).abs() < f64::EPSILON),
                Err(err) => panic!("Expected: `{expected}`, Error: `{err}`"),
            }
        }
    }
}
