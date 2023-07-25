use std::{collections::HashMap, hash::Hash};

use byteorder::{ReadBytesExt, BE};
use uuid::Uuid;

use super::{Decode, DecodeError, VarDecode};

impl Decode for bool {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        match u8::decode(buf)? {
            0 => Ok(false),
            1 => Ok(true),
            n => Err(DecodeError::Boolean(n)),
        }
    }
}

impl Decode for i8 {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_i8().map_err(DecodeError::from)
    }
}

impl Decode for u8 {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_u8().map_err(DecodeError::from)
    }
}

impl Decode for i16 {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_i16::<BE>().map_err(DecodeError::from)
    }
}

impl Decode for u16 {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_u16::<BE>().map_err(DecodeError::from)
    }
}

impl Decode for i32 {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_i32::<BE>().map_err(DecodeError::from)
    }
}

impl Decode for u32 {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_u32::<BE>().map_err(DecodeError::from)
    }
}

impl Decode for i64 {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_i64::<BE>().map_err(DecodeError::from)
    }
}

impl Decode for u64 {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_u64::<BE>().map_err(DecodeError::from)
    }
}

impl Decode for i128 {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_i128::<BE>().map_err(DecodeError::from)
    }
}

impl Decode for u128 {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_u128::<BE>().map_err(DecodeError::from)
    }
}

impl Decode for f32 {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_f32::<BE>().map_err(DecodeError::from)
    }
}

impl Decode for f64 {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_f64::<BE>().map_err(DecodeError::from)
    }
}

const MAX_STRING_LENGTH: u32 = 131068;

impl Decode for String {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let len = u32::var_decode(buf)?;

        if len > MAX_STRING_LENGTH {
            Err(DecodeError::StringTooLong(len))
        } else {
            let mut vec = vec![0; len as usize];
            buf.read_exact(&mut vec)?;

            Ok(String::from_utf8(vec)?)
        }
    }
}

impl Decode for Uuid {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        Ok(Uuid::from_u128(u128::decode(buf)?))
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let len = u32::var_decode(buf)?;

        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::decode(buf)?);
        }

        Ok(vec)
    }
}

impl<T: Decode, const N: usize> Decode for [T; N] {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let mut arr = Vec::with_capacity(N);
        for _ in 0..N {
            arr.push(T::decode(buf)?);
        }

        arr.try_into()
            .map_err(|_| unreachable!("Length is constant"))
    }
}

impl<T: Decode> Decode for Option<T> {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        match bool::decode(buf)? {
            true => Ok(Some(T::decode(buf)?)),
            false => Ok(None),
        }
    }
}

impl<K: Decode + Eq + Hash, V: Decode> Decode for HashMap<K, V> {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let len = u32::var_decode(buf)?;

        let mut map = HashMap::with_capacity(len as usize);
        for _ in 0..len {
            map.insert(K::decode(buf)?, V::decode(buf)?);
        }

        Ok(map)
    }
}

#[cfg(feature = "hashbrown")]
impl<K: Decode + Eq + Hash, V: Decode> Decode for hashbrown::HashMap<K, V> {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let len = u32::var_decode(buf)?;

        let mut map = hashbrown::HashMap::with_capacity(len as usize);
        for _ in 0..len {
            map.insert(K::decode(buf)?, V::decode(buf)?);
        }

        Ok(map)
    }
}

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
