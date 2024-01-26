use super::{FrogWrite, WriteError};

impl<T: FrogWrite> FrogWrite for Option<T> {
    fn frog_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        match self {
            Some(value) => {
                true.frog_write(buf)?;
                value.frog_write(buf)
            }
            None => false.frog_write(buf),
        }
    }
}

#[test]
fn proto_write_option() {
    let mut buf = Vec::new();

    Some(1u8).frog_write(&mut buf).unwrap();
    None::<u8>.frog_write(&mut buf).unwrap();
    Some(2u8).frog_write(&mut buf).unwrap();

    // [some, [1], none, some, [2]]
    assert_eq!(buf, vec![1, 1, 0, 1, 2]);
    buf.clear();

    Some(1u16).frog_write(&mut buf).unwrap();
    None::<u16>.frog_write(&mut buf).unwrap();
    Some(2u16).frog_write(&mut buf).unwrap();

    // [some, [0, 1], none, some, [0, 2]]
    assert_eq!(buf, vec![1, 0, 1, 0, 1, 0, 2]);
}
