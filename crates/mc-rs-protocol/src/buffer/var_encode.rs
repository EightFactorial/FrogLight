use std::collections::HashMap;

use crate::buffer::Encode;

use super::{EncodeError, VarEncode};

impl VarEncode for i16 {
    #[inline]
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        i32::from(*self).var_encode(buf)
    }
}

impl VarEncode for u16 {
    #[inline]
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        i32::from(*self).var_encode(buf)
    }
}

impl VarEncode for i32 {
    #[inline]
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
    #[inline]
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        (*self as i32).var_encode(buf)
    }
}

impl VarEncode for i64 {
    #[inline]
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
    #[inline]
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        (*self as i64).var_encode(buf)
    }
}

impl VarEncode for isize {
    #[inline]
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        i64::try_from(*self)?.var_encode(buf)
    }
}

impl VarEncode for usize {
    #[inline]
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        u64::try_from(*self)?.var_encode(buf)
    }
}

impl<T: VarEncode> VarEncode for Vec<T> {
    #[inline]
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.len().var_encode(buf)?;
        for item in self {
            item.var_encode(buf)?;
        }
        Ok(())
    }
}

impl<T: VarEncode> VarEncode for Option<T> {
    #[inline]
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
    #[inline]
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.len().var_encode(buf)?;
        for (key, value) in self {
            key.encode(buf)?;
            value.var_encode(buf)?;
        }
        Ok(())
    }
}

#[cfg(feature = "hashbrown")]
impl<K: Encode, V: VarEncode> VarEncode for hashbrown::HashMap<K, V> {
    #[inline]
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.len().var_encode(buf)?;
        for (key, value) in self {
            key.encode(buf)?;
            value.var_encode(buf)?;
        }
        Ok(())
    }
}
