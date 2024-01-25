use super::{FrogRead, ReadError};

impl FrogRead for bool {
    #[inline]
    fn frog_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        match u8::frog_read(buf)? {
            0 => Ok(false),
            1 => Ok(true),
            o => Err(ReadError::InvalidBool(o)),
        }
    }
}

#[test]
fn proto_read_bool() {
    let mut cursor = std::io::Cursor::new([0, 1, 0, 1, 0, 0, 1, 1, 2].as_slice());

    assert!(!bool::frog_read(&mut cursor).unwrap());
    assert!(bool::frog_read(&mut cursor).unwrap());
    assert!(!bool::frog_read(&mut cursor).unwrap());
    assert!(bool::frog_read(&mut cursor).unwrap());
    assert!(!bool::frog_read(&mut cursor).unwrap());
    assert!(!bool::frog_read(&mut cursor).unwrap());
    assert!(bool::frog_read(&mut cursor).unwrap());
    assert!(bool::frog_read(&mut cursor).unwrap());

    let err = bool::frog_read(&mut cursor).unwrap_err();
    assert!(matches!(err, ReadError::InvalidBool(2)));
}
