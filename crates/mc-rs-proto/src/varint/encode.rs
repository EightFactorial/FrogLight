use std::io::Write;

use super::{VarEncode, VarError};

impl VarEncode for i16 {
    fn var_encode(&self) -> Result<Vec<u8>, VarError> { i32::var_encode(&(*self as i32)) }
}

impl VarEncode for u16 {
    fn var_encode(&self) -> Result<Vec<u8>, VarError> { u32::var_encode(&(*self as u32)) }
}

impl VarEncode for i32 {
    fn var_encode(&self) -> Result<Vec<u8>, VarError> {
        let mut value = *self;
        let mut buf = Vec::with_capacity(5);

        let mut byte = [0];
        if value == 0 {
            buf.push(byte[0]);
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
        Ok(buf)
    }
}

impl VarEncode for u32 {
    fn var_encode(&self) -> Result<Vec<u8>, VarError> { i32::var_encode(&(*self as i32)) }
}

impl VarEncode for i64 {
    fn var_encode(&self) -> Result<Vec<u8>, VarError> {
        let mut value = *self;
        let mut buf = Vec::with_capacity(10);

        let mut byte = [0];
        if value == 0 {
            buf.push(byte[0]);
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
        Ok(buf)
    }
}

impl VarEncode for u64 {
    fn var_encode(&self) -> Result<Vec<u8>, VarError> { i64::var_encode(&(*self as i64)) }
}

impl VarEncode for isize {
    fn var_encode(&self) -> Result<Vec<u8>, VarError> { i64::var_encode(&(*self as i64)) }
}

impl VarEncode for usize {
    fn var_encode(&self) -> Result<Vec<u8>, VarError> { u64::var_encode(&(*self as u64)) }
}

#[test]
fn encode_i32() {
    assert_eq!(
        (-2147483648i32).var_encode(),
        Ok(vec![128, 128, 128, 128, 8])
    );
    assert_eq!((-1i32).var_encode(), Ok(vec![255, 255, 255, 255, 15]));
    assert_eq!(0i32.var_encode(), Ok(vec![0]));
    assert_eq!(1i32.var_encode(), Ok(vec![1]));
    assert_eq!(2i32.var_encode(), Ok(vec![2]));
    assert_eq!(127i32.var_encode(), Ok(vec![127]));
    assert_eq!(128i32.var_encode(), Ok(vec![128, 1]));
    assert_eq!(254i32.var_encode(), Ok(vec![254, 1]));
    assert_eq!(255i32.var_encode(), Ok(vec![255, 1]));
    assert_eq!(25565i32.var_encode(), Ok(vec![221, 199, 1]));
    assert_eq!(2097151i32.var_encode(), Ok(vec![255, 255, 127]));
    assert_eq!(2147483647i32.var_encode(), Ok(vec![255, 255, 255, 255, 7]));
}

#[test]
fn encode_u32() {
    assert_eq!(0u32.var_encode(), Ok(vec![0]));
    assert_eq!(1u32.var_encode(), Ok(vec![1]));
    assert_eq!(2u32.var_encode(), Ok(vec![2]));
    assert_eq!(127u32.var_encode(), Ok(vec![127]));
    assert_eq!(128u32.var_encode(), Ok(vec![128, 1]));
    assert_eq!(254u32.var_encode(), Ok(vec![254, 1]));
    assert_eq!(255u32.var_encode(), Ok(vec![255, 1]));
    assert_eq!(25565u32.var_encode(), Ok(vec![221, 199, 1]));
    assert_eq!(2097151u32.var_encode(), Ok(vec![255, 255, 127]));
    assert_eq!(2147483647u32.var_encode(), Ok(vec![255, 255, 255, 255, 7]));
}

#[test]
fn encode_i64() {
    assert_eq!(
        (-9223372036854775808i64).var_encode(),
        Ok(vec![128, 128, 128, 128, 128, 128, 128, 128, 128, 1])
    );
    assert_eq!(
        (-2147483648i64).var_encode(),
        Ok(vec![128, 128, 128, 128, 248, 255, 255, 255, 255, 1])
    );
    assert_eq!(
        (-1i64).var_encode(),
        Ok(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1])
    );
    assert_eq!(0i64.var_encode(), Ok(vec![0]));
    assert_eq!(1i64.var_encode(), Ok(vec![1]));
    assert_eq!(2i64.var_encode(), Ok(vec![2]));
    assert_eq!(127i64.var_encode(), Ok(vec![127]));
    assert_eq!(128i64.var_encode(), Ok(vec![128, 1]));
    assert_eq!(254i64.var_encode(), Ok(vec![254, 1]));
    assert_eq!(255i64.var_encode(), Ok(vec![255, 1]));
    assert_eq!(25565i64.var_encode(), Ok(vec![221, 199, 1]));
    assert_eq!(2097151i64.var_encode(), Ok(vec![255, 255, 127]));
    assert_eq!(2147483647i64.var_encode(), Ok(vec![255, 255, 255, 255, 7]));
    assert_eq!(
        9223372036854775807i64.var_encode(),
        Ok(vec![255, 255, 255, 255, 255, 255, 255, 255, 127])
    );
}

#[test]
fn encode_u64() {
    assert_eq!(0u64.var_encode(), Ok(vec![0]));
    assert_eq!(1u64.var_encode(), Ok(vec![1]));
    assert_eq!(2u64.var_encode(), Ok(vec![2]));
    assert_eq!(127u64.var_encode(), Ok(vec![127]));
    assert_eq!(128u64.var_encode(), Ok(vec![128, 1]));
    assert_eq!(254u64.var_encode(), Ok(vec![254, 1]));
    assert_eq!(255u64.var_encode(), Ok(vec![255, 1]));
    assert_eq!(25565u64.var_encode(), Ok(vec![221, 199, 1]));
    assert_eq!(2097151u64.var_encode(), Ok(vec![255, 255, 127]));
    assert_eq!(2147483647u64.var_encode(), Ok(vec![255, 255, 255, 255, 7]));
    assert_eq!(
        9223372036854775807u64.var_encode(),
        Ok(vec![255, 255, 255, 255, 255, 255, 255, 255, 127])
    );
    assert_eq!(
        18446744073709551615u64.var_encode(),
        Ok(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1])
    );
}
