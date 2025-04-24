#[cfg(test)]
use std::io::Cursor;
use std::{
    hash::{BuildHasher, Hash},
    io::{Read, Write},
};

use bevy_platform::collections::{HashMap, HashSet};
#[cfg(test)]
use proptest::prelude::*;

use super::{FrogRead, FrogWrite, ReadError, WriteError};
use crate::variable::{FrogVarRead, FrogVarWrite};

impl<K: FrogRead + Eq + Hash, V: FrogRead, S: BuildHasher + Default> FrogRead for HashMap<K, V, S> {
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        (0..usize::frog_var_read(buffer)?)
            .map(|_| Ok((K::frog_read(buffer)?, V::frog_read(buffer)?)))
            .collect()
    }
}
impl<K: FrogWrite, V: FrogWrite, S> FrogWrite for HashMap<K, V, S> {
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        self.iter().try_fold(self.len().frog_var_write(buffer)?, |acc, (key, value)| {
            key.frog_write(buffer)
                .and_then(|len| value.frog_write(buffer).map(|len2| acc + len + len2))
        })
    }

    fn frog_len(&self) -> usize {
        self.iter().fold(self.len().frog_var_len(), |acc, (key, value)| {
            acc + key.frog_len() + value.frog_len()
        })
    }
}

impl<T: FrogRead + Eq + Hash, S: BuildHasher + Default> FrogRead for HashSet<T, S> {
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        (0..usize::frog_var_read(buffer)?).map(|_| T::frog_read(buffer)).collect()
    }
}
impl<T: FrogWrite, S: BuildHasher> FrogWrite for HashSet<T, S> {
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        self.iter().try_fold(self.len().frog_var_write(buffer)?, |acc, item| {
            item.frog_write(buffer).map(|len| acc + len)
        })
    }

    fn frog_len(&self) -> usize {
        self.iter().fold(self.len().frog_var_len(), |acc, item| acc + item.frog_len())
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn proto_hashbrown_hashmap_u8_u8(data in proptest::collection::hash_map(proptest::num::u8::ANY, proptest::num::u8::ANY, 0..128)) {
        let data = HashMap::from_iter(data);
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashMap::<u8, u8>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashbrown_hashmap_u8_u16(data in proptest::collection::hash_map(proptest::num::u8::ANY, proptest::num::u16::ANY, 0..128)) {
        let data = HashMap::from_iter(data);
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashMap::<u8, u16>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashbrown_hashmap_u8_string(data in proptest::collection::hash_map(proptest::num::u8::ANY, ".*", 0..32)) {
        let data = HashMap::from_iter(data);
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashMap::<u8, String>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashbrown_hashmap_u8_vec_u8(data in proptest::collection::hash_map(proptest::num::u8::ANY, proptest::collection::vec(proptest::num::u8::ANY, 0..128), 0..16)) {
        let data = HashMap::from_iter(data);
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashMap::<u8, Vec<u8>>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_hashbrown_hashmap_string_u32(data in proptest::collection::hash_map(".*", proptest::num::u32::ANY, 0..32)) {
        let data = HashMap::from_iter(data);
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashMap::<String, u32>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashbrown_hashmap_string_string(data in proptest::collection::hash_map(".*", ".*", 0..32)) {
        let data = HashMap::from_iter(data);
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashMap::<String, String>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashbrown_hashmap_string_vec_u128(data in proptest::collection::hash_map(".*", proptest::collection::vec(proptest::num::u128::ANY, 0..128), 0..16)) {
        let data = HashMap::from_iter(data);
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashMap::<String, Vec<u128>>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_hashbrown_hashset_u8(data in proptest::collection::hash_set(proptest::num::u8::ANY, 0..128)) {
        let data = HashSet::from_iter(data);
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashSet::<u8>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashbrown_hashset_u16(data in proptest::collection::hash_set(proptest::num::u16::ANY, 0..128)) {
        let data = HashSet::from_iter(data);
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashSet::<u16>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashbrown_hashset_u128(data in proptest::collection::hash_set(proptest::num::u128::ANY, 0..128)) {
        let data = HashSet::from_iter(data);
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashSet::<u128>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashbrown_hashset_string(data in proptest::collection::hash_set(".*", 0..32)) {
        let data = HashSet::from_iter(data);
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashSet::<String>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashbrown_hashset_vec_u8(data in proptest::collection::hash_set(proptest::collection::vec(proptest::num::u8::ANY, 0..128), 0..16)) {
        let data = HashSet::from_iter(data);
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashSet::<Vec<u8>>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
}
