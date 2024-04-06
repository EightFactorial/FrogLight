use super::{FrogRead, ReadError};

impl<T: FrogRead> FrogRead for Option<T> {
    #[inline]
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        if bool::fg_read(buf)? {
            Ok(Some(T::fg_read(buf)?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(128))]

    #[test]
    fn proto_read_option_u8(data in proptest::option::of(proptest::bits::u8::ANY)) {
        let vec = match &data {
            Some(num) => vec![1, *num],
            None => vec![0],
        };

        let mut cursor = std::io::Cursor::new(vec.as_slice());
        match (data, Option::<u8>::fg_read(&mut cursor)) {
            (expected, Ok(read)) => {
                assert_eq!(expected, read);
                assert_eq!(cursor.position(), vec.len() as u64);
            }
            (expected, Err(err)) => panic!("Expected: `{expected:?}`, Error: `{err:?}`"),
        }
    }

    #[test]
    fn proto_read_option_u16(data in proptest::option::of(proptest::bits::u16::ANY)) {
        // Convert the data to bytes
        let mut vec;
        match &data {
            Some(num) => {
                vec = Vec::with_capacity(3);
                vec.push(1);

                vec.extend_from_slice(&num.to_be_bytes());
            }
            None => vec = vec![0],
        }

        let mut cursor = std::io::Cursor::new(vec.as_slice());
        match (data, Option::<u16>::fg_read(&mut cursor)) {
            (expected, Ok(read)) => {
                assert_eq!(expected, read);
                assert_eq!(cursor.position(), vec.len() as u64);
            }
            (expected, Err(err)) => panic!("Expected: `{expected:?}`, Error: `{err:?}`"),
        }
    }

    #[test]
    fn proto_read_option_string(data in proptest::option::of(".*")) {
        use crate::protocol::var_write::FrogVarWrite;

        // Convert the data to bytes
        let mut vec;
        match &data {
            Some(string) => {
                vec = Vec::with_capacity(3 + string.len());
                vec.push(1);

                let len = u32::try_from(string.len()).unwrap();
                len.fg_var_write(&mut vec).unwrap();

                vec.extend_from_slice(string.as_bytes());
            }
            None => vec = vec![0],
        }

        let mut cursor = std::io::Cursor::new(vec.as_slice());
        match (data, Option::<String>::fg_read(&mut cursor)) {
            (expected, Ok(read)) => {
                assert_eq!(expected, read);
                assert_eq!(cursor.position(), vec.len() as u64);
            }
            (expected, Err(err)) => panic!("Expected: `{expected:?}`, Error: `{err:?}`"),
        }
    }
}
