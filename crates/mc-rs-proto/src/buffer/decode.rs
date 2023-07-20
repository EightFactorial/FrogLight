use byteorder::{ReadBytesExt, BE};
use uuid::Uuid;

use super::{varint::VarDecode, Decode, DecodeError};

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

        match len > MAX_STRING_LENGTH {
            true => Err(DecodeError::StringTooLong(len)),
            false => {
                let mut vec = vec![0; len as usize];
                buf.read_exact(&mut vec)?;

                Ok(String::from_utf8(vec)?)
            }
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

impl<T: Decode> Decode for Option<T> {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        match bool::decode(buf)? {
            true => Ok(Some(T::decode(buf)?)),
            false => Ok(None),
        }
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
