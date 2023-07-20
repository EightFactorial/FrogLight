use byteorder::{WriteBytesExt, BE};
use uuid::Uuid;

use super::{varint::VarEncode, Encode, EncodeError};

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

impl<T: Encode, const N: usize> Encode for [T; N] {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        for item in self {
            item.encode(buf)?;
        }
        Ok(())
    }
}
