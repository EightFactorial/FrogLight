mod float;
mod integer;

/// A trait for reading a type from a buffer
pub trait FrogRead {
    /// Read a type from a cursor
    ///
    /// # Errors
    ///
    /// Returns an error if the cursor does not contain
    /// enough data to read the type
    fn frog_read(buf: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self>
    where
        Self: Sized;
}
