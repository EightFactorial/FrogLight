#[cfg(test)]
use std::io::Cursor;
use std::{
    collections::{HashMap, HashSet},
    hash::{BuildHasher, Hash},
    io::{Read, Write},
};

#[cfg(test)]
use proptest::prelude::*;

use super::{FrogRead, FrogWrite, ReadError, WriteError};
use crate::variable::{FrogVarRead, FrogVarWrite};

macro_rules! impl_integer {
    ($($ty:ty),*) => {
        $(
            impl FrogRead for $ty {
                #[inline]
                fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
                    let mut bytes = [0u8; std::mem::size_of::<$ty>()];
                    buffer.read_exact(&mut bytes).map_or_else(|err| Err(ReadError::Io(err)), |_| Ok(<$ty>::from_be_bytes(bytes)))
                }
            }
            impl FrogWrite for $ty {
                #[inline]
                fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
                    buffer.write_all(&self.to_be_bytes()).map_or_else(|err| Err(WriteError::Io(err)), |_| Ok(std::mem::size_of::<$ty>()))
                }

                #[inline]
                fn frog_len(&self) -> usize { std::mem::size_of::<$ty>() }
            }
        )*
    };
}

impl_integer!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);

impl FrogRead for bool {
    #[inline]
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        match u8::frog_read(buffer)? {
            0 => Ok(false),
            1 => Ok(true),
            other => Err(ReadError::InvalidBool(other)),
        }
    }
}
impl FrogWrite for bool {
    #[inline]
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        u8::from(*self).frog_write(buffer)
    }
    #[inline]
    fn frog_len(&self) -> usize { std::mem::size_of::<bool>() }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    #[test]
    fn proto_read_u16(data in proptest::bits::u16::ANY) {
        match (data, u16::frog_read(&mut Cursor::new(data.to_be_bytes().as_slice()))) {
            (expected, Ok(read)) => assert_eq!(expected, read),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }
    #[test]
    fn proto_write_u16(data in proptest::num::u16::ANY) {
        assert_eq!(data.frog_to_buf::<Vec<u8>>().unwrap(), data.to_be_bytes());
        assert_eq!(data.frog_len(), std::mem::size_of::<u16>());
    }

    #[test]
    fn proto_read_i16(data in proptest::bits::i16::ANY) {
        match (data, i16::frog_read(&mut Cursor::new(data.to_be_bytes().as_slice()))) {
            (expected, Ok(read)) => assert_eq!(expected, read),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }
    #[test]
    fn proto_write_i16(data in proptest::num::i16::ANY) {
        assert_eq!(data.frog_to_buf::<Vec<u8>>().unwrap(), data.to_be_bytes());
        assert_eq!(data.frog_len(), std::mem::size_of::<i16>());
    }

    #[test]
    fn proto_read_u32(data in proptest::bits::u32::ANY) {
        match (data, u32::frog_read(&mut Cursor::new(data.to_be_bytes().as_slice()))) {
            (expected, Ok(read)) => assert_eq!(expected, read),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }
    #[test]
    fn proto_write_u32(data in proptest::num::u32::ANY) {
        assert_eq!(data.frog_to_buf::<Vec<u8>>().unwrap(), data.to_be_bytes());
        assert_eq!(data.frog_len(), std::mem::size_of::<u32>());
    }

    #[test]
    fn proto_read_i32(data in proptest::bits::i32::ANY) {
        match (data, i32::frog_read(&mut Cursor::new(data.to_be_bytes().as_slice()))) {
            (expected, Ok(read)) => assert_eq!(expected, read),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }
    #[test]
    fn proto_write_i32(data in proptest::num::i32::ANY) {
        assert_eq!(data.frog_to_buf::<Vec<u8>>().unwrap(), data.to_be_bytes());
        assert_eq!(data.frog_len(), std::mem::size_of::<i32>());
    }

    #[test]
    fn proto_read_u64(data in proptest::bits::u64::ANY) {
        match (data, u64::frog_read(&mut Cursor::new(data.to_be_bytes().as_slice()))) {
            (expected, Ok(read)) => assert_eq!(expected, read),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }
    #[test]
    fn proto_write_u64(data in proptest::num::u64::ANY) {
        assert_eq!(data.frog_to_buf::<Vec<u8>>().unwrap(), data.to_be_bytes());
        assert_eq!(data.frog_len(), std::mem::size_of::<u64>());
    }

    #[test]
    fn proto_read_i64(data in proptest::bits::i64::ANY) {
        match (data, i64::frog_read(&mut Cursor::new(data.to_be_bytes().as_slice()))) {
            (expected, Ok(read)) => assert_eq!(expected, read),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }
    #[test]
    fn proto_write_i64(data in proptest::num::i64::ANY) {
        assert_eq!(data.frog_to_buf::<Vec<u8>>().unwrap(), data.to_be_bytes());
        assert_eq!(data.frog_len(), std::mem::size_of::<i64>());
    }

    // #[test]
    // fn proto_read_u128(data in proptest::bits::u128::ANY) {
    //     match (data, u128::frog_read(&mut Cursor::new(data.to_be_bytes().as_slice()))) {
    //         (expected, Ok(read)) => assert_eq!(expected, read),
    //         (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
    //     }
    // }
    #[test]
    fn proto_write_u128(data in proptest::num::u128::ANY) {
        assert_eq!(data.frog_to_buf::<Vec<u8>>().unwrap(), data.to_be_bytes());
        assert_eq!(data.frog_len(), std::mem::size_of::<u128>());
    }

    // #[test]
    // fn proto_read_i128(data in proptest::bits::i128::ANY) {
    //     match (data, i128::frog_read(&mut Cursor::new(data.to_be_bytes().as_slice()))) {
    //         (expected, Ok(read)) => assert_eq!(expected, read),
    //         (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
    //     }
    // }
    #[test]
    fn proto_write_i128(data in proptest::num::i128::ANY) {
        assert_eq!(data.frog_to_buf::<Vec<u8>>().unwrap(), data.to_be_bytes());
        assert_eq!(data.frog_len(), std::mem::size_of::<i128>());
    }

    #[test]
    fn proto_read_f32(data in proptest::num::f32::ANY) {
        match (data, f32::frog_read(&mut Cursor::new(data.to_be_bytes().as_slice()))) {
            (expected, Ok(read)) => assert_eq!(expected.total_cmp(&read), std::cmp::Ordering::Equal),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }
    #[test]
    fn proto_write_f32(data in proptest::num::f32::ANY) {
        assert_eq!(data.frog_to_buf::<Vec<u8>>().unwrap(), data.to_be_bytes());
        assert_eq!(data.frog_len(), std::mem::size_of::<f32>());
    }

    #[test]
    fn proto_read_f64(data in proptest::num::f64::ANY) {
        match (data, f64::frog_read(&mut Cursor::new(data.to_be_bytes().as_slice()))) {
            (expected, Ok(read)) => assert_eq!(expected.total_cmp(&read), std::cmp::Ordering::Equal),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }
    #[test]
    fn proto_write_f64(data in proptest::num::f64::ANY) {
        assert_eq!(data.frog_to_buf::<Vec<u8>>().unwrap(), data.to_be_bytes());
        assert_eq!(data.frog_len(), std::mem::size_of::<f64>());
    }

    #[test]
    fn proto_read_bool(data in proptest::bool::ANY) {
        match (data, bool::frog_read(&mut Cursor::new(std::slice::from_ref(&u8::from(data))))) {
            (expected, Ok(read)) => assert_eq!(expected, read),
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }
    #[test]
    fn proto_write_bool(data in proptest::bool::ANY) {
        assert_eq!(data.frog_to_buf::<Vec<u8>>().unwrap(), std::slice::from_ref(&u8::from(data)));
        assert_eq!(data.frog_len(), std::mem::size_of::<bool>());
    }
}

impl FrogRead for String {
    #[inline]
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        Vec::<u8>::frog_read(buffer)
            .and_then(|bytes| String::from_utf8(bytes).map_err(ReadError::Utf8))
    }
}
impl FrogWrite for String {
    #[inline]
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        <str>::frog_write(self, buffer)
    }
    #[inline]
    fn frog_len(&self) -> usize { <str>::frog_len(self) }
}
impl FrogWrite for str {
    #[inline]
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        <[u8]>::frog_write(self.as_bytes(), buffer)
    }

    #[inline]
    fn frog_len(&self) -> usize { <[u8]>::frog_len(self.as_bytes()) }
}

impl<T: FrogRead> FrogRead for Vec<T> {
    #[inline]
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        (0..usize::frog_var_read(buffer)?).map(|_| T::frog_read(buffer)).collect()
    }
}
impl<T: FrogWrite> FrogWrite for Vec<T> {
    #[inline]
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        <[T]>::frog_write(self, buffer)
    }

    #[inline]
    fn frog_len(&self) -> usize { <[T]>::frog_len(self) }
}
impl<T: FrogWrite> FrogWrite for [T] {
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        self.iter().try_fold(self.len().frog_var_write(buffer)?, |acc, item| {
            item.frog_write(buffer).map(|len| acc + len)
        })
    }

    fn frog_len(&self) -> usize {
        self.iter().fold(self.len().frog_var_len(), |acc, item| acc + item.frog_len())
    }
}

impl<T: FrogRead, const N: usize> FrogRead for [T; N] {
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        std::array::try_from_fn(|_| T::frog_read(buffer))
    }
}
impl<T: FrogWrite, const N: usize> FrogWrite for [T; N] {
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        self.iter().try_fold(0, |acc, item| item.frog_write(buffer).map(|len| acc + len))
    }

    fn frog_len(&self) -> usize { self.iter().fold(0, |acc, item| acc + item.frog_len()) }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn proto_string(data in ".*") {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(usize::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.len());
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_vec_string(data in proptest::collection::vec(".*", 0..32)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec::<String>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_vec_u8(data in proptest::collection::vec(proptest::num::u8::ANY, 0..128)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec::<u8>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_vec_i8(data in proptest::collection::vec(proptest::num::i8::ANY, 0..128)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec::<i8>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_vec_u16(data in proptest::collection::vec(proptest::num::u16::ANY, 0..128)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec::<u16>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_vec_i16(data in proptest::collection::vec(proptest::num::i16::ANY, 0..128)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec::<i16>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_vec_u32(data in proptest::collection::vec(proptest::num::u32::ANY, 0..256)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec::<u32>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_vec_i32(data in proptest::collection::vec(proptest::num::i32::ANY, 0..256)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec::<i32>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_vec_u64(data in proptest::collection::vec(proptest::num::u64::ANY, 0..256)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec::<u64>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_vec_i64(data in proptest::collection::vec(proptest::num::i64::ANY, 0..256)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec::<i64>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_vec_u128(data in proptest::collection::vec(proptest::num::u128::ANY, 0..256)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec::<u128>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_vec_i128(data in proptest::collection::vec(proptest::num::i128::ANY, 0..256)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec::<i128>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_vec_f32(data in proptest::collection::vec(proptest::num::f32::ANY, 0..256)) {
        if data.iter().any(|x| x.is_nan()) {
            return Ok(());
        }

        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec::<f32>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_vec_f64(data in proptest::collection::vec(proptest::num::f64::ANY, 0..256)) {
        if data.iter().any(|x| x.is_nan()) {
            return Ok(());
        }

        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec::<f64>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_array_1_u8(data in proptest::array::uniform1(proptest::num::u8::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(<[u8; 1]>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_array_2_u8(data in proptest::array::uniform2(proptest::num::u8::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(<[u8; 2]>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_array_3_u64(data in proptest::array::uniform3(proptest::num::u64::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(<[u64; 3]>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_array_4_bool(data in proptest::array::uniform4(proptest::bool::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(<[bool; 4]>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_array_5_string(data in proptest::array::uniform5(".*")) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(<[String; 5]>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
}

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
    fn proto_hashmap_u8_u8(data in proptest::collection::hash_map(proptest::num::u8::ANY, proptest::num::u8::ANY, 0..128)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashMap::<u8, u8>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashmap_u8_u16(data in proptest::collection::hash_map(proptest::num::u8::ANY, proptest::num::u16::ANY, 0..128)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashMap::<u8, u16>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashmap_u8_string(data in proptest::collection::hash_map(proptest::num::u8::ANY, ".*", 0..32)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashMap::<u8, String>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashmap_u8_vec_u8(data in proptest::collection::hash_map(proptest::num::u8::ANY, proptest::collection::vec(proptest::num::u8::ANY, 0..128), 0..16)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashMap::<u8, Vec<u8>>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_hashmap_string_u32(data in proptest::collection::hash_map(".*", proptest::num::u32::ANY, 0..32)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashMap::<String, u32>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashmap_string_string(data in proptest::collection::hash_map(".*", ".*", 0..32)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashMap::<String, String>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashmap_string_vec_u128(data in proptest::collection::hash_map(".*", proptest::collection::vec(proptest::num::u128::ANY, 0..128), 0..16)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashMap::<String, Vec<u128>>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_hashset_u8(data in proptest::collection::hash_set(proptest::num::u8::ANY, 0..128)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashSet::<u8>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashset_u16(data in proptest::collection::hash_set(proptest::num::u16::ANY, 0..128)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashSet::<u16>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashset_u128(data in proptest::collection::hash_set(proptest::num::u128::ANY, 0..128)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashSet::<u128>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashset_string(data in proptest::collection::hash_set(".*", 0..32)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashSet::<String>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashset_vec_u8(data in proptest::collection::hash_set(proptest::collection::vec(proptest::num::u8::ANY, 0..128), 0..16)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashSet::<Vec<u8>>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
}

impl<T: FrogRead> FrogRead for Option<T> {
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        if bool::frog_read(buffer)? {
            T::frog_read(buffer).map(Some)
        } else {
            Ok(None)
        }
    }
}
impl<T: FrogWrite> FrogWrite for Option<T> {
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        if let Some(value) = self {
            let prefix = bool::frog_write(&true, buffer)?;
            value.frog_write(buffer).map(|len| prefix + len)
        } else {
            bool::frog_write(&false, buffer)
        }
    }

    fn frog_len(&self) -> usize {
        if let Some(value) = self {
            bool::frog_len(&true) + value.frog_len()
        } else {
            bool::frog_len(&false)
        }
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn proto_option_bool(data in proptest::option::of(proptest::bool::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Option::<bool>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_option_u8(data in proptest::option::of(proptest::num::u8::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Option::<u8>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_option_u32(data in proptest::option::of(proptest::num::u32::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Option::<u32>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_option_string(data in proptest::option::of(".*")) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Option::<String>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_option_option_bool(data in proptest::option::of(proptest::option::of(proptest::bool::ANY))) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Option::<Option<bool>>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_option_option_option_string(data in proptest::option::of(proptest::option::of(proptest::option::of(".*")))) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Option::<Option<Option<String>>>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_option_vec_u8(data in proptest::option::of(proptest::collection::vec(proptest::num::u8::ANY, 0..128))) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Option::<Vec<u8>>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_vec_option_u64(data in proptest::collection::vec(proptest::option::of(proptest::num::u64::ANY), 0..128)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec::<Option<u64>>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_option_hashmap_u8_u32(data in proptest::option::of(proptest::collection::hash_map(proptest::num::u8::ANY, proptest::num::u32::ANY, 0..128))) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Option::<HashMap<u8, u32>>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashmap_u8_option_u32(data in proptest::collection::hash_map(proptest::num::u8::ANY, proptest::option::of(proptest::num::u32::ANY), 0..128)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashMap::<u8, Option<u32>>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_hashset_option_u32(data in proptest::collection::hash_set(proptest::option::of(proptest::num::u32::ANY), 0..128)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(HashSet::<Option<u32>>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }
}
