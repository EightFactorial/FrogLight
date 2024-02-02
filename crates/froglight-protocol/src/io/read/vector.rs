use smallvec::SmallVec;

use crate::io::{FrogRead, FrogVarRead, ReadError};

impl<T: FrogRead, const N: usize> FrogRead for [T; N] {
    #[inline]
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        core::array::try_from_fn(|_| T::fg_read(buf))
    }
}

impl<T: FrogRead> FrogRead for Vec<T> {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        let len = u32::fg_var_read(buf)?;
        let mut vec = Vec::with_capacity(len as usize);

        for i in 0..len {
            match T::fg_read(buf) {
                Ok(value) => vec.push(value),
                Err(err) => {
                    return Err(ReadError::ListError(len as usize, i as usize, Box::new(err)));
                }
            }
        }

        Ok(vec)
    }
}

impl<T: FrogRead, const N: usize> FrogRead for SmallVec<[T; N]> {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        let len = u32::fg_var_read(buf)?;
        let mut vec = SmallVec::with_capacity(len as usize);

        for i in 0..len {
            match T::fg_read(buf) {
                Ok(value) => vec.push(value),
                Err(err) => {
                    return Err(ReadError::ListError(len as usize, i as usize, Box::new(err)));
                }
            }
        }

        Ok(vec)
    }
}

#[test]
fn proto_read_array() {
    let mut cursor = std::io::Cursor::new([0, 1, 0, 1, 0, 0, 1, 1u8].as_slice());

    assert_eq!(<[u8; 4]>::fg_read(&mut cursor).unwrap(), [0, 1, 0, 1]);
    assert_eq!(<[u8; 4]>::fg_read(&mut cursor).unwrap(), [0, 0, 1, 1]);

    let err = <[u8; 4]>::fg_read(&mut cursor).unwrap_err();
    assert!(matches!(err, ReadError::EndOfBuffer(1, 0)));
}
#[test]
fn proto_read_vector() {
    let mut cursor =
        std::io::Cursor::new([2, 1, 1, 2, 0, 8, 0, 8, 2, 0, 0, 1, 0, 0, 0, 1, 0].as_slice());

    let vec: Vec<u8> = Vec::fg_read(&mut cursor).unwrap();
    assert_eq!(vec, [1, 1]);

    let vec: Vec<u16> = Vec::fg_read(&mut cursor).unwrap();
    assert_eq!(vec, [8, 8]);

    let vec: Vec<u32> = Vec::fg_read(&mut cursor).unwrap();
    assert_eq!(vec, [256, 256]);
}
#[test]
fn proto_read_smallvec() {
    let mut cursor =
        std::io::Cursor::new([2, 1, 1, 2, 0, 8, 0, 8, 2, 0, 0, 1, 0, 0, 0, 1, 0].as_slice());

    let vec: SmallVec<[u8; 4]> = SmallVec::fg_read(&mut cursor).unwrap();
    assert_eq!(vec, [1, 1].into());

    let vec: SmallVec<[u16; 4]> = SmallVec::fg_read(&mut cursor).unwrap();
    assert_eq!(vec, [8, 8].into());

    let vec: SmallVec<[u32; 4]> = SmallVec::fg_read(&mut cursor).unwrap();
    assert_eq!(vec, [256, 256].into());
}
