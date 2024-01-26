use simdnbt::owned::Nbt;
use uuid::Uuid;

use super::{FrogRead, ReadError};

impl FrogRead for Nbt {
    #[inline]
    fn frog_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        Nbt::read(buf).map_err(ReadError::from)
    }
}

// TODO: Test nbt read
// #[test]
// fn proto_read_nbt() {}

impl FrogRead for Uuid {
    #[inline]
    fn frog_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        Ok(Uuid::from_u128(u128::frog_read(buf)?))
    }
}

#[test]
fn proto_read_uuid() {
    use std::str::FromStr;

    let mut cursor =
        std::io::Cursor::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].as_slice());
    let uuid: Uuid = Uuid::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);
    assert_eq!(uuid, Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());

    let mut cursor =
        std::io::Cursor::new([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].as_slice());
    let uuid: Uuid = Uuid::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);
    assert_eq!(uuid, Uuid::from_str("01000000-0000-0000-0000-000000000000").unwrap());

    let mut cursor =
        std::io::Cursor::new([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1].as_slice());
    let uuid: Uuid = Uuid::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);
    assert_eq!(uuid, Uuid::from_str("01000000-0000-0000-0000-000000000001").unwrap());

    let mut cursor =
        std::io::Cursor::new([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2].as_slice());
    let uuid: Uuid = Uuid::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);
    assert_eq!(uuid, Uuid::from_str("01000000-0000-0000-0000-000000000002").unwrap());

    let mut cursor =
        std::io::Cursor::new([3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3].as_slice());
    let uuid: Uuid = Uuid::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);
    assert_eq!(uuid, Uuid::from_str("03000000-0000-0000-0000-000000000003").unwrap());
}
