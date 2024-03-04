use smallvec::SmallVec;

use crate::io::{FrogVarWrite, FrogWrite, WriteError};

impl<T: FrogWrite, const N: usize> FrogWrite for [T; N] {
    #[inline]
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        for value in self {
            value.fg_write(buf)?;
        }
        Ok(())
    }
}

impl<T: FrogWrite> FrogWrite for Vec<T> {
    #[inline]
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        u32::try_from(self.len()).expect("Vector length too long").fg_var_write(buf)?;
        for value in self {
            value.fg_write(buf)?;
        }
        Ok(())
    }
}

impl<T: FrogWrite, const N: usize> FrogWrite for SmallVec<[T; N]> {
    #[inline]
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        u32::try_from(self.len()).expect("Vector length too long").fg_var_write(buf)?;
        for value in self {
            value.fg_write(buf)?;
        }
        Ok(())
    }
}

#[cfg(test)]
proptest::proptest! {

    #[test]
    fn proto_write_vec_u8(data in proptest::collection::vec(proptest::num::u8::ANY, 0..1024)) {
        use crate::io::FrogVarWrite;

        let mut bytes = Vec::with_capacity(data.len() + 2);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut bytes).unwrap();
        bytes.extend(&data);

        assert_eq!(data.fg_to_bytes(), bytes);
    }

    #[test]
    fn proto_write_vec_u16(data in proptest::collection::vec(proptest::num::u16::ANY, 0..512)) {
        use crate::io::FrogVarWrite;

        let mut bytes = Vec::with_capacity(data.len() * 2 + 2);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut bytes).unwrap();
        for &value in &data {
            value.fg_write(&mut bytes).unwrap();
        }

        assert_eq!(data.fg_to_bytes(), bytes);
    }

    #[test]
    fn proto_write_vec_i16(data in proptest::collection::vec(proptest::num::i16::ANY, 0..512)) {
        use crate::io::FrogVarWrite;

        let mut bytes = Vec::with_capacity(data.len() * 2 + 2);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut bytes).unwrap();
        for &value in &data {
            value.fg_write(&mut bytes).unwrap();
        }

        assert_eq!(data.fg_to_bytes(), bytes);
    }

    #[test]
    fn proto_write_smallvec_u8(data in proptest::collection::vec(proptest::num::u8::ANY, 0..1024)) {
        use crate::io::FrogVarWrite;

        let mut bytes = Vec::with_capacity(data.len() + 2);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut bytes).unwrap();
        bytes.extend(&data);

        assert_eq!(SmallVec::<[u8; 8]>::from_vec(data).fg_to_bytes(), bytes);
    }

    #[test]
    fn proto_write_array4_u8(data in proptest::array::uniform4(proptest::num::u8::ANY)) {
        assert_eq!(data.fg_to_bytes(), data.to_vec());
    }
}
