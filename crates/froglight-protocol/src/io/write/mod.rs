mod float;
mod integer;

/// A trait for writing a type to a buffer
pub trait FrogWrite {
    /// Write the type to the buffer
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer cannot be written to
    fn frog_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()>;

    /// Write the type to a new buffer
    fn as_byte_vec(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.frog_write(&mut buf)
            .expect("Failed to write into Vec<u8>?");
        buf
    }
}
