#[cfg(test)]
use std::io::Cursor;
use std::{
    hash::{BuildHasher, Hash},
    io::{Read, Write},
};

use bevy_platform::collections::{HashMap, HashSet};
#[cfg(test)]
use proptest::prelude::*;

use super::{FrogVarRead, FrogVarWrite, ReadError, WriteError};
use crate::standard::{FrogRead, FrogWrite};

impl<K: FrogVarRead + Eq + Hash, V: FrogRead, S: BuildHasher + Default> FrogVarRead
    for HashMap<K, V, S>
{
    fn frog_var_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        (0..usize::frog_var_read(buffer)?)
            .map(|_| Ok((K::frog_var_read(buffer)?, V::frog_read(buffer)?)))
            .collect()
    }
}
impl<K: FrogVarWrite, V: FrogWrite, S> FrogVarWrite for HashMap<K, V, S> {
    fn frog_var_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        self.iter().try_fold(self.len().frog_var_write(buffer)?, |acc, (key, value)| {
            key.frog_var_write(buffer)
                .and_then(|len| value.frog_write(buffer).map(|len2| acc + len + len2))
        })
    }

    fn frog_var_len(&self) -> usize {
        self.iter().fold(self.len().frog_var_len(), |acc, (key, value)| {
            acc + key.frog_var_len() + value.frog_len()
        })
    }
}

impl<T: FrogVarRead + Eq + Hash, S: BuildHasher + Default> FrogVarRead for HashSet<T, S> {
    fn frog_var_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        (0..usize::frog_var_read(buffer)?).map(|_| T::frog_var_read(buffer)).collect()
    }
}
impl<T: FrogVarWrite, S: BuildHasher> FrogVarWrite for HashSet<T, S> {
    fn frog_var_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        self.iter().try_fold(self.len().frog_var_write(buffer)?, |acc, item| {
            item.frog_var_write(buffer).map(|len| acc + len)
        })
    }

    fn frog_var_len(&self) -> usize {
        self.iter().fold(self.len().frog_var_len(), |acc, item| acc + item.frog_var_len())
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn proto_hashbrown_hashmap_u32_varint_u32(data in proptest::collection::hash_map(proptest::num::u32::ANY, proptest::num::u32::ANY, 0..256)) {
        let data = HashMap::from_iter(data);
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(HashMap::<u32, u32>::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }
    #[test]
    fn proto_hashbrown_hashmap_u64_varint_u64(data in proptest::collection::hash_map(proptest::num::u64::ANY, proptest::num::u64::ANY, 0..256)) {
        let data = HashMap::from_iter(data);
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(HashMap::<u64, u64>::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }

    #[test]
    fn proto_hashbrown_hashset_varint_u32(data in proptest::collection::hash_set(proptest::num::u32::ANY, 0..256)) {
        let data = HashSet::from_iter(data);
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(HashSet::<u32>::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }
    #[test]
    fn proto_hashbrown_hashset_varint_u64(data in proptest::collection::hash_set(proptest::num::u64::ANY, 0..256)) {
        let data = HashSet::from_iter(data);
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(HashSet::<u64>::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }
}
