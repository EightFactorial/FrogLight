use super::WriteError;

mod integer;

/// A trait for writing a variable-length type to a buffer
///
/// Uses LEB128 encoding
pub trait FrogVarWrite {
    /// Write a variable-length type to a buffer
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer cannot be written to
    fn fg_var_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError>;

    /// Write a variable-length type to a new byte buffer
    #[inline]
    fn fg_var_to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.fg_var_write(&mut buf).expect("Failed to write into new Vec<u8>");

        buf
    }
}
