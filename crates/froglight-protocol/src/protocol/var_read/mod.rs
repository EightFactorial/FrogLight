use super::ReadError;

mod integer;
mod map;
mod option;
mod tuple;

/// A trait for reading a variable-length type from a buffer
///
/// Uses LEB128 encoding
pub trait FrogVarRead {
    /// Read a variable-length type from a buffer
    /// # Errors
    ///
    /// Returns an error if the cursor does not contain
    /// enough data to read the type
    fn fg_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized;
}
