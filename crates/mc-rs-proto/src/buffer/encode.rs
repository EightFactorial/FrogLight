use std::collections::HashMap;

use azalea_chat::FormattedText;
use azalea_nbt::Nbt;
use byteorder::{WriteBytesExt, BE};
use compact_str::CompactString;
use smallvec::SmallVec;
use uuid::Uuid;

use super::{Encode, EncodeError, VarEncode};

impl Encode for bool {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_u8(*self as u8).map_err(EncodeError::from)
    }
}

impl Encode for i8 {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_i8(*self).map_err(EncodeError::from)
    }
}

impl Encode for u8 {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_u8(*self).map_err(EncodeError::from)
    }
}

impl Encode for i16 {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_i16::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for u16 {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_u16::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for i32 {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_i32::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for u32 {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_u32::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for i64 {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_i64::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for u64 {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_u64::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for i128 {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_i128::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for u128 {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_u128::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for isize {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        i64::try_from(*self)?.encode(buf)
    }
}

impl Encode for usize {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        u64::try_from(*self)?.encode(buf)
    }
}

impl Encode for f32 {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_f32::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for f64 {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_f64::<BE>(*self).map_err(EncodeError::from)
    }
}

impl Encode for String {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.len().var_encode(buf)?;
        buf.write_all(self.as_bytes()).map_err(EncodeError::from)
    }
}

impl Encode for &str {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.len().var_encode(buf)?;
        buf.write_all(self.as_bytes()).map_err(EncodeError::from)
    }
}

impl Encode for CompactString {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.len().var_encode(buf)?;
        buf.write_all(self.as_bytes()).map_err(EncodeError::from)
    }
}

impl Encode for Uuid {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.as_u128().encode(buf)
    }
}

impl<T: Encode> Encode for Vec<T> {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.len().var_encode(buf)?;
        for item in self {
            item.encode(buf)?;
        }
        Ok(())
    }
}

impl<T: Encode, const N: usize> Encode for SmallVec<[T; N]> {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.len().var_encode(buf)?;
        for item in self {
            item.encode(buf)?;
        }
        Ok(())
    }
}

impl<T: Encode, const N: usize> Encode for [T; N] {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        for item in self {
            item.encode(buf)?;
        }
        Ok(())
    }
}

impl<T: Encode> Encode for Option<T> {
    #[inline]
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
    #[inline]
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
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.len().var_encode(buf)?;
        for (key, value) in self {
            key.encode(buf)?;
            value.encode(buf)?;
        }
        Ok(())
    }
}

impl Encode for Nbt {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.write(buf);
        Ok(())
    }
}

impl Encode for FormattedText {
    #[inline]
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        serde_json::to_string(self)?.encode(buf)
    }
}
