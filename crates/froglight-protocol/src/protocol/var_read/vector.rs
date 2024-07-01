use crate::protocol::{FrogVarRead, ReadError};

impl<T: FrogVarRead, const N: usize> FrogVarRead for [T; N] {
    #[inline]
    fn fg_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        core::array::try_from_fn(|i| {
            T::fg_var_read(buf).map_err(|err| ReadError::ListError(N, i, Box::new(err)))
        })
    }
}

impl<T: FrogVarRead> FrogVarRead for Vec<T> {
    fn fg_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        let len = u32::fg_var_read(buf)? as usize;

        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::fg_var_read(buf)?);
        }

        Ok(vec)
    }
}

#[cfg(feature = "smallvec")]
impl<T: FrogVarRead, const N: usize> FrogVarRead for smallvec::SmallVec<[T; N]> {
    fn fg_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        let len = u32::fg_var_read(buf)? as usize;

        let mut vec = smallvec::SmallVec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::fg_var_read(buf)?);
        }

        Ok(vec)
    }
}
