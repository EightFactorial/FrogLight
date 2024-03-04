use super::{FrogWrite, WriteError};

impl FrogWrite for bool {
    #[inline]
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        u8::from(*self).fg_write(buf)
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(32))]

    #[test]
    fn proto_write_bool(data in proptest::bool::ANY) {
        assert_eq!(data.fg_to_bytes(), std::slice::from_ref(&u8::from(data)));
    }

    #[test]
    fn proto_write_bool_vec(data in proptest::collection::vec(proptest::bool::ANY, 0..32)) {
        use crate::io::FrogVarWrite;

        let mut bytes = Vec::with_capacity(data.len() + 1);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut bytes).unwrap();
        bytes.extend(data.iter().map(|b| u8::from(*b)));

        assert_eq!(data.fg_to_bytes(), bytes);
    }
}
