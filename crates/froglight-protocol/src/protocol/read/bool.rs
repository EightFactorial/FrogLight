use super::{FrogRead, ReadError};

impl FrogRead for bool {
    #[inline]
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        match u8::fg_read(buf)? {
            0 => Ok(false),
            1 => Ok(true),
            o => Err(ReadError::InvalidBool(o)),
        }
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(256))]

    #[test]
    fn proto_read_bool(data in proptest::bits::u8::ANY) {
        use bitvec::view::BitViewSized;

        let mut cursor = std::io::Cursor::new(data.as_raw_slice());
        match (data, bool::fg_read(&mut cursor)) {
            (exp @ (0|1), Ok(read)) => assert_eq!(exp, u8::from(read)),
            (oth, Err(ReadError::InvalidBool(err))) => assert_eq!(oth, err),
            (oth, err) => panic!("Data: `{oth}`, Error: `{err:?}`"),
        }
    }
}
