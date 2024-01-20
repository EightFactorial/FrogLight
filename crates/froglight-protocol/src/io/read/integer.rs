use super::FrogRead;

macro_rules! impl_integer_read {
    ($ty:ty) => {
        impl_integer_read!($ty, |slice| <$ty>::from_be(*bytemuck::from_bytes(slice)));
    };
    ($ty:ty, $expr:expr) => {
        impl FrogRead for $ty {
            #[inline]
            fn frog_read(buf: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
                let position = usize::try_from(buf.position()).expect("position too large");
                let length = (Self::BITS / 8) as usize;

                <std::io::Cursor<_> as std::io::BufRead>::consume(buf, length);
                let slice = &buf.get_ref()[position..position + length];

                #[allow(clippy::redundant_closure_call)]
                Ok($expr(slice))
            }
        }
    };
}

impl_integer_read!(u8);
impl_integer_read!(u16);
impl_integer_read!(u32);
impl_integer_read!(u64);
impl_integer_read!(u128);

#[test]
fn proto_read_u8() {
    let buf = [0x00, 0x01, 0x7f, 0xff];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(u8::frog_read(&mut cursor).unwrap(), 0);
    assert_eq!(u8::frog_read(&mut cursor).unwrap(), 1);
    assert_eq!(u8::frog_read(&mut cursor).unwrap(), 127);
    assert_eq!(u8::frog_read(&mut cursor).unwrap(), 255);
    assert_eq!(cursor.position(), buf.len() as u64);
}
#[test]
fn proto_read_u16() {
    let buf = [0x00, 0x01, 0x7f, 0xff, 0xff, 0xff];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(u16::frog_read(&mut cursor).unwrap(), 1);
    assert_eq!(u16::frog_read(&mut cursor).unwrap(), 32767);
    assert_eq!(u16::frog_read(&mut cursor).unwrap(), 65535);
    assert_eq!(cursor.position(), buf.len() as u64);
}
#[test]
fn proto_read_u32() {
    let buf = [0x00, 0x00, 0x00, 0x01, 0x7f, 0xff, 0xff, 0xff];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(u32::frog_read(&mut cursor).unwrap(), 1);
    assert_eq!(u32::frog_read(&mut cursor).unwrap(), 2_147_483_647);
    assert_eq!(cursor.position(), buf.len() as u64);
}
#[test]
fn proto_read_u64() {
    let buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x7f, 0xff];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(u64::frog_read(&mut cursor).unwrap(), 98303);
    assert_eq!(cursor.position(), buf.len() as u64);
}
#[test]
fn proto_read_u128() {
    let buf = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff,
    ];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(u128::frog_read(&mut cursor).unwrap(), 1_813_388_729_421_943_762_059_263);
    assert_eq!(
        u128::frog_read(&mut cursor).unwrap(),
        340_282_366_920_938_463_463_374_607_431_768_211_455
    );
    assert_eq!(cursor.position(), buf.len() as u64);
}

#[test]
fn proto_read_i64() {
    let buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x7f, 0xff];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(i64::frog_read(&mut cursor).unwrap(), 98303);
    assert_eq!(cursor.position(), buf.len() as u64);
}
#[test]
fn proto_read_i128() {
    let buf = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff,
    ];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(i128::frog_read(&mut cursor).unwrap(), 1_813_388_729_421_943_762_059_263);
    assert_eq!(i128::frog_read(&mut cursor).unwrap(), -1);
    assert_eq!(cursor.position(), buf.len() as u64);
}

impl_integer_read!(i8);
impl_integer_read!(i16);
impl_integer_read!(i32);
impl_integer_read!(i64);
impl_integer_read!(i128);

#[test]
fn proto_read_i8() {
    let buf = [0x00, 0x01, 0x7f, 0xff];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(i8::frog_read(&mut cursor).unwrap(), 0);
    assert_eq!(i8::frog_read(&mut cursor).unwrap(), 1);
    assert_eq!(i8::frog_read(&mut cursor).unwrap(), 127);
    assert_eq!(i8::frog_read(&mut cursor).unwrap(), -1);
    assert_eq!(cursor.position(), buf.len() as u64);
}
#[test]
fn proto_read_i16() {
    let buf = [0x00, 0x01, 0x7f, 0xff, 0xff, 0xff];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(i16::frog_read(&mut cursor).unwrap(), 1);
    assert_eq!(i16::frog_read(&mut cursor).unwrap(), 32767);
    assert_eq!(i16::frog_read(&mut cursor).unwrap(), -1);
    assert_eq!(cursor.position(), buf.len() as u64);
}
#[test]
fn proto_read_i32() {
    let buf = [0x00, 0x00, 0x00, 0x01, 0x7f, 0xff, 0xff, 0xff];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(i32::frog_read(&mut cursor).unwrap(), 1);
    assert_eq!(i32::frog_read(&mut cursor).unwrap(), 2_147_483_647);
    assert_eq!(cursor.position(), buf.len() as u64);
}
