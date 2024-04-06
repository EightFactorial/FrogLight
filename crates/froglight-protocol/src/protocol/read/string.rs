use std::io::BufRead;

use compact_str::CompactString;

use crate::protocol::{FrogRead, FrogVarRead};

impl FrogRead for String {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::protocol::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::fg_var_read(buf)?).expect("String too long");
        let position = usize::try_from(buf.position()).expect("Cursor position too large");

        if let Some(slice) = buf.get_ref().get(position..position + len) {
            let str = std::str::from_utf8(slice)?;
            buf.consume(len);

            Ok(String::from(str))
        } else {
            let leftover = buf.get_ref().len() - position;
            Err(crate::protocol::ReadError::EndOfBuffer(len, leftover))
        }
    }
}

impl FrogRead for CompactString {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::protocol::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::fg_var_read(buf)?).expect("String too long");
        let position = usize::try_from(buf.position()).expect("Cursor position too large");

        if let Some(slice) = buf.get_ref().get(position..position + len) {
            let str = std::str::from_utf8(slice)?;
            buf.consume(len);

            Ok(CompactString::from(str))
        } else {
            let leftover = buf.get_ref().len() - position;
            Err(crate::protocol::ReadError::EndOfBuffer(len, leftover))
        }
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(128))]

    #[test]
    fn proto_read_string(data in ".*") {
        use crate::protocol::var_write::FrogVarWrite;

        // Convert the data to bytes
        let mut vec = Vec::new();
        let len = u32::try_from(data.len()).unwrap();
        len.fg_var_write(&mut vec).unwrap();
        vec.extend_from_slice(data.as_bytes());

        // Read the data back
        let mut cursor = std::io::Cursor::new(vec.as_slice());
        let read = String::fg_read(&mut cursor).unwrap();

        assert_eq!(data, read);
        assert_eq!(cursor.position(), vec.len() as u64);
    }

    #[test]
    fn proto_read_compact_string(data in ".*") {
        use crate::protocol::var_write::FrogVarWrite;

        // Convert the data to bytes
        let mut vec = Vec::new();
        let len = u32::try_from(data.len()).unwrap();
        len.fg_var_write(&mut vec).unwrap();
        vec.extend_from_slice(data.as_bytes());

        // Read the data back
        let mut cursor = std::io::Cursor::new(vec.as_slice());
        let read = CompactString::fg_read(&mut cursor).unwrap();

        assert_eq!(data, read);
        assert_eq!(cursor.position(), vec.len() as u64);
    }
}
