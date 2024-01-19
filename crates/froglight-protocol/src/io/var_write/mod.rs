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
    fn frog_var_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()>;
}
