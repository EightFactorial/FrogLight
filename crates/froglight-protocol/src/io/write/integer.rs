use super::{FrogWrite, WriteError};

macro_rules! impl_integer_write {
    ($ty:ty) => {
        impl FrogWrite for $ty {
            #[inline]
            fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
                Ok(buf.write_all(bytemuck::bytes_of(&self.to_be()))?)
            }
        }
    };
}

impl_integer_write!(u8);
impl_integer_write!(u16);
impl_integer_write!(u32);
impl_integer_write!(u64);
impl_integer_write!(u128);

#[test]
fn proto_write_u8() {
    let mut buf = Vec::new();

    assert!(0u8.fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(255u8.fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255]);
}
#[test]
fn proto_write_u16() {
    let mut buf = Vec::new();

    assert!(0u16.fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0]);
    buf.clear();

    assert!(65535u16.fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255]);
}
#[test]
fn proto_write_u32() {
    let mut buf = Vec::new();

    assert!(0u32.fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0]);
    buf.clear();

    assert!(4_294_967_295_u32.fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255]);
}
#[test]
fn proto_write_u64() {
    let mut buf = Vec::new();

    assert!(0u64.fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0, 0, 0, 0, 0]);
    buf.clear();

    assert!(18_446_744_073_709_551_615_u64.fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 255, 255, 255, 255]);
}
#[test]
fn proto_write_u128() {
    let mut buf = Vec::new();

    assert!(0u128.fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![0; 16]);
    buf.clear();

    assert!(340_282_366_920_938_463_463_374_607_431_768_211_455_u128.fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255; 16]);
    buf.clear();
}

impl_integer_write!(i8);
impl_integer_write!(i16);
impl_integer_write!(i32);
impl_integer_write!(i64);
impl_integer_write!(i128);

#[test]
fn proto_write_i16() {
    let mut buf = Vec::new();

    assert!(0i16.fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0]);
    buf.clear();

    assert!(32767i16.fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![127, 255]);
    buf.clear();

    assert!((-32768i16).fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 0]);
}
#[test]
fn proto_write_i32() {
    let mut buf = Vec::new();

    assert!(0i32.fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0]);
    buf.clear();

    assert!(2_147_483_647_i32.fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![127, 255, 255, 255]);
    buf.clear();

    assert!((-2_147_483_648_i32).fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 0, 0, 0]);
}
#[test]
fn proto_write_i64() {
    let mut buf = Vec::new();

    assert!(0i64.fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0, 0, 0, 0, 0]);
    buf.clear();

    assert!(9_223_372_036_854_775_807_i64.fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![127, 255, 255, 255, 255, 255, 255, 255]);
    buf.clear();

    assert!((-9_223_372_036_854_775_808_i64).fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 0, 0, 0, 0, 0, 0, 0]);
}
#[test]
fn proto_write_i128() {
    let mut buf = Vec::new();

    assert!(0i128.fg_write(&mut buf).is_ok());
    assert_eq!(buf, vec![0; 16]);
    buf.clear();

    assert!(170_141_183_460_469_231_731_687_303_715_884_105_727_i128.fg_write(&mut buf).is_ok());
    assert_eq!(
        buf,
        vec![127, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
    );
    buf.clear();
}
