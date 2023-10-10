use super::{Encode, EncodeError, VarEncode};

pub trait FromValue: Sized {
    fn from_value<T: Encode>(value: &T) -> Result<Self, EncodeError>;
    fn from_var_value<T: VarEncode>(value: &T) -> Result<Self, EncodeError>;
}

impl FromValue for Vec<u8> {
    fn from_value<T: Encode>(value: &T) -> Result<Self, EncodeError> {
        let mut buffer = Self::new();
        value.encode(&mut buffer)?;
        Ok(buffer)
    }

    fn from_var_value<T: VarEncode>(value: &T) -> Result<Self, EncodeError> {
        let mut buffer = Self::new();
        value.var_encode(&mut buffer)?;
        Ok(buffer)
    }
}

impl<const N: usize> FromValue for [u8; N] {
    fn from_value<T: Encode>(value: &T) -> Result<Self, EncodeError> {
        let mut vec = Vec::with_capacity(N);
        value.encode(&mut vec)?;

        vec.resize(N, 0u8);
        Ok(vec.try_into().expect("array.len() == N"))
    }

    fn from_var_value<T: VarEncode>(value: &T) -> Result<Self, EncodeError> {
        let mut vec = Vec::with_capacity(N);
        value.var_encode(&mut vec)?;

        vec.resize(N, 0u8);
        Ok(vec.try_into().expect("array.len() == N"))
    }
}
