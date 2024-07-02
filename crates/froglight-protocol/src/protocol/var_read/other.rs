use std::io::Cursor;

use simdnbt::owned::Nbt;

use super::FrogVarRead;
use crate::protocol::ReadError;

impl FrogVarRead for Nbt {
    #[inline]
    fn fg_var_read(buf: &mut Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        simdnbt::owned::read_unnamed(buf).map_err(ReadError::from)
    }
}
