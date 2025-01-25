#[cfg(test)]
use std::io::Cursor;
use std::io::{Read, Write};

#[cfg(test)]
use proptest::prelude::*;
use smallvec::{Array, SmallVec};

use super::{FrogVarRead, FrogVarWrite};
use crate::standard::{ReadError, WriteError};

impl<A: Array> FrogVarRead for SmallVec<A>
where
    A::Item: FrogVarRead,
{
    #[inline]
    fn frog_var_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        Vec::<A::Item>::frog_var_read(buffer).map(SmallVec::from_vec)
    }
}

impl<A: Array> FrogVarWrite for SmallVec<A>
where
    A::Item: FrogVarWrite,
{
    #[inline]
    fn frog_var_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        <[A::Item]>::frog_var_write(self.as_slice(), buffer)
    }

    #[inline]
    fn frog_var_len(&self) -> usize { <[A::Item]>::frog_var_len(self.as_slice()) }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn proto_smallvec_varint_u16(data in proptest::collection::vec(proptest::num::u16::ANY, 0..128)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(SmallVec::<[u16; 4]>::frog_var_read(&mut Cursor::new(&buffer)).unwrap().as_slice(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }

    #[test]
    fn proto_smallvec_varint_u32(data in proptest::collection::vec(proptest::num::u32::ANY, 0..256)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(SmallVec::<[u32; 2]>::frog_var_read(&mut Cursor::new(&buffer)).unwrap().as_slice(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }

    #[test]
    fn proto_smallvec_varint_u64(data in proptest::collection::vec(proptest::num::u64::ANY, 0..256)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(SmallVec::<[u64; 8]>::frog_var_read(&mut Cursor::new(&buffer)).unwrap().as_slice(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }

    #[test]
    fn proto_smallvec_varint_u128(data in proptest::collection::vec(proptest::num::u128::ANY, 0..256)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(SmallVec::<[u128; 1]>::frog_var_read(&mut Cursor::new(&buffer)).unwrap().as_slice(), data);
        assert_eq!(data.frog_var_len(), buffer.len());
    }
}
