use super::WriteError;

mod bool;
mod float;
mod integer;
mod map;
mod option;
mod other;
mod string;
mod vector;

/// A trait for writing a type to a buffer
pub trait FrogWrite {
    /// Write the type to the buffer
    ///
    /// # Errors
    /// Returns an error if the buffer cannot be written to
    fn frog_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError>;

    /// Write the type to a new buffer
    #[inline]
    fn as_byte_vec(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.frog_write(&mut buf).expect("Failed to write into new Vec<u8>");
        buf
    }
}
