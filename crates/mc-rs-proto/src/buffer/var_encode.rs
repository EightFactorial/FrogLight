use std::collections::HashMap;

use crate::buffer::Encode;

use super::{EncodeError, VarEncode};

impl VarEncode for i16 {
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        i32::from(*self).var_encode(buf)
    }
}

impl VarEncode for u16 {
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        i32::from(*self).var_encode(buf)
    }
}

impl VarEncode for i32 {
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        let mut value = *self;
        let mut byte = [0];
        if value == 0 {
            buf.write_all(&byte)?;
        } else {
            while value != 0 {
                byte[0] = (value & 0b0111_1111) as u8;
                value = (value >> 7) & (i32::max_value() >> 6);
                if value != 0 {
                    byte[0] |= 0b1000_0000;
                }
                buf.write_all(&byte)?;
            }
        }
        Ok(())
    }
}

impl VarEncode for u32 {
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        (*self as i32).var_encode(buf)
    }
}

impl VarEncode for i64 {
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        let mut value = *self;
        let mut byte = [0];

        if value == 0 {
            buf.write_all(&byte)?;
        } else {
            while value != 0 {
                byte[0] = (value & 0b0111_1111) as u8;
                value = (value >> 7) & (i64::max_value() >> 6);
                if value != 0 {
                    byte[0] |= 0b1000_0000;
                }
                buf.write_all(&byte)?;
            }
        }
        Ok(())
    }
}

impl VarEncode for u64 {
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        (*self as i64).var_encode(buf)
    }
}

impl VarEncode for isize {
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        i64::try_from(*self)?.var_encode(buf)
    }
}

impl VarEncode for usize {
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        u64::try_from(*self)?.var_encode(buf)
    }
}

impl<T: VarEncode> VarEncode for Option<T> {
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        match self {
            Some(value) => {
                1u32.var_encode(buf)?;
                value.var_encode(buf)
            }
            None => 0u32.var_encode(buf),
        }
    }
}

impl<K: Encode, V: VarEncode> VarEncode for HashMap<K, V> {
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.len().var_encode(buf)?;
        for (key, value) in self {
            key.encode(buf)?;
            value.var_encode(buf)?;
        }
        Ok(())
    }
}

impl<K: Encode, V: VarEncode> VarEncode for hashbrown::HashMap<K, V> {
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.len().var_encode(buf)?;
        for (key, value) in self {
            key.encode(buf)?;
            value.var_encode(buf)?;
        }
        Ok(())
    }
}

#[test]
fn var_encode_i32() {
    let mut buf = Vec::with_capacity(5);

    assert!((-2147483648i32).var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 128, 128, 128, 8]);
    buf.clear();

    assert!((-1i32).var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 15]);
    buf.clear();

    assert!(0i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(1i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![1]);
    buf.clear();

    assert!(2i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![2]);
    buf.clear();

    assert!(127i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![127]);
    buf.clear();

    assert!(128i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 1]);
    buf.clear();

    assert!(254i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![254, 1]);
    buf.clear();

    assert!(255i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 1]);
    buf.clear();

    assert!(25565i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![221, 199, 1]);
    buf.clear();

    assert!(2097151i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 127]);
    buf.clear();

    assert!(2147483647i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 7]);
    buf.clear();
}

#[test]
fn var_encode_u32() {
    let mut buf = Vec::with_capacity(5);

    assert!(0u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(1u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![1]);
    buf.clear();

    assert!(2u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![2]);
    buf.clear();

    assert!(127u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![127]);
    buf.clear();

    assert!(128u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 1]);
    buf.clear();

    assert!(254u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![254, 1]);
    buf.clear();

    assert!(255u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 1]);
    buf.clear();

    assert!(25565u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![221, 199, 1]);
    buf.clear();

    assert!(2097151u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 127]);
    buf.clear();

    assert!(2147483647u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 7]);
    buf.clear();

    assert!(4294967295u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 15]);
    buf.clear();
}

#[test]
fn var_encode_i64() {
    let mut buf = Vec::with_capacity(10);

    assert!((-9223372036854775808i64).var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 128, 128, 128, 128, 128, 128, 128, 128, 1]);
    buf.clear();

    assert!((-2147483648i64).var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 128, 128, 128, 248, 255, 255, 255, 255, 1]);
    buf.clear();

    assert!((-1i64).var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
    buf.clear();

    assert!(0i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(1i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![1]);
    buf.clear();

    assert!(2i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![2]);
    buf.clear();

    assert!(127i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![127]);
    buf.clear();

    assert!(128i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 1]);
    buf.clear();

    assert!(254i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![254, 1]);
    buf.clear();

    assert!(255i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 1]);
    buf.clear();

    assert!(25565i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![221, 199, 1]);
    buf.clear();

    assert!(2097151i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 127]);
    buf.clear();

    assert!(2147483647i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 7]);
    buf.clear();

    assert!(4294967295i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 15]);
    buf.clear();

    assert!(9223372036854775807i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 255, 255, 255, 255, 127]);
    buf.clear();
}

#[test]
fn var_encode_u64() {
    let mut buf = Vec::with_capacity(10);

    assert!(0u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(1u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![1]);
    buf.clear();

    assert!(2u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![2]);
    buf.clear();

    assert!(127u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![127]);
    buf.clear();

    assert!(128u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 1]);
    buf.clear();

    assert!(254u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![254, 1]);
    buf.clear();

    assert!(255u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 1]);
    buf.clear();

    assert!(25565u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![221, 199, 1]);
    buf.clear();

    assert!(2097151u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 127]);
    buf.clear();

    assert!(2147483647u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 7]);
    buf.clear();

    assert!(4294967295u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 15]);
    buf.clear();

    assert!(9223372036854775807u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 255, 255, 255, 255, 127]);
    buf.clear();

    assert!(18446744073709551615u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
    buf.clear();
}
