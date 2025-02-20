#[cfg(test)]
use std::io::Cursor;
use std::{
    collections::{HashMap, HashSet},
    hash::{BuildHasher, Hash},
    io::{Read, Write},
};

#[cfg(test)]
use proptest::prelude::*;

use super::{FrogVarRead, FrogVarWrite};
use crate::standard::{FrogRead, FrogWrite, ReadError, WriteError};

macro_rules! impl_variable_integer {
    ($($ty:ty, $size:expr),*) => {
        $(
            impl FrogVarRead for $ty {
                fn frog_var_read(buffer: &mut impl Read) -> Result<$ty, ReadError> {
                    let mut byte = [0];
                    let mut number: $ty = 0;
                    for i in 0..$size {
                        buffer.read_exact(&mut byte)?;
                        number |= <$ty>::from(byte[0] & 0b0111_1111) << (7 * i);
                        if byte[0] & 0b1000_0000 == 0 {
                            break;
                        }
                    }
                    Ok(number)
                }
            }
            impl FrogVarWrite for $ty {
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                fn frog_var_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
                    let mut count = 0;
                    let mut byte = [0];
                    let mut number = *self;
                    while (number != 0 || count == 0) && count < $size {
                        byte[0] = (number & 0b0111_1111) as u8;
                        number = (number >> 7) & (<$ty>::MAX >> 6);
                        if number != 0 {
                            byte[0] |= 0b1000_0000;
                        }

                        count += 1;
                        buffer.write_all(&byte)?;
                    }
                    Ok(count)
                }
                fn frog_var_len(&self) -> usize {
                    for i in 1..$size {
                        if (self & ((<$ty>::MAX >> 1) << (7 * i))) == 0 {
                            return i;
                        }
                    }
                    $size
                }
            }
        )*
    };
}

impl_variable_integer!(u16, 3, u32, 5, u64, 10, u128, 19);
impl_variable_integer!(i16, 3, i32, 5, i64, 10, i128, 19);

impl FrogVarRead for usize {
    #[inline]
    fn frog_var_read(buffer: &mut impl Read) -> Result<usize, ReadError> {
        u32::frog_var_read(buffer).map(|value| usize::try_from(value).unwrap_or(u32::MAX as usize))
    }
}
impl FrogVarWrite for usize {
    #[inline]
    fn frog_var_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        u32::try_from(*self).unwrap_or(u32::MAX).frog_var_write(buffer)
    }
    #[inline]
    fn frog_var_len(&self) -> usize { u32::try_from(*self).unwrap_or(u32::MAX).frog_var_len() }
}

impl FrogVarRead for isize {
    #[inline]
    fn frog_var_read(buffer: &mut impl Read) -> Result<isize, ReadError> {
        i32::frog_var_read(buffer).map(|value| isize::try_from(value).unwrap_or(i32::MAX as isize))
    }
}
impl FrogVarWrite for isize {
    #[inline]
    fn frog_var_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        i32::try_from(*self)
            .unwrap_or(if self.is_negative() { i32::MIN } else { i32::MAX })
            .frog_var_write(buffer)
    }
    #[inline]
    fn frog_var_len(&self) -> usize { i32::try_from(*self).unwrap_or(i32::MAX).frog_var_len() }
}

#[test]
fn proto_verify_varint_u32() {
    assert_eq!(0u32.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![0]);
    assert_eq!(0u32.frog_var_len(), 1);

    assert_eq!(1u32.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![1]);
    assert_eq!(1u32.frog_var_len(), 1);

    assert_eq!(2u32.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![2]);
    assert_eq!(2u32.frog_var_len(), 1);

    assert_eq!(127u32.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![127]);
    assert_eq!(127u32.frog_var_len(), 1);

    assert_eq!(128u32.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![128, 1]);
    assert_eq!(128u32.frog_var_len(), 2);

    assert_eq!(254u32.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![254, 1]);
    assert_eq!(254u32.frog_var_len(), 2);

    assert_eq!(255u32.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![255, 1]);
    assert_eq!(255u32.frog_var_len(), 2);

    assert_eq!(25565u32.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![221, 199, 1]);
    assert_eq!(25565u32.frog_var_len(), 3);

    assert_eq!(2_097_151_u32.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![255, 255, 127]);
    assert_eq!(2_097_151_u32.frog_var_len(), 3);

    assert_eq!(
        2_147_483_647_u32.frog_to_var_buf::<Vec<u8>>().unwrap(),
        vec![255, 255, 255, 255, 7]
    );
    assert_eq!(2_147_483_647_u32.frog_var_len(), 5);

    assert_eq!(
        4_294_967_295_u32.frog_to_var_buf::<Vec<u8>>().unwrap(),
        vec![255, 255, 255, 255, 15]
    );
    assert_eq!(4_294_967_295_u32.frog_var_len(), 5);
}
#[test]
fn proto_verify_varint_i32() {
    assert_eq!(
        (-2_147_483_648_i32).frog_to_var_buf::<Vec<u8>>().unwrap(),
        vec![128, 128, 128, 128, 8]
    );
    assert_eq!((-2_147_483_648_i32).frog_var_len(), 5);

    assert_eq!((-1i32).frog_to_var_buf::<Vec<u8>>().unwrap(), vec![255, 255, 255, 255, 15]);
    assert_eq!((-1i32).frog_var_len(), 5);

    assert_eq!(0i32.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![0]);
    assert_eq!(0i32.frog_var_len(), 1);

    assert_eq!(1i32.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![1]);
    assert_eq!(1i32.frog_var_len(), 1);

    assert_eq!(2i32.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![2]);
    assert_eq!(2i32.frog_var_len(), 1);

    assert_eq!(127i32.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![127]);
    assert_eq!(127i32.frog_var_len(), 1);

    assert_eq!(128i32.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![128, 1]);
    assert_eq!(128i32.frog_var_len(), 2);

    assert_eq!(254i32.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![254, 1]);
    assert_eq!(254i32.frog_var_len(), 2);

    assert_eq!(255i32.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![255, 1]);
    assert_eq!(255i32.frog_var_len(), 2);

    assert_eq!(25565i32.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![221, 199, 1]);
    assert_eq!(25565i32.frog_var_len(), 3);

    assert_eq!(2_097_151_i32.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![255, 255, 127]);
    assert_eq!(2_097_151_i32.frog_var_len(), 3);

    assert_eq!(
        2_147_483_647_i32.frog_to_var_buf::<Vec<u8>>().unwrap(),
        vec![255, 255, 255, 255, 7]
    );
    assert_eq!(2_147_483_647_i32.frog_var_len(), 5);
}

#[test]
fn proto_verify_varint_u64() {
    assert_eq!(0u64.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![0]);
    assert_eq!(0u64.frog_var_len(), 1);

    assert_eq!(1u64.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![1]);
    assert_eq!(1u64.frog_var_len(), 1);

    assert_eq!(2u64.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![2]);
    assert_eq!(2u64.frog_var_len(), 1);

    assert_eq!(127u64.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![127]);
    assert_eq!(127u64.frog_var_len(), 1);

    assert_eq!(128u64.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![128, 1]);
    assert_eq!(128u64.frog_var_len(), 2);

    assert_eq!(254u64.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![254, 1]);
    assert_eq!(254u64.frog_var_len(), 2);

    assert_eq!(255u64.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![255, 1]);
    assert_eq!(255u64.frog_var_len(), 2);

    assert_eq!(25565u64.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![221, 199, 1]);
    assert_eq!(25565u64.frog_var_len(), 3);

    assert_eq!(2_097_151_u64.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![255, 255, 127]);
    assert_eq!(2_097_151_u64.frog_var_len(), 3);

    assert_eq!(
        2_147_483_647_u64.frog_to_var_buf::<Vec<u8>>().unwrap(),
        vec![255, 255, 255, 255, 7]
    );
    assert_eq!(2_147_483_647_u64.frog_var_len(), 5);

    assert_eq!(
        4_294_967_295_u64.frog_to_var_buf::<Vec<u8>>().unwrap(),
        vec![255, 255, 255, 255, 15]
    );
    assert_eq!(4_294_967_295_u64.frog_var_len(), 5);

    assert_eq!(
        9_223_372_036_854_775_807_u64.frog_to_var_buf::<Vec<u8>>().unwrap(),
        vec![255, 255, 255, 255, 255, 255, 255, 255, 127]
    );
    assert_eq!(9_223_372_036_854_775_807_u64.frog_var_len(), 9);

    assert_eq!(
        18_446_744_073_709_551_615_u64.frog_to_var_buf::<Vec<u8>>().unwrap(),
        vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1]
    );
    assert_eq!(18_446_744_073_709_551_615_u64.frog_var_len(), 10);
}
#[test]
fn proto_verify_varint_i64() {
    assert_eq!(
        (-9_223_372_036_854_775_808_i64).frog_to_var_buf::<Vec<u8>>().unwrap(),
        vec![128, 128, 128, 128, 128, 128, 128, 128, 128, 1]
    );
    assert_eq!((-9_223_372_036_854_775_808_i64).frog_var_len(), 10);

    assert_eq!(
        (-2_147_483_648_i64).frog_to_var_buf::<Vec<u8>>().unwrap(),
        vec![128, 128, 128, 128, 248, 255, 255, 255, 255, 1]
    );
    assert_eq!((-2_147_483_648_i64).frog_var_len(), 10);

    assert_eq!(
        (-1i64).frog_to_var_buf::<Vec<u8>>().unwrap(),
        vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1]
    );
    assert_eq!((-1i64).frog_var_len(), 10);

    assert_eq!(0i64.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![0]);
    assert_eq!(0i64.frog_var_len(), 1);

    assert_eq!(1i64.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![1]);
    assert_eq!(1i64.frog_var_len(), 1);

    assert_eq!(2i64.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![2]);
    assert_eq!(2i64.frog_var_len(), 1);

    assert_eq!(127i64.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![127]);
    assert_eq!(127i64.frog_var_len(), 1);

    assert_eq!(128i64.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![128, 1]);
    assert_eq!(128i64.frog_var_len(), 2);

    assert_eq!(254i64.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![254, 1]);
    assert_eq!(254i64.frog_var_len(), 2);

    assert_eq!(255i64.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![255, 1]);
    assert_eq!(255i64.frog_var_len(), 2);

    assert_eq!(25565i64.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![221, 199, 1]);
    assert_eq!(25565i64.frog_var_len(), 3);
    assert_eq!(2_097_151_i64.frog_to_var_buf::<Vec<u8>>().unwrap(), vec![255, 255, 127]);
    assert_eq!(2_097_151_i64.frog_var_len(), 3);

    assert_eq!(
        2_147_483_647_i64.frog_to_var_buf::<Vec<u8>>().unwrap(),
        vec![255, 255, 255, 255, 7]
    );
    assert_eq!(2_147_483_647_i64.frog_var_len(), 5);

    assert_eq!(
        4_294_967_295_i64.frog_to_var_buf::<Vec<u8>>().unwrap(),
        vec![255, 255, 255, 255, 15]
    );
    assert_eq!(4_294_967_295_i64.frog_var_len(), 5);

    assert_eq!(
        9_223_372_036_854_775_807_i64.frog_to_var_buf::<Vec<u8>>().unwrap(),
        vec![255, 255, 255, 255, 255, 255, 255, 255, 127]
    );
    assert_eq!(9_223_372_036_854_775_807_i64.frog_var_len(), 9);
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(ProptestConfig::with_cases(1024))]

    #[test]
    fn proto_varint_u16(data in proptest::num::u16::ANY) {
        let buffer = data.frog_to_var_buf::<Vec<u8>>().unwrap();
        assert_eq!(u16::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(buffer.len(), data.frog_var_len());
    }
    #[test]
    fn proto_varint_i16(data in proptest::num::i16::ANY) {
        let buffer = data.frog_to_var_buf::<Vec<u8>>().unwrap();
        assert_eq!(i16::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(buffer.len(), data.frog_var_len());
    }

    #[test]
    fn proto_varint_u32(data in proptest::num::u32::ANY) {
        let buffer = data.frog_to_var_buf::<Vec<u8>>().unwrap();
        assert_eq!(u32::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(buffer.len(), data.frog_var_len());
    }

    #[test]
    fn proto_varint_i32(data in proptest::num::i32::ANY) {
        let buffer = data.frog_to_var_buf::<Vec<u8>>().unwrap();
        assert_eq!(i32::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(buffer.len(), data.frog_var_len());
    }

    #[test]
    fn proto_varint_usize(data in proptest::num::usize::ANY) {
        let buffer = data.frog_to_var_buf::<Vec<u8>>().unwrap();
        assert_eq!(usize::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), match data {
            data if data > u32::MAX as usize => u32::MAX as usize,
            data => data,
        });
        assert_eq!(buffer.len(), data.frog_var_len());
    }

    #[test]
    fn proto_varint_isize(data in proptest::num::isize::ANY) {
        let buffer = data.frog_to_var_buf::<Vec<u8>>().unwrap();
        assert_eq!(isize::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), match data {
            data if data < i32::MIN as isize => i32::MIN as isize,
            data if data > i32::MAX as isize => i32::MAX as isize,
            data => data,
        });
        assert_eq!(buffer.len(), data.frog_var_len());
    }

    #[test]
    fn proto_varint_u64(data in proptest::num::u64::ANY) {
        let buffer = data.frog_to_var_buf::<Vec<u8>>().unwrap();
        assert_eq!(u64::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(buffer.len(), data.frog_var_len());
    }

    #[test]
    fn proto_varint_i64(data in proptest::num::i64::ANY) {
        let buffer = data.frog_to_var_buf::<Vec<u8>>().unwrap();
        assert_eq!(i64::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(buffer.len(), data.frog_var_len());
    }

    #[test]
    fn proto_varint_u128(data in proptest::num::u128::ANY) {
        let buffer = data.frog_to_var_buf::<Vec<u8>>().unwrap();
        assert_eq!(u128::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(buffer.len(), data.frog_var_len());
    }

    #[test]
    fn proto_varint_i128(data in proptest::num::i128::ANY) {
        let buffer = data.frog_to_var_buf::<Vec<u8>>().unwrap();
        assert_eq!(i128::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(buffer.len(), data.frog_var_len());
    }
}

impl<T: FrogVarRead> FrogVarRead for Vec<T> {
    fn frog_var_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        (0..usize::frog_var_read(buffer)?).map(|_| T::frog_var_read(buffer)).collect()
    }
}
impl<T: FrogVarWrite> FrogVarWrite for Vec<T> {
    #[inline]
    fn frog_var_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        <[T]>::frog_var_write(self, buffer)
    }
    #[inline]
    fn frog_var_len(&self) -> usize { <[T]>::frog_var_len(self) }
}
impl<T: FrogVarWrite> FrogVarWrite for [T] {
    fn frog_var_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        self.iter().try_fold(self.len().frog_var_write(buffer)?, |acc, item| {
            item.frog_var_write(buffer).map(|len| acc + len)
        })
    }
    fn frog_var_len(&self) -> usize {
        self.iter().fold(self.len().frog_var_len(), |acc, item| acc + item.frog_var_len())
    }
}

impl<T: FrogVarRead, const N: usize> FrogVarRead for [T; N] {
    fn frog_var_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        std::array::try_from_fn(|_| T::frog_var_read(buffer))
    }
}
impl<T: FrogVarWrite, const N: usize> FrogVarWrite for [T; N] {
    fn frog_var_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        self.iter().try_fold(0, |acc, item| item.frog_var_write(buffer).map(|len| acc + len))
    }
    fn frog_var_len(&self) -> usize { self.iter().fold(0, |acc, item| acc + item.frog_var_len()) }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn proto_vec_varint_u16(data in proptest::collection::vec(proptest::num::u16::ANY, 0..128)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(Vec::<u16>::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }

    #[test]
    fn proto_vec_varint_u32(data in proptest::collection::vec(proptest::num::u32::ANY, 0..256)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(Vec::<u32>::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }

    #[test]
    fn proto_vec_varint_u64(data in proptest::collection::vec(proptest::num::u64::ANY, 0..256)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(Vec::<u64>::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }

    #[test]
    fn proto_vec_varint_u128(data in proptest::collection::vec(proptest::num::u128::ANY, 0..256)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(Vec::<u128>::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }
}

impl<K: FrogRead + Eq + Hash, V: FrogVarRead, S: BuildHasher + Default> FrogVarRead
    for HashMap<K, V, S>
{
    fn frog_var_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        (0..usize::frog_var_read(buffer)?)
            .map(|_| Ok((K::frog_read(buffer)?, V::frog_var_read(buffer)?)))
            .collect()
    }
}
impl<K: FrogWrite, V: FrogVarWrite, S> FrogVarWrite for HashMap<K, V, S> {
    fn frog_var_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        self.iter().try_fold(self.len().frog_var_write(buffer)?, |acc, (key, value)| {
            key.frog_write(buffer)?;
            value.frog_var_write(buffer).map(|len| acc + len)
        })
    }

    fn frog_var_len(&self) -> usize {
        self.iter().fold(self.len().frog_var_len(), |acc, (key, value)| {
            acc + key.frog_len() + value.frog_var_len()
        })
    }
}

impl<T: FrogVarRead + Eq + Hash, S: BuildHasher + Default> FrogVarRead for HashSet<T, S> {
    fn frog_var_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        (0..usize::frog_var_read(buffer)?).map(|_| T::frog_var_read(buffer)).collect()
    }
}
impl<T: FrogVarWrite + Eq + Hash, S: BuildHasher> FrogVarWrite for HashSet<T, S> {
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
    fn proto_hashmap_u32_varint_u32(data in proptest::collection::hash_map(proptest::num::u32::ANY, proptest::num::u32::ANY, 0..256)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(HashMap::<u32, u32>::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }
    #[test]
    fn proto_hashset_varint_u32(data in proptest::collection::hash_set(proptest::num::u32::ANY, 0..256)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(HashSet::<u32>::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }

    #[test]
    fn proto_hashmap_u64_varint_u64(data in proptest::collection::hash_map(proptest::num::u64::ANY, proptest::num::u64::ANY, 0..256)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(HashMap::<u64, u64>::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }
    #[test]
    fn proto_hashset_varint_u64(data in proptest::collection::hash_set(proptest::num::u64::ANY, 0..256)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(HashSet::<u64>::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }
}

impl<T: FrogVarRead> FrogVarRead for Option<T> {
    fn frog_var_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        if bool::frog_read(buffer)? { T::frog_var_read(buffer).map(Some) } else { Ok(None) }
    }
}
impl<T: FrogVarWrite> FrogVarWrite for Option<T> {
    fn frog_var_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        if let Some(value) = self {
            let prefix = bool::frog_write(&true, buffer)?;
            value.frog_var_write(buffer).map(|len| prefix + len)
        } else {
            bool::frog_write(&false, buffer)
        }
    }

    fn frog_var_len(&self) -> usize {
        if let Some(value) = self {
            bool::frog_len(&true) + value.frog_var_len()
        } else {
            bool::frog_len(&false)
        }
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn proto_option_varint_u32(data in proptest::option::of(proptest::num::u32::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(Option::<u32>::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }
    #[test]
    fn proto_option_varint_u64(data in proptest::option::of(proptest::num::u64::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(Option::<u64>::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }

    #[test]
    fn proto_option_option_varint_u16(data in proptest::option::of(proptest::option::of(proptest::num::u16::ANY))) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(Option::<Option<u16>>::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }
    #[test]
    fn proto_option_vec_varint_u128(data in proptest::option::of(proptest::collection::vec(proptest::num::u128::ANY, 0..256))) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(Option::<Vec<u128>>::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }
}
