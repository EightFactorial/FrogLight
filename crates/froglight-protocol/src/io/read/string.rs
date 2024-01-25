use std::io::BufRead;

use compact_str::CompactString;

use crate::io::{FrogRead, FrogVarRead};

impl FrogRead for String {
    fn frog_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::frog_var_read(buf)?).expect("String too long");
        let position = usize::try_from(buf.position()).expect("Cursor position too large");

        if let Some(slice) = buf.get_ref().get(position..position + len) {
            let str = std::str::from_utf8(slice)?;
            buf.consume(len);

            Ok(String::from(str))
        } else {
            let leftover = buf.get_ref().len() - position;
            Err(crate::io::ReadError::EndOfBuffer(len, leftover))
        }
    }
}

impl FrogRead for CompactString {
    fn frog_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::frog_var_read(buf)?).expect("String too long");
        let position = usize::try_from(buf.position()).expect("Cursor position too large");

        if let Some(slice) = buf.get_ref().get(position..position + len) {
            let str = std::str::from_utf8(slice)?;
            buf.consume(len);

            Ok(CompactString::from(str))
        } else {
            let leftover = buf.get_ref().len() - position;
            Err(crate::io::ReadError::EndOfBuffer(len, leftover))
        }
    }
}

#[test]
fn proto_read_string() {
    let mut cursor = std::io::Cursor::new([0].as_slice());
    let string: String = String::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);
    assert_eq!(string, "");

    let mut cursor = std::io::Cursor::new([1, 65].as_slice());
    let string: String = String::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);
    assert_eq!(string, "A");

    let mut cursor = std::io::Cursor::new([2, 65, 66].as_slice());
    let string: String = String::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);
    assert_eq!(string, "AB");

    let mut cursor = std::io::Cursor::new(
        [12, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100].as_slice(),
    );
    let string: String = String::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);
    assert_eq!(string, "Hello, world");
}

#[test]
fn proto_read_compactstring() {
    let mut cursor = std::io::Cursor::new([0].as_slice());
    let string: CompactString = CompactString::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);
    assert_eq!(string, "");

    let mut cursor = std::io::Cursor::new([1, 65].as_slice());
    let string: CompactString = CompactString::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);
    assert_eq!(string, "A");

    let mut cursor = std::io::Cursor::new([2, 65, 66].as_slice());
    let string: CompactString = CompactString::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);
    assert_eq!(string, "AB");

    let mut cursor = std::io::Cursor::new(
        [12, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100].as_slice(),
    );
    let string: CompactString = CompactString::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);
    assert_eq!(string, "Hello, world");
}
