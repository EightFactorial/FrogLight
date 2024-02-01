use std::io::Read;

use super::{FrogVarRead, ReadError};

macro_rules! impl_integer_var_read {
    ($ty:ty, $bytes:expr) => {
        impl FrogVarRead for $ty {
            fn fg_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<$ty, ReadError> {
                let mut byte = [0];
                let mut ans: $ty = 0;
                for i in 0..$bytes {
                    buf.read_exact(&mut byte)?;
                    ans |= ((byte[0] & 0b0111_1111) as $ty) << (7 * i);
                    if byte[0] & 0b1000_0000 == 0 {
                        break;
                    }
                }
                Ok(ans)
            }
        }
    };
}

impl_integer_var_read!(u16, 5);
impl_integer_var_read!(u32, 5);
impl_integer_var_read!(u64, 10);
impl_integer_var_read!(u128, 20);
impl_integer_var_read!(usize, 10);

impl_integer_var_read!(i16, 5);
impl_integer_var_read!(i32, 5);
impl_integer_var_read!(i64, 10);
impl_integer_var_read!(i128, 20);
impl_integer_var_read!(isize, 10);

#[test]
fn proto_var_read_u32() {
    assert_eq!(u32::fg_var_read(&mut std::io::Cursor::new(&[0])).unwrap(), 0);
    assert_eq!(u32::fg_var_read(&mut std::io::Cursor::new(&[1])).unwrap(), 1);
    assert_eq!(u32::fg_var_read(&mut std::io::Cursor::new(&[2])).unwrap(), 2);
    assert_eq!(u32::fg_var_read(&mut std::io::Cursor::new(&[127])).unwrap(), 127);
    assert_eq!(u32::fg_var_read(&mut std::io::Cursor::new(&[128, 1])).unwrap(), 128);
    assert_eq!(u32::fg_var_read(&mut std::io::Cursor::new(&[254, 1])).unwrap(), 254);
    assert_eq!(u32::fg_var_read(&mut std::io::Cursor::new(&[255, 1])).unwrap(), 255);
    assert_eq!(u32::fg_var_read(&mut std::io::Cursor::new(&[221, 199, 1])).unwrap(), 25565);
    assert_eq!(u32::fg_var_read(&mut std::io::Cursor::new(&[255, 255, 127])).unwrap(), 2_097_151);
    assert_eq!(
        u32::fg_var_read(&mut std::io::Cursor::new(&[255, 255, 255, 255, 7])).unwrap(),
        2_147_483_647
    );
    assert_eq!(
        u32::fg_var_read(&mut std::io::Cursor::new(&[255, 255, 255, 255, 15])).unwrap(),
        4_294_967_295
    );
}
#[test]
fn proto_var_read_u64() {
    assert_eq!(u64::fg_var_read(&mut std::io::Cursor::new(&[0])).unwrap(), 0);
    assert_eq!(u64::fg_var_read(&mut std::io::Cursor::new(&[1])).unwrap(), 1);
    assert_eq!(u64::fg_var_read(&mut std::io::Cursor::new(&[2])).unwrap(), 2);
    assert_eq!(u64::fg_var_read(&mut std::io::Cursor::new(&[127])).unwrap(), 127);
    assert_eq!(u64::fg_var_read(&mut std::io::Cursor::new(&[128, 1])).unwrap(), 128);
    assert_eq!(u64::fg_var_read(&mut std::io::Cursor::new(&[254, 1])).unwrap(), 254);
    assert_eq!(u64::fg_var_read(&mut std::io::Cursor::new(&[255, 1])).unwrap(), 255);
    assert_eq!(u64::fg_var_read(&mut std::io::Cursor::new(&[221, 199, 1])).unwrap(), 25565);
    assert_eq!(u64::fg_var_read(&mut std::io::Cursor::new(&[255, 255, 127])).unwrap(), 2_097_151);
    assert_eq!(
        u64::fg_var_read(&mut std::io::Cursor::new(&[255, 255, 255, 255, 255, 255, 255, 255, 127]))
            .unwrap(),
        9_223_372_036_854_775_807
    );
    assert_eq!(
        u64::fg_var_read(&mut std::io::Cursor::new(&[
            255, 255, 255, 255, 255, 255, 255, 255, 255, 127
        ]))
        .unwrap(),
        18_446_744_073_709_551_615
    );
}

#[test]
fn proto_var_read_i32() {
    assert_eq!(
        i32::fg_var_read(&mut std::io::Cursor::new(&[128, 128, 128, 128, 8])).unwrap(),
        -2_147_483_648
    );
    assert_eq!(i32::fg_var_read(&mut std::io::Cursor::new(&[255, 255, 255, 255, 15])).unwrap(), -1);
    assert_eq!(i32::fg_var_read(&mut std::io::Cursor::new(&[0])).unwrap(), 0);
    assert_eq!(i32::fg_var_read(&mut std::io::Cursor::new(&[1])).unwrap(), 1);
    assert_eq!(i32::fg_var_read(&mut std::io::Cursor::new(&[2])).unwrap(), 2);
    assert_eq!(i32::fg_var_read(&mut std::io::Cursor::new(&[127])).unwrap(), 127);
    assert_eq!(i32::fg_var_read(&mut std::io::Cursor::new(&[128, 1])).unwrap(), 128);
    assert_eq!(i32::fg_var_read(&mut std::io::Cursor::new(&[254, 1])).unwrap(), 254);
    assert_eq!(i32::fg_var_read(&mut std::io::Cursor::new(&[255, 1])).unwrap(), 255);
    assert_eq!(i32::fg_var_read(&mut std::io::Cursor::new(&[221, 199, 1])).unwrap(), 25565);
    assert_eq!(i32::fg_var_read(&mut std::io::Cursor::new(&[255, 255, 127])).unwrap(), 2_097_151);
    assert_eq!(
        i32::fg_var_read(&mut std::io::Cursor::new(&[255, 255, 255, 255, 7])).unwrap(),
        2_147_483_647
    );
}
#[test]
fn proto_var_read_i64() {
    assert_eq!(
        i64::fg_var_read(&mut std::io::Cursor::new(&[
            128, 128, 128, 128, 128, 128, 128, 128, 128, 1
        ]))
        .unwrap(),
        -9_223_372_036_854_775_808
    );
    assert_eq!(
        i64::fg_var_read(&mut std::io::Cursor::new(&[
            255, 255, 255, 255, 255, 255, 255, 255, 255, 1
        ]))
        .unwrap(),
        -1
    );
    assert_eq!(i64::fg_var_read(&mut std::io::Cursor::new(&[0])).unwrap(), 0);
    assert_eq!(i64::fg_var_read(&mut std::io::Cursor::new(&[1])).unwrap(), 1);
    assert_eq!(i64::fg_var_read(&mut std::io::Cursor::new(&[2])).unwrap(), 2);
    assert_eq!(i64::fg_var_read(&mut std::io::Cursor::new(&[127])).unwrap(), 127);
    assert_eq!(i64::fg_var_read(&mut std::io::Cursor::new(&[128, 1])).unwrap(), 128);
    assert_eq!(i64::fg_var_read(&mut std::io::Cursor::new(&[254, 1])).unwrap(), 254);
    assert_eq!(i64::fg_var_read(&mut std::io::Cursor::new(&[255, 1])).unwrap(), 255);
    assert_eq!(i64::fg_var_read(&mut std::io::Cursor::new(&[221, 199, 1])).unwrap(), 25565);
    assert_eq!(i64::fg_var_read(&mut std::io::Cursor::new(&[255, 255, 127])).unwrap(), 2_097_151);
    assert_eq!(
        i64::fg_var_read(&mut std::io::Cursor::new(&[255, 255, 255, 255, 255, 255, 255, 255, 127]))
            .unwrap(),
        9_223_372_036_854_775_807
    );
}
