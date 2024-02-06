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
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(128))]

    #[test]
    fn proto_read_bool(data in proptest::collection::vec(0u8..=255u8, 0..128)) {
        let mut cursor = std::io::Cursor::new(data.as_slice());

        for (index, byte) in data.iter().enumerate() {
            match (*byte, bool::fg_read(&mut cursor)) {
                (exp @ (0|1), Ok(read)) => assert_eq!(exp, u8::from(read)),
                (oth, Err(ReadError::InvalidBool(err))) => assert_eq!(oth, err),
                (oth, err) => panic!("Length: `{}`, Index: `{index}`, Data: `{oth}`, Error: `{err:?}`", data.len()),
            }
        }
    }
}
