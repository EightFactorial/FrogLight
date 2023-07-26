use std::collections::HashMap;

use byteorder::{WriteBytesExt, BE};
use uuid::Uuid;

use super::{Encode, EncodeError, VarEncode};

impl Encode for bool {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_u8(*self as u8).map_err(EncodeError::from)
    }
}

impl Encode for i8 {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_i8(*self).map_err(EncodeError::from)
    }
}

impl Encode for u8 {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_u8(*self).map_err(EncodeError::from)
    }
}

impl Encode for i16 {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_i16::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for u16 {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_u16::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for i32 {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_i32::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for u32 {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_u32::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for i64 {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_i64::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for u64 {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_u64::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for i128 {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_i128::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for u128 {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_u128::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for isize {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        i64::try_from(*self)?.encode(buf)
    }
}

impl Encode for usize {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        u64::try_from(*self)?.encode(buf)
    }
}

impl Encode for f32 {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_f32::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for f64 {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_f64::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for String {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.len().var_encode(buf)?;
        buf.write_all(self.as_bytes()).map_err(EncodeError::from)
    }
}

impl Encode for &str {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.len().var_encode(buf)?;
        buf.write_all(self.as_bytes()).map_err(EncodeError::from)
    }
}

impl Encode for Uuid {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.as_u128().encode(buf)
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.len().var_encode(buf)?;
        for item in self {
            item.encode(buf)?;
        }
        Ok(())
    }
}

impl<T: Encode, const N: usize> Encode for [T; N] {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        for item in self {
            item.encode(buf)?;
        }
        Ok(())
    }
}

impl<T: Encode> Encode for Option<T> {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        match self {
            Some(item) => {
                true.encode(buf)?;
                item.encode(buf)?;
            }
            None => false.encode(buf)?,
        }
        Ok(())
    }
}

impl<K: Encode, V: Encode> Encode for HashMap<K, V> {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.len().var_encode(buf)?;
        for (key, value) in self {
            key.encode(buf)?;
            value.encode(buf)?;
        }
        Ok(())
    }
}

#[cfg(feature = "hashbrown")]
impl<K: Encode, V: Encode> Encode for hashbrown::HashMap<K, V> {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.len().var_encode(buf)?;
        for (key, value) in self {
            key.encode(buf)?;
            value.encode(buf)?;
        }
        Ok(())
    }
}

#[test]
fn encode_bool() {
    let mut buf = Vec::new();

    assert!(true.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![1]);
    buf.clear();

    assert!(false.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
}

#[test]
fn encode_i8() {
    let mut buf = Vec::new();

    assert!(0i8.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(127i8.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![127]);
    buf.clear();

    assert!((-128i8).encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128]);
}

#[test]
fn encode_u8() {
    let mut buf = Vec::new();

    assert!(0u8.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(255u8.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255]);
}

#[test]
fn encode_i16() {
    let mut buf = Vec::new();

    assert!(0i16.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0]);
    buf.clear();

    assert!(32767i16.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![127, 255]);
    buf.clear();

    assert!((-32768i16).encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 0]);
}

#[test]
fn encode_u16() {
    let mut buf = Vec::new();

    assert!(0u16.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0]);
    buf.clear();

    assert!(65535u16.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255]);
}

#[test]
fn encode_i32() {
    let mut buf = Vec::new();

    assert!(0i32.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0]);
    buf.clear();

    assert!(2147483647i32.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![127, 255, 255, 255]);
    buf.clear();

    assert!((-2147483648i32).encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 0, 0, 0]);
}

#[test]
fn encode_u32() {
    let mut buf = Vec::new();

    assert!(0u32.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0]);
    buf.clear();

    assert!(4294967295u32.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255]);
}

#[test]
fn encode_i64() {
    let mut buf = Vec::new();

    assert!(0i64.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0, 0, 0, 0, 0]);
    buf.clear();

    assert!(9223372036854775807i64.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![127, 255, 255, 255, 255, 255, 255, 255]);
    buf.clear();

    assert!((-9223372036854775808i64).encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn encode_u64() {
    let mut buf = Vec::new();

    assert!(0u64.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0, 0, 0, 0, 0]);
    buf.clear();

    assert!(18446744073709551615u64.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 255, 255, 255, 255]);
}

#[test]
fn encode_i128() {
    let mut buf = Vec::new();

    assert!(0i128.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0; 16]);
    buf.clear();

    assert!(170141183460469231731687303715884105727i128
        .encode(&mut buf)
        .is_ok());
    assert_eq!(
        buf,
        vec![127, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
    );
    buf.clear();
}

#[test]
fn encode_u128() {
    let mut buf = Vec::new();

    assert!(0u128.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0; 16]);
    buf.clear();

    assert!(340282366920938463463374607431768211455u128
        .encode(&mut buf)
        .is_ok());
    assert_eq!(buf, vec![255; 16]);
    buf.clear();
}

#[test]
fn encode_f32() {
    let mut buf = Vec::new();

    assert!(0f32.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0]);
    buf.clear();

    assert!(1.0f32.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![63, 128, 0, 0]);
    buf.clear();

    assert!(1.5f32.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![63, 192, 0, 0]);
    buf.clear();
}

#[test]
fn encode_f64() {
    let mut buf = Vec::new();

    assert!(0f64.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0, 0, 0, 0, 0]);
    buf.clear();

    assert!(1.0f64.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![63, 240, 0, 0, 0, 0, 0, 0]);
    buf.clear();

    assert!(1.5f64.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![63, 248, 0, 0, 0, 0, 0, 0]);
    buf.clear();
}

#[test]
fn encode_string() {
    let mut buf = Vec::new();

    assert!("".encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!("hello world".encode(&mut buf).is_ok());
    assert_eq!(
        buf,
        vec![11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]
    );
}

#[test]
fn encode_option() {
    let mut buf = Vec::new();

    assert!(None::<i32>.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(Some(42i32).encode(&mut buf).is_ok());
    assert_eq!(buf, vec![1u8, 0u8, 0u8, 0u8, 42u8]);
}

#[test]
fn encode_vec() {
    let mut buf = Vec::new();

    assert!(vec![0u32].encode(&mut buf).is_ok());
    assert_eq!(buf, vec![1u8, 0u8, 0u8, 0u8, 0u8]);
    buf.clear();

    assert!(vec![1u8, 2u8, 3u8].encode(&mut buf).is_ok());
    assert_eq!(buf, vec![3u8, 1u8, 2u8, 3u8]);
}
