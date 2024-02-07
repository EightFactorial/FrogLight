use super::{FrogRead, ReadError};

macro_rules! impl_integer_read {
    ($ty:ty) => {
        impl FrogRead for $ty {
            #[inline]
            fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError> {
                let position = usize::try_from(buf.position()).expect("Cursor position too large");
                let length = usize::try_from(Self::BITS / 8).expect("Integer too large");

                <std::io::Cursor<_> as std::io::BufRead>::consume(buf, length);

                if let Some(slice) = &buf.get_ref().get(position..position + length) {
                    #[allow(clippy::redundant_closure_call)]
                    Ok(<$ty>::from_be(bytemuck::pod_read_unaligned(slice)))
                } else {
                    let leftover = buf.get_ref().len() - position;
                    Err(ReadError::EndOfBuffer(length, leftover))
                }
            }
        }
    };
}

impl_integer_read!(u8);
impl_integer_read!(u16);
impl_integer_read!(u32);
impl_integer_read!(u64);
impl_integer_read!(u128);

impl_integer_read!(i8);
impl_integer_read!(i16);
impl_integer_read!(i32);
impl_integer_read!(i64);
impl_integer_read!(i128);

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(256))]

    #[test]
    fn proto_read_u8(data in proptest::bits::u8::ANY) {
        use bitvec::view::BitViewSized;
        let mut cursor = std::io::Cursor::new(data.as_raw_slice());
        match (data, u8::fg_read(&mut cursor)) {
            (expected, Ok(read)) => assert_eq!(expected, read),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }

    #[test]
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn proto_read_i8(data in proptest::bits::i8::ANY) {
        use bitvec::view::BitViewSized;
        let data = data as u8;
        let mut cursor = std::io::Cursor::new(data.as_raw_slice());
        match (data, i8::fg_read(&mut cursor)) {
            (expected, Ok(read)) => assert_eq!((expected as i8), read),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(1024))]

    #[test]
    fn proto_read_u16(data in proptest::bits::u16::ANY) {
        let bytes = data.to_be_bytes();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());
        match (data, u16::fg_read(&mut cursor)) {
            (expected, Ok(read)) => assert_eq!(expected, read),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }

    #[test]
    fn proto_read_u32(data in proptest::bits::u32::ANY) {
        let bytes = data.to_be_bytes();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());
        match (data, u32::fg_read(&mut cursor)) {
            (expected, Ok(read)) => assert_eq!(expected, read),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }

    #[test]
    fn proto_read_u64(data in proptest::bits::u64::ANY) {
        let bytes = data.to_be_bytes();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());
        match (data, u64::fg_read(&mut cursor)) {
            (expected, Ok(read)) => assert_eq!(expected, read),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }

    #[test]
    fn proto_read_u128(data in (proptest::bits::u64::ANY, proptest::bits::u64::ANY)) {
        let data = u128::from_be_bytes([data.0.to_be_bytes(), data.1.to_be_bytes()].concat().try_into().unwrap());
        let bytes = data.to_be_bytes();

        let mut cursor = std::io::Cursor::new(bytes.as_slice());
        match (data, u128::fg_read(&mut cursor)) {
            (expected, Ok(read)) => assert_eq!(expected, read),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }

    #[test]
    fn proto_read_i16(data in proptest::bits::i16::ANY) {
        let bytes = data.to_be_bytes();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());
        match (data, i16::fg_read(&mut cursor)) {
            (expected, Ok(read)) => assert_eq!(expected, read),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }


    #[test]
    fn proto_read_i32(data in proptest::bits::i32::ANY) {
        let bytes = data.to_be_bytes();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());
        match (data, i32::fg_read(&mut cursor)) {
            (expected, Ok(read)) => assert_eq!(expected, read),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }

    #[test]
    fn proto_read_i64(data in proptest::bits::i64::ANY) {
        let bytes = data.to_be_bytes();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());
        match (data, i64::fg_read(&mut cursor)) {
            (expected, Ok(read)) => assert_eq!(expected, read),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }

    #[test]
    fn proto_read_i128(data in (proptest::bits::i64::ANY, proptest::bits::i64::ANY)) {
        let data = i128::from_be_bytes([data.0.to_be_bytes(), data.1.to_be_bytes()].concat().try_into().unwrap());
        let bytes = data.to_be_bytes();

        let mut cursor = std::io::Cursor::new(bytes.as_slice());
        match (data, i128::fg_read(&mut cursor)) {
            (expected, Ok(read)) => assert_eq!(expected, read),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }
}
