use simdnbt::owned::Nbt;
use uuid::Uuid;

use super::{FrogWrite, WriteError};

impl FrogWrite for Nbt {
    #[inline]
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        let mut vec = Vec::new();
        self.write(&mut vec);

        Ok(buf.write_all(&vec)?)
    }
}

// TODO: Write NBT tests
// #[test]
// fn proto_write_nbt() {}

impl FrogWrite for Uuid {
    #[inline]
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        self.as_u128().fg_write(buf)
    }
}

#[test]
fn proto_write_uuid() {
    use std::str::FromStr;

    let mut buf = Vec::new();

    Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap().fg_write(&mut buf).unwrap();
    assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    buf.clear();

    Uuid::from_str("01000000-0000-0000-0000-000000000000").unwrap().fg_write(&mut buf).unwrap();
    assert_eq!(buf, [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    buf.clear();

    Uuid::from_str("01000000-0000-0000-0000-000000000001").unwrap().fg_write(&mut buf).unwrap();
    assert_eq!(buf, [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    buf.clear();

    Uuid::from_str("01000000-0000-0000-0000-000000000002").unwrap().fg_write(&mut buf).unwrap();
    assert_eq!(buf, [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);
    buf.clear();

    Uuid::from_str("01000000-0000-0000-0000-000000000003").unwrap().fg_write(&mut buf).unwrap();
    assert_eq!(buf, [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3]);
    buf.clear();
}
