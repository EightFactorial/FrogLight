use super::{FrogRead, ReadError};

macro_rules! impl_float_read {
    ($ty1:ty, $ty2:ty) => {
        impl FrogRead for $ty1 {
            #[inline]
            fn frog_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError> {
                Ok(Self::from_bits(<$ty2>::frog_read(buf)?))
            }
        }
    };
}

impl_float_read!(f32, u32);
impl_float_read!(f64, u64);

#[test]
fn proto_read_f32() {
    let buf = [0x3f, 0x80, 0x00, 0x00];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(f32::frog_read(&mut cursor).unwrap().to_be_bytes(), buf);
    assert_eq!(cursor.position(), buf.len() as u64);
}
#[test]
fn proto_read_f64() {
    let buf = [0x3f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(f64::frog_read(&mut cursor).unwrap().to_be_bytes(), buf);
    assert_eq!(cursor.position(), buf.len() as u64);
}
