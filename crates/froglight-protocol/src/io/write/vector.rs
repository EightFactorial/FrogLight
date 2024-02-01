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

#[test]
fn proto_write_array() {
    let mut buf = Vec::new();

    [0, 1, 0, 1u8].fg_write(&mut buf).unwrap();
    [0, 0, 1, 1u8].fg_write(&mut buf).unwrap();

    assert_eq!(buf, [0, 1, 0, 1, 0, 0, 1, 1]);
}
#[test]
fn proto_write_vector() {
    let mut buf = Vec::new();

    vec![8u8, 8, 2, 1, 1].fg_write(&mut buf).unwrap();

    assert_eq!(buf, [5, 8, 8, 2, 1, 1]);
}
#[test]
fn proto_write_smallvec() {
    let mut buf = Vec::new();

    SmallVec::<[u8; 4]>::from_vec(vec![8u8, 8, 2, 1, 1]).fg_write(&mut buf).unwrap();

    assert_eq!(buf, [5, 8, 8, 2, 1, 1]);
}
