#[cfg(test)]
use pretty_assertions::assert_eq;

#[cfg(test)]
use crate::buffer::Decode;

#[test]
fn decode_bool() {
    let buf = [0x00, 0x01];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(bool::decode(&mut cursor), Ok(false));
    assert_eq!(bool::decode(&mut cursor), Ok(true));
    assert_eq!(cursor.position(), buf.len() as u64);
}

#[test]
fn decode_i8() {
    let buf = [0x00, 0x01, 0x7f, 0xff];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(i8::decode(&mut cursor), Ok(0));
    assert_eq!(i8::decode(&mut cursor), Ok(1));
    assert_eq!(i8::decode(&mut cursor), Ok(127));
    assert_eq!(i8::decode(&mut cursor), Ok(-1));
    assert_eq!(cursor.position(), buf.len() as u64);
}

#[test]
fn decode_u8() {
    let buf = [0x00, 0x01, 0x7f, 0xff];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(u8::decode(&mut cursor), Ok(0));
    assert_eq!(u8::decode(&mut cursor), Ok(1));
    assert_eq!(u8::decode(&mut cursor), Ok(127));
    assert_eq!(u8::decode(&mut cursor), Ok(255));
    assert_eq!(cursor.position(), buf.len() as u64);
}

#[test]
fn decode_i16() {
    let buf = [0x00, 0x01, 0x7f, 0xff, 0xff, 0xff];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(i16::decode(&mut cursor), Ok(1));
    assert_eq!(i16::decode(&mut cursor), Ok(32767));
    assert_eq!(i16::decode(&mut cursor), Ok(-1));
    assert_eq!(cursor.position(), buf.len() as u64);
}

#[test]
fn decode_u16() {
    let buf = [0x00, 0x01, 0x7f, 0xff, 0xff, 0xff];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(u16::decode(&mut cursor), Ok(1));
    assert_eq!(u16::decode(&mut cursor), Ok(32767));
    assert_eq!(u16::decode(&mut cursor), Ok(65535));
    assert_eq!(cursor.position(), buf.len() as u64);
}

#[test]
fn decode_i32() {
    let buf = [0x00, 0x00, 0x00, 0x01, 0x7f, 0xff, 0xff, 0xff];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(i32::decode(&mut cursor), Ok(1));
    assert_eq!(i32::decode(&mut cursor), Ok(2147483647));
    assert_eq!(cursor.position(), buf.len() as u64);
}

#[test]
fn decode_u32() {
    let buf = [0x00, 0x00, 0x00, 0x01, 0x7f, 0xff, 0xff, 0xff];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(u32::decode(&mut cursor), Ok(1));
    assert_eq!(u32::decode(&mut cursor), Ok(2147483647));
    assert_eq!(cursor.position(), buf.len() as u64);
}

#[test]
fn decode_i64() {
    let buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x7f, 0xff];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(i64::decode(&mut cursor), Ok(98303));
    assert_eq!(cursor.position(), buf.len() as u64);
}

#[test]
fn decode_u64() {
    let buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x7f, 0xff];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(u64::decode(&mut cursor), Ok(98303));
    assert_eq!(cursor.position(), buf.len() as u64);
}

#[test]
fn decode_i128() {
    let buf = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff,
    ];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(i128::decode(&mut cursor), Ok(1813388729421943762059263));
    assert_eq!(i128::decode(&mut cursor), Ok(-1));
    assert_eq!(cursor.position(), buf.len() as u64);
}

#[test]
fn decode_u128() {
    let buf = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff,
    ];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(u128::decode(&mut cursor), Ok(1813388729421943762059263));
    assert_eq!(
        u128::decode(&mut cursor),
        Ok(340282366920938463463374607431768211455)
    );
    assert_eq!(cursor.position(), buf.len() as u64);
}

#[test]
fn decode_f32() {
    let buf = [0x3f, 0x80, 0x00, 0x00];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(f32::decode(&mut cursor), Ok(1.0));
    assert_eq!(cursor.position(), buf.len() as u64);
}

#[test]
fn decode_f64() {
    let buf = [0x3f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(f64::decode(&mut cursor), Ok(1.0));
    assert_eq!(cursor.position(), buf.len() as u64);
}

#[test]
fn decode_string() {
    let buf = [0x01, 0x61, 0x01, 0x62, 0x02, 0x61, 0x62];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(String::decode(&mut cursor), Ok("a".to_string()));
    assert_eq!(String::decode(&mut cursor), Ok("b".to_string()));
    assert_eq!(String::decode(&mut cursor), Ok("ab".to_string()));
    assert_eq!(cursor.position(), buf.len() as u64);
}

#[test]
fn decode_option() {
    let buf = [0x00, 0x01, 0x01, 0x61, 0x01, 0x01, 0x62, 0x00];
    let mut cursor = std::io::Cursor::new(&buf[..]);

    assert_eq!(Option::<String>::decode(&mut cursor), Ok(None));
    assert_eq!(
        Option::<String>::decode(&mut cursor),
        Ok(Some("a".to_string()))
    );
    assert_eq!(
        Option::<String>::decode(&mut cursor),
        Ok(Some("b".to_string()))
    );
    assert_eq!(Option::<String>::decode(&mut cursor), Ok(None));
    assert_eq!(cursor.position(), buf.len() as u64);
}
