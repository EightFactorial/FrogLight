#[cfg(test)]
use std::io::Cursor;
use std::io::{Read, Write};

#[cfg(test)]
use proptest::prelude::*;
use smol_str::SmolStr;

use super::{FrogRead, FrogWrite, ReadError, WriteError};

impl FrogRead for SmolStr {
    #[inline]
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        String::frog_read(buffer).map(SmolStr::from)
    }
}

impl FrogWrite for SmolStr {
    #[inline]
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        self.as_str().frog_write(buffer)
    }

    #[inline]
    fn frog_len(&self) -> usize { self.as_str().frog_len() }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn proto_smolstr(data in ".*") {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(SmolStr::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_vec_smolstr(data in proptest::collection::vec(".*", 0..32)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec::<SmolStr>::frog_read(&mut Cursor::new(&buffer)).unwrap(), data);
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_option_smolstr(data in proptest::option::of(".*")) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Option::<SmolStr>::frog_read(&mut Cursor::new(&buffer)).unwrap().as_deref(), data.as_deref());
        assert_eq!(data.frog_len(), buffer.len());
    }
}
