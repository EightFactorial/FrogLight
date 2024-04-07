use smallvec::SmallVec;

use crate::protocol::{FrogRead, FrogVarRead, ReadError};

impl<T: FrogRead, const N: usize> FrogRead for [T; N] {
    #[inline]
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        core::array::try_from_fn(|i| {
            T::fg_read(buf).map_err(|err| ReadError::ListError(N, i, Box::new(err)))
        })
    }
}

impl<T: FrogRead> FrogRead for Vec<T> {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        let len: usize = u32::fg_var_read(buf)? as usize;
        let mut vec = Vec::with_capacity(len);

        for i in 0..len {
            match T::fg_read(buf) {
                Ok(value) => vec.push(value),
                Err(err) => {
                    return Err(ReadError::ListError(len, i, Box::new(err)));
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
        let len: usize = u32::fg_var_read(buf)? as usize;
        let mut vec = SmallVec::with_capacity(len);

        for i in 0..len {
            match T::fg_read(buf) {
                Ok(value) => vec.push(value),
                Err(err) => {
                    return Err(ReadError::ListError(len, i, Box::new(err)));
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
    fn proto_read_array2_u8(data in proptest::array::uniform2(proptest::bits::u8::ANY)) {
        let mut cursor = std::io::Cursor::new(data.as_slice());
        let result: Result<[u8; 2], _> = <[u8; 2]>::fg_read(&mut cursor);

        assert_eq!(result.unwrap(), data);
        assert_eq!(cursor.position(), 2);
    }

    #[test]
    fn proto_read_array2_u32(data in proptest::array::uniform2(proptest::bits::u32::ANY)) {
        let data_bytes = bytemuck::cast_slice(&data);
        let mut cursor = std::io::Cursor::new(data_bytes);

        let result: Result<[u32; 2], _> = <[u32; 2]>::fg_read(&mut cursor);
        let result = result.unwrap().map(u32::from_be);

        assert_eq!(result, data);
        assert_eq!(cursor.position(), 8);
    }

    #[test]
    fn proto_read_array5_u8(data in proptest::array::uniform5(proptest::bits::u8::ANY)) {
        let mut cursor = std::io::Cursor::new(data.as_slice());
        let result: Result<[u8; 5], _> = <[u8; 5]>::fg_read(&mut cursor);

        assert_eq!(result.unwrap(), data);
        assert_eq!(cursor.position(), 5);
    }

    #[test]
    fn proto_read_array5_u16(data in proptest::array::uniform5(proptest::bits::u16::ANY)) {
        let data_bytes = bytemuck::cast_slice(&data);
        let mut cursor = std::io::Cursor::new(data_bytes);

        let result: Result<[u16; 5], _> = <[u16; 5]>::fg_read(&mut cursor);
        let result = result.unwrap().map(u16::from_be);

        assert_eq!(result, data);
        assert_eq!(cursor.position(), 10);
    }

    #[test]
    fn proto_read_array10_u8(data in proptest::array::uniform10(proptest::bits::u8::ANY)) {
        let mut cursor = std::io::Cursor::new(data.as_slice());
        let result: Result<[u8; 10], _> = <[u8; 10]>::fg_read(&mut cursor);

        assert_eq!(result.unwrap(), data);
        assert_eq!(cursor.position(), 10);
    }

    #[test]
    fn proto_read_array20_u8(data in proptest::array::uniform20(proptest::bits::u8::ANY)) {
        let mut cursor = std::io::Cursor::new(data.as_slice());
        let result: Result<[u8; 20], _> = <[u8; 20]>::fg_read(&mut cursor);

        assert_eq!(result.unwrap(), data);
        assert_eq!(cursor.position(), 20);
    }

    #[test]
    fn proto_read_vector_u8(data in proptest::collection::vec(proptest::bits::u8::ANY, 0..512)) {
        use crate::protocol::var_write::FrogVarWrite;

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
    fn proto_read_vector_string(data in proptest::collection::vec(".*", 0..64)) {
        use crate::protocol::var_write::FrogVarWrite;

        // Prefix the data with the length
        let mut vec = Vec::new();
        let len = u32::try_from(data.len()).unwrap();
        len.fg_var_write(&mut vec).unwrap();

        // Write the strings
        for string in &data {
            u32::try_from(string.len()).unwrap().fg_var_write(&mut vec).unwrap();
            vec.extend_from_slice(string.as_bytes());
        }

        // Read the data back
        let mut cursor = std::io::Cursor::new(vec.as_slice());
        let result: Result<Vec<String>, _> = Vec::fg_read(&mut cursor);

        assert_eq!(result.unwrap(), data);
        assert_eq!(cursor.position(), vec.len() as u64);
    }

    #[test]
    fn proto_read_smallvec_u8(data in proptest::collection::vec(proptest::bits::u8::ANY, 0..512)) {
        use crate::protocol::var_write::FrogVarWrite;

        // Prefix the data with the length
        let mut vec = Vec::new();
        let len = u32::try_from(data.len()).unwrap();
        len.fg_var_write(&mut vec).unwrap();
        vec.extend_from_slice(&data);

        // Read the data back
        let mut cursor = std::io::Cursor::new(vec.as_slice());
        let result: Result<SmallVec<[u8; 32]>, _> = SmallVec::fg_read(&mut cursor);

        assert_eq!(result.unwrap().to_vec(), data);
        assert_eq!(cursor.position(), vec.len() as u64);
    }

    #[test]
    fn proto_read_smallvec_string(data in proptest::collection::vec(".*", 0..64)) {
        use crate::protocol::var_write::FrogVarWrite;

        // Prefix the data with the length
        let mut vec = Vec::new();
        let len = u32::try_from(data.len()).unwrap();
        len.fg_var_write(&mut vec).unwrap();

        // Write the strings
        for string in &data {
            u32::try_from(string.len()).unwrap().fg_var_write(&mut vec).unwrap();
            vec.extend_from_slice(string.as_bytes());
        }

        // Read the data back
        let mut cursor = std::io::Cursor::new(vec.as_slice());
        let result: Result<SmallVec<[String; 32]>, _> = SmallVec::fg_read(&mut cursor);

        assert_eq!(result.unwrap().to_vec(), data);
        assert_eq!(cursor.position(), vec.len() as u64);
    }
}
