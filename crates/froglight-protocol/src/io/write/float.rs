use super::FrogWrite;

macro_rules! impl_float_write {
    ($ty1:ty, $ty2:ty) => {
        impl FrogWrite for $ty1 {
            #[inline]
            fn frog_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
                <$ty2>::frog_write(&self.to_bits(), buf)
            }
        }
    };
}

impl_float_write!(f32, u32);
impl_float_write!(f64, u64);

#[test]
fn proto_write_f32() {
    let mut buf = Vec::new();

    assert!(0f32.frog_write(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0]);
    buf.clear();

    assert!(1.0f32.frog_write(&mut buf).is_ok());
    assert_eq!(buf, vec![63, 128, 0, 0]);
    buf.clear();

    assert!(1.5f32.frog_write(&mut buf).is_ok());
    assert_eq!(buf, vec![63, 192, 0, 0]);
    buf.clear();
}
#[test]
fn proto_write_f64() {
    let mut buf = Vec::new();

    assert!(0f64.frog_write(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0, 0, 0, 0, 0]);
    buf.clear();

    assert!(1.0f64.frog_write(&mut buf).is_ok());
    assert_eq!(buf, vec![63, 240, 0, 0, 0, 0, 0, 0]);
    buf.clear();

    assert!(1.5f64.frog_write(&mut buf).is_ok());
    assert_eq!(buf, vec![63, 248, 0, 0, 0, 0, 0, 0]);
    buf.clear();
}
