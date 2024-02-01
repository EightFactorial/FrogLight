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
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError>;

    /// Write the type to a new byte buffer
    #[inline]
    fn fg_to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.fg_write(&mut buf).expect("Failed to write into new Vec<u8>");

        buf
    }
}
