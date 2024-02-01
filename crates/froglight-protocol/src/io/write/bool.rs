use super::{FrogWrite, WriteError};

impl FrogWrite for bool {
    #[inline]
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        u8::from(*self).fg_write(buf)
    }
}

#[test]
fn proto_write_bool() {
    let mut buf = Vec::new();

    true.fg_write(&mut buf).unwrap();
    true.fg_write(&mut buf).unwrap();
    false.fg_write(&mut buf).unwrap();
    false.fg_write(&mut buf).unwrap();

    assert_eq!(buf, vec![1, 1, 0, 0]);
}
