use super::{FrogRead, ReadError};

impl<T: FrogRead> FrogRead for Option<T> {
    #[inline]
    fn frog_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        if bool::frog_read(buf)? {
            Ok(Some(T::frog_read(buf)?))
        } else {
            Ok(None)
        }
    }
}

#[test]
fn proto_read_option() {
    let mut cursor = std::io::Cursor::new([0, 0, 1, 0, 1, 1].as_slice());

    assert_eq!(Option::<bool>::frog_read(&mut cursor).unwrap(), None);
    assert_eq!(Option::<bool>::frog_read(&mut cursor).unwrap(), None);
    assert_eq!(Option::<bool>::frog_read(&mut cursor).unwrap(), Some(false));
    assert_eq!(Option::<bool>::frog_read(&mut cursor).unwrap(), Some(true));

    let mut cursor = std::io::Cursor::new([0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1].as_slice());

    assert_eq!(Option::<u32>::frog_read(&mut cursor).unwrap(), None);
    assert_eq!(Option::<u32>::frog_read(&mut cursor).unwrap(), Some(0));
    assert_eq!(Option::<u32>::frog_read(&mut cursor).unwrap(), Some(1));
}
