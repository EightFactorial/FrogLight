use std::{collections::HashMap, hash::Hash};

use crate::buffer::Decode;

use super::{DecodeError, VarDecode};

impl VarDecode for i16 {
    #[inline]
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        Ok(i32::var_decode(buf)?.try_into()?)
    }
}

impl VarDecode for u16 {
    #[inline]
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        Ok(u32::var_decode(buf)?.try_into()?)
    }
}

impl VarDecode for i32 {
    #[inline]
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let mut byte = [0];
        let mut ans = 0;
        for i in 0..5 {
            buf.read_exact(&mut byte)?;
            ans |= ((byte[0] & 0b0111_1111) as i32) << (7 * i);
            if byte[0] & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(ans)
    }
}

impl VarDecode for u32 {
    #[inline]
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        i32::var_decode(buf).map(|x| x as u32)
    }
}

impl VarDecode for i64 {
    #[inline]
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let mut byte = [0];
        let mut ans = 0;
        for i in 0..10 {
            buf.read_exact(&mut byte)?;
            ans |= ((byte[0] & 0b0111_1111) as i64) << (7 * i);
            if byte[0] & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(ans)
    }
}

impl VarDecode for u64 {
    #[inline]
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        i64::var_decode(buf).map(|x| x as u64)
    }
}

impl VarDecode for isize {
    #[inline]
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        isize::try_from(i64::var_decode(buf)?).map_err(DecodeError::from)
    }
}

impl VarDecode for usize {
    #[inline]
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        usize::try_from(u64::var_decode(buf)?).map_err(DecodeError::from)
    }
}

impl<T: VarDecode> VarDecode for Vec<T> {
    #[inline]
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let len = u32::var_decode(buf)?;

        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::var_decode(buf)?);
        }

        Ok(vec)
    }
}

impl<T: VarDecode> VarDecode for Option<T> {
    #[inline]
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        match u32::var_decode(buf)? {
            0 => Ok(None),
            _ => Ok(Some(T::var_decode(buf)?)),
        }
    }
}

impl<K: Decode + Eq + Hash, V: VarDecode> VarDecode for HashMap<K, V> {
    #[inline]
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
    #[inline]
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let len = u32::var_decode(buf)?;
        let mut map = hashbrown::HashMap::with_capacity(len as usize);
        for _ in 0..len {
            map.insert(K::decode(buf)?, V::var_decode(buf)?);
        }
        Ok(map)
    }
}
