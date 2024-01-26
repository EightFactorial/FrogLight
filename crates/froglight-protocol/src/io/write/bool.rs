use super::{FrogWrite, WriteError};

impl FrogWrite for bool {
    #[inline]
    fn frog_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        u8::from(*self).frog_write(buf)
    }
}

#[test]
fn proto_write_bool() {
    let mut buf = Vec::new();

    true.frog_write(&mut buf).unwrap();
    true.frog_write(&mut buf).unwrap();
    false.frog_write(&mut buf).unwrap();
    false.frog_write(&mut buf).unwrap();

    assert_eq!(buf, vec![1, 1, 0, 0]);
}
