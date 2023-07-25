use std::{collections::HashMap, hash::Hash};

use crate::buffer::Decode;

use super::{DecodeError, VarDecode};

impl VarDecode for i16 {
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        Ok(i32::var_decode(buf)?.try_into()?)
    }
}

impl VarDecode for u16 {
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        Ok(u32::var_decode(buf)?.try_into()?)
    }
}

impl VarDecode for i32 {
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let mut num = [0];
        let mut ans = 0;
        for i in 0..5 {
            buf.read_exact(&mut num)?;
            ans |= ((num[0] & 0b0111_1111) as i32) << (7 * i);
            if num[0] & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(ans)
    }
}

impl VarDecode for u32 {
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        i32::var_decode(buf).map(|x| x as u32)
    }
}

impl VarDecode for i64 {
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let mut num = [0];
        let mut ans = 0;
        for i in 0..10 {
            buf.read_exact(&mut num)?;
            ans |= ((num[0] & 0b0111_1111) as i64) << (7 * i);
            if num[0] & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(ans)
    }
}

impl VarDecode for u64 {
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        i64::var_decode(buf).map(|x| x as u64)
    }
}

impl VarDecode for isize {
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        isize::try_from(i64::var_decode(buf)?).map_err(DecodeError::from)
    }
}

impl VarDecode for usize {
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        usize::try_from(u64::var_decode(buf)?).map_err(DecodeError::from)
    }
}

impl<T: VarDecode> VarDecode for Option<T> {
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        match u32::var_decode(buf)? {
            0 => Ok(None),
            _ => Ok(Some(T::var_decode(buf)?)),
        }
    }
}

impl<K: Decode + Eq + Hash, V: VarDecode> VarDecode for HashMap<K, V> {
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let len = u32::var_decode(buf)?;
        let mut map = HashMap::with_capacity(len as usize);
        for _ in 0..len {
            map.insert(K::decode(buf)?, V::var_decode(buf)?);
        }
        Ok(map)
    }
}

#[cfg(feature = "hashbrown")]
impl<K: Decode + Eq + Hash, V: VarDecode> VarDecode for hashbrown::HashMap<K, V> {
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let len = u32::var_decode(buf)?;
        let mut map = hashbrown::HashMap::with_capacity(len as usize);
        for _ in 0..len {
            map.insert(K::decode(buf)?, V::var_decode(buf)?);
        }
        Ok(map)
    }
}

#[test]
fn decode_i32() {
    assert_eq!(
        i32::var_decode(&mut &[128, 128, 128, 128, 8][..]),
        Ok(-2147483648)
    );
    assert_eq!(i32::var_decode(&mut &[255, 255, 255, 255, 15][..]), Ok(-1));
    assert_eq!(i32::var_decode(&mut &[0][..]), Ok(0));
    assert_eq!(i32::var_decode(&mut &[1][..]), Ok(1));
    assert_eq!(i32::var_decode(&mut &[2][..]), Ok(2));
    assert_eq!(i32::var_decode(&mut &[127][..]), Ok(127));
    assert_eq!(i32::var_decode(&mut &[128, 1][..]), Ok(128));
    assert_eq!(i32::var_decode(&mut &[254, 1][..]), Ok(254));
    assert_eq!(i32::var_decode(&mut &[255, 1][..]), Ok(255));
    assert_eq!(i32::var_decode(&mut &[221, 199, 1][..]), Ok(25565));
    assert_eq!(i32::var_decode(&mut &[255, 255, 127][..]), Ok(2097151));
    assert_eq!(
        i32::var_decode(&mut &[255, 255, 255, 255, 7][..]),
        Ok(2147483647)
    );
}

#[test]
fn decode_u32() {
    assert_eq!(u32::var_decode(&mut &[0][..]), Ok(0));
    assert_eq!(u32::var_decode(&mut &[1][..]), Ok(1));
    assert_eq!(u32::var_decode(&mut &[2][..]), Ok(2));
    assert_eq!(u32::var_decode(&mut &[127][..]), Ok(127));
    assert_eq!(u32::var_decode(&mut &[128, 1][..]), Ok(128));
    assert_eq!(u32::var_decode(&mut &[254, 1][..]), Ok(254));
    assert_eq!(u32::var_decode(&mut &[255, 1][..]), Ok(255));
    assert_eq!(u32::var_decode(&mut &[221, 199, 1][..]), Ok(25565));
    assert_eq!(u32::var_decode(&mut &[255, 255, 127][..]), Ok(2097151));
    assert_eq!(
        u32::var_decode(&mut &[255, 255, 255, 255, 7][..]),
        Ok(2147483647)
    );
    assert_eq!(
        u32::var_decode(&mut &[255, 255, 255, 255, 15][..]),
        Ok(4294967295)
    );
}

#[test]
fn decode_i64() {
    assert_eq!(
        i64::var_decode(&mut &[128, 128, 128, 128, 128, 128, 128, 128, 128, 1][..]),
        Ok(-9223372036854775808)
    );
    assert_eq!(
        i64::var_decode(&mut &[255, 255, 255, 255, 255, 255, 255, 255, 255, 1][..]),
        Ok(-1)
    );
    assert_eq!(i64::var_decode(&mut &[0][..]), Ok(0));
    assert_eq!(i64::var_decode(&mut &[1][..]), Ok(1));
    assert_eq!(i64::var_decode(&mut &[2][..]), Ok(2));
    assert_eq!(i64::var_decode(&mut &[127][..]), Ok(127));
    assert_eq!(i64::var_decode(&mut &[128, 1][..]), Ok(128));
    assert_eq!(i64::var_decode(&mut &[254, 1][..]), Ok(254));
    assert_eq!(i64::var_decode(&mut &[255, 1][..]), Ok(255));
    assert_eq!(i64::var_decode(&mut &[221, 199, 1][..]), Ok(25565));
    assert_eq!(i64::var_decode(&mut &[255, 255, 127][..]), Ok(2097151));
    assert_eq!(
        i64::var_decode(&mut &[255, 255, 255, 255, 255, 255, 255, 255, 127][..]),
        Ok(9223372036854775807)
    );
}

#[test]
fn decode_u64() {
    assert_eq!(u64::var_decode(&mut &[0][..]), Ok(0));
    assert_eq!(u64::var_decode(&mut &[1][..]), Ok(1));
    assert_eq!(u64::var_decode(&mut &[2][..]), Ok(2));
    assert_eq!(u64::var_decode(&mut &[127][..]), Ok(127));
    assert_eq!(u64::var_decode(&mut &[128, 1][..]), Ok(128));
    assert_eq!(u64::var_decode(&mut &[254, 1][..]), Ok(254));
    assert_eq!(u64::var_decode(&mut &[255, 1][..]), Ok(255));
    assert_eq!(u64::var_decode(&mut &[221, 199, 1][..]), Ok(25565));
    assert_eq!(u64::var_decode(&mut &[255, 255, 127][..]), Ok(2097151));
    assert_eq!(
        u64::var_decode(&mut &[255, 255, 255, 255, 255, 255, 255, 255, 127][..]),
        Ok(9223372036854775807)
    );
    assert_eq!(
        u64::var_decode(&mut &[255, 255, 255, 255, 255, 255, 255, 255, 255, 127][..]),
        Ok(18446744073709551615)
    );
}
