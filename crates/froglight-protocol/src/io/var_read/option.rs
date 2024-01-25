use crate::io::{FrogRead, FrogVarRead, ReadError};

impl<T: FrogVarRead> FrogVarRead for Option<T> {
    fn frog_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        if bool::frog_read(buf)? {
            Ok(Some(T::frog_var_read(buf)?))
        } else {
            Ok(None)
        }
    }
}
