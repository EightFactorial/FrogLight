#[cfg(test)]
use std::io::Cursor;
use std::io::{Read, Write};

#[cfg(test)]
use proptest::prelude::*;
use uuid::Uuid;

use super::{FrogRead, FrogWrite, ReadError, WriteError};

impl FrogRead for Uuid {
    #[inline]
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        u128::frog_read(buffer).map(Uuid::from_u128)
    }
}

impl FrogWrite for Uuid {
    #[inline]
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        self.as_u128().frog_write(buffer)
    }

    #[inline]
    fn frog_len(&self) -> usize { std::mem::size_of::<u128>() }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn proto_uuid(data in proptest::num::u128::ANY) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Uuid::frog_read(&mut Cursor::new(&buffer)).unwrap().as_u128(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_option_uuid(data in proptest::option::of(proptest::num::u128::ANY)) {
        let data = data.map(Uuid::from_u128);
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Option::<Uuid>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_vec_uuid(data in proptest::collection::vec(proptest::num::u128::ANY, 0..256)) {
        let data = data.into_iter().map(Uuid::from_u128).collect::<Vec<_>>();
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec::<Uuid>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    #[expect(clippy::disallowed_types)]
    fn proto_hashmap_uuid_u8(data in proptest::collection::hash_map(proptest::num::u128::ANY, proptest::num::u8::ANY, 0..256)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(std::collections::HashMap::<Uuid, u8>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.iter().map(|(k, v)| (Uuid::from_u128(*k), *v)).collect());
        assert_eq!(data.frog_len(), buffer.len());
    }
}
