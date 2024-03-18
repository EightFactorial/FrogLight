use serde_json::Value;
use simdnbt::owned::Nbt;
use uuid::Uuid;

use super::{FrogWrite, WriteError};

impl FrogWrite for () {
    #[inline]
    fn fg_write(&self, _: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> { Ok(()) }
}

impl FrogWrite for Nbt {
    #[inline]
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        let mut vec = Vec::new();
        self.write(&mut vec);

        Ok(buf.write_all(&vec)?)
    }
}

impl FrogWrite for Uuid {
    #[inline]
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        self.as_u128().fg_write(buf)
    }
}

impl FrogWrite for Value {
    #[inline]
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        let content = serde_json::to_string(self)?;
        content.fg_write(buf)
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(128))]

    #[test]
    fn proto_write_uuid(data in proptest::array::uniform2(proptest::num::u64::ANY)) {
        let data = Uuid::from_u64_pair(data[0], data[1]);
        assert_eq!(data.fg_to_bytes(), data.into_bytes());
    }

    #[test]
    fn proto_write_option_uuid(data in proptest::option::of(proptest::array::uniform2(proptest::num::u64::ANY))) {
        let mut bytes: Vec<u8> = Vec::with_capacity(17);
        match &data {
            Some(value) => {
                bytes.extend(&[1]);
                bytes.extend(Uuid::from_u64_pair(value[0], value[1]).into_bytes());
            }
            None => bytes.extend(&[0]),
        }
        assert_eq!(data.fg_to_bytes(), bytes);
    }
}
