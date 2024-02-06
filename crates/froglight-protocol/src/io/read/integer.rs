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
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(128))]

    #[test]
    fn proto_read_u8(data in proptest::collection::vec(u8::MIN..u8::MAX, 0..128)) {
        let mut cursor = std::io::Cursor::new(data.as_slice());

        for (index, (expected, result)) in data.iter().map(|&byte| (byte, u8::fg_read(&mut cursor))).enumerate() {
            match result {
                Ok(read) => assert_eq!(expected, read),
                Err(err) => panic!("Length: `{}`, Index: `{index}`, Expected: `{expected}`, Error: `{err}`", data.len()),
            }
        }
    }

    #[test]
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn proto_read_i8(data in proptest::collection::vec(i8::MIN..i8::MAX, 0..128)) {
        // Convert the data to bytes
        let data = data.iter().map(|&i| i as u8).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(data.as_slice());

        for (index, (expected, result)) in data.iter().map(|&byte| (byte, i8::fg_read(&mut cursor))).enumerate() {
            match result {
                Ok(read) => assert_eq!(expected as i8, read),
                Err(err) => panic!("Length: `{}`, Index: `{index}`, Expected: `{expected}`, Error: `{err}`", data.len()),
            }
        }
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(512))]

    #[test]
    fn proto_read_u16(data in proptest::collection::vec(u16::MIN..u16::MAX, 0..256)) {
        // Convert the data to bytes
        let data = data.iter().flat_map(|i| i.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(data.as_slice());

        for (expected, result) in data.chunks_exact(2).map(|chunk|
            (u16::from_be_bytes([chunk[0], chunk[1]]), u16::fg_read(&mut cursor))
        ) {
            match result {
                Ok(read) => assert_eq!(expected, read),
                Err(err) => panic!("Expected: `{expected}`, Error: `{err}`"),
            }
        }
    }

    #[test]
    fn proto_read_u32(data in proptest::collection::vec(u32::MIN..u32::MAX, 0..512)) {
        // Convert the data to bytes
        let data = data.iter().flat_map(|i| i.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(data.as_slice());

        for (expected, result) in data.chunks_exact(4).map(|chunk|
            (u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]), u32::fg_read(&mut cursor))
        ) {
            match result {
                Ok(read) => assert_eq!(expected, read),
                Err(err) => panic!("Expected: `{expected}`, Error: `{err}`"),
            }
        }
    }

    #[test]
    fn proto_read_u64(data in proptest::collection::vec(u64::MIN..u64::MAX, 0..512)) {
        // Convert the data to bytes
        let data = data.iter().flat_map(|i| i.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(data.as_slice());

        for (expected, result) in data.chunks_exact(8).map(|chunk|
            (u64::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7]]), u64::fg_read(&mut cursor))
        ) {
            match result {
                Ok(read) => assert_eq!(expected, read),
                Err(err) => panic!("Expected: `{expected}`, Error: `{err}`"),
            }
        }
    }

    #[test]
    fn proto_read_u128(data in proptest::collection::vec(u128::MIN..u128::MAX, 0..512)) {
        // Convert the data to bytes
        let data = data.iter().flat_map(|i| i.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(data.as_slice());

        for (expected, result) in data.chunks_exact(16).map(|chunk|
            (u128::from_be_bytes([
                chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
                chunk[8], chunk[9], chunk[10], chunk[11], chunk[12], chunk[13], chunk[14], chunk[15],
            ]), u128::fg_read(&mut cursor))
        ) {
            match result {
                Ok(read) => assert_eq!(expected, read),
                Err(err) => panic!("Expected: `{expected}`, Error: `{err}`"),
            }
        }
    }

    #[test]
    fn proto_read_i16(data in proptest::collection::vec(i16::MIN..i16::MAX, 0..256)) {
        // Convert the data to bytes
        let data = data.iter().flat_map(|i| i.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(data.as_slice());

        for (expected, result) in data.chunks_exact(2).map(|chunk|
            (i16::from_be_bytes([chunk[0], chunk[1]]), i16::fg_read(&mut cursor))
        ) {
            match result {
                Ok(read) => assert_eq!(expected, read),
                Err(err) => panic!("Expected: `{expected}`, Error: `{err}`"),
            }
        }
    }


    #[test]
    fn proto_read_i32(data in proptest::collection::vec(i32::MIN..i32::MAX, 0..512)) {
        // Convert the data to bytes
        let data = data.iter().flat_map(|i| i.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(data.as_slice());

        for (expected, result) in data.chunks_exact(4).map(|chunk|
            (i32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]), i32::fg_read(&mut cursor))
        ) {
            match result {
                Ok(read) => assert_eq!(expected, read),
                Err(err) => panic!("Expected: `{expected}`, Error: `{err}`"),
            }
        }
    }

    #[test]
    fn proto_read_i64(data in proptest::collection::vec(i64::MIN..i64::MAX, 0..512)) {
        // Convert the data to bytes
        let data = data.iter().flat_map(|i| i.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(data.as_slice());

        for (expected, result) in data.chunks_exact(8).map(|chunk|
            (i64::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7]]), i64::fg_read(&mut cursor))
        ) {
            match result {
                Ok(read) => assert_eq!(expected, read),
                Err(err) => panic!("Expected: `{expected}`, Error: `{err}`"),
            }
        }
    }

    #[test]
    fn proto_read_i128(data in proptest::collection::vec(i128::MIN..i128::MAX, 0..512)) {
        // Convert the data to bytes
        let data = data.iter().flat_map(|i| i.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(data.as_slice());

        for (expected, result) in data.chunks_exact(16).map(|chunk|
            (i128::from_be_bytes([
                chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
                chunk[8], chunk[9], chunk[10], chunk[11], chunk[12], chunk[13], chunk[14], chunk[15],
            ]), i128::fg_read(&mut cursor))
        ) {
            match result {
                Ok(read) => assert_eq!(expected, read),
                Err(err) => panic!("Expected: `{expected}`, Error: `{err}`"),
            }
        }
    }
}
