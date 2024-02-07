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

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(128))]

    #[test]
    fn proto_read_array2(data in proptest::array::uniform2(0u8..=255u8)) {
        let mut cursor = std::io::Cursor::new(data.as_slice());
        let result: Result<[u8; 2], _> = <[u8; 2]>::fg_read(&mut cursor);

        assert_eq!(result.unwrap(), data);
        assert_eq!(cursor.position(), 2);
    }

    #[test]
    fn proto_read_array5(data in proptest::array::uniform5(0u8..=255u8)) {
        let mut cursor = std::io::Cursor::new(data.as_slice());
        let result: Result<[u8; 5], _> = <[u8; 5]>::fg_read(&mut cursor);

        assert_eq!(result.unwrap(), data);
        assert_eq!(cursor.position(), 5);
    }

    #[test]
    fn proto_read_array10(data in proptest::array::uniform10(0u8..=255u8)) {
        let mut cursor = std::io::Cursor::new(data.as_slice());
        let result: Result<[u8; 10], _> = <[u8; 10]>::fg_read(&mut cursor);

        assert_eq!(result.unwrap(), data);
        assert_eq!(cursor.position(), 10);
    }

    #[test]
    fn proto_read_array20(data in proptest::array::uniform20(0u8..=255u8)) {
        let mut cursor = std::io::Cursor::new(data.as_slice());
        let result: Result<[u8; 20], _> = <[u8; 20]>::fg_read(&mut cursor);

        assert_eq!(result.unwrap(), data);
        assert_eq!(cursor.position(), 20);
    }

    #[test]
    fn proto_read_vector(data in proptest::collection::vec(0u8..=255u8, 0..128)) {
        use crate::io::var_write::FrogVarWrite;

        // Prefix the data with the length
        let mut vec = Vec::new();
        let len = u32::try_from(data.len()).unwrap();
        len.fg_var_write(&mut vec).unwrap();
        vec.extend_from_slice(&data);

        // Read the data back
        let mut cursor = std::io::Cursor::new(vec.as_slice());
        let result: Result<Vec<u8>, _> = Vec::fg_read(&mut cursor);

        assert_eq!(result.unwrap(), data);
        assert_eq!(cursor.position(), vec.len() as u64);
    }

    #[test]
    fn proto_read_smallvec(data in proptest::collection::vec(0u8..=255u8, 0..128)) {
        use crate::io::var_write::FrogVarWrite;

        // Prefix the data with the length
        let mut vec = Vec::new();
        let len = u32::try_from(data.len()).unwrap();
        len.fg_var_write(&mut vec).unwrap();
        vec.extend_from_slice(&data);

        // Read the data back
        let mut cursor = std::io::Cursor::new(vec.as_slice());
        let result: Result<SmallVec<[u8; 4]>, _> = SmallVec::fg_read(&mut cursor);

        assert_eq!(result.unwrap().to_vec(), data);
        assert_eq!(cursor.position(), vec.len() as u64);
    }
}
