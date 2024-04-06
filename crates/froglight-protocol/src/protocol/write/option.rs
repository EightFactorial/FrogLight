use super::{FrogWrite, WriteError};

impl<T: FrogWrite> FrogWrite for Option<T> {
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        match self {
            Some(value) => {
                true.fg_write(buf)?;
                value.fg_write(buf)
            }
            None => false.fg_write(buf),
        }
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(128))]

    #[test]
    fn proto_write_option_bool(data in proptest::option::of(proptest::bool::ANY)) {
        let mut bytes = Vec::with_capacity(2);
        match &data {
            Some(value) => {
                true.fg_write(&mut bytes).unwrap();
                value.fg_write(&mut bytes).unwrap();
            }
            None => false.fg_write(&mut bytes).unwrap(),
        }

        assert_eq!(data.fg_to_bytes(), bytes);
    }

    #[test]
    fn proto_write_option_u8(data in proptest::option::of(proptest::num::u8::ANY)) {
        let mut bytes = Vec::with_capacity(2);
        match &data {
            Some(value) => {
                true.fg_write(&mut bytes).unwrap();
                value.fg_write(&mut bytes).unwrap();
            }
            None => false.fg_write(&mut bytes).unwrap(),
        }

        assert_eq!(data.fg_to_bytes(), bytes);
    }

    #[test]
    fn proto_write_option_string(data in proptest::option::of(".*")) {
        let mut bytes = Vec::with_capacity(2);
        match &data {
            Some(value) => {
                true.fg_write(&mut bytes).unwrap();
                value.fg_write(&mut bytes).unwrap();
            }
            None => false.fg_write(&mut bytes).unwrap(),
        }

        assert_eq!(data.fg_to_bytes(), bytes);
    }

    #[test]
    fn proto_write_vec_option_i32(data in proptest::collection::vec(proptest::option::of(proptest::num::i32::ANY), 0..128)) {
        use crate::protocol::FrogVarWrite;

        let mut bytes = Vec::with_capacity(data.len() + 1);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut bytes).unwrap();
        for value in &data {
            match value {
                Some(value) => {
                    true.fg_write(&mut bytes).unwrap();
                    value.fg_write(&mut bytes).unwrap();
                }
                None => false.fg_write(&mut bytes).unwrap(),
            }
        }

        assert_eq!(data.fg_to_bytes(), bytes);
    }
}
