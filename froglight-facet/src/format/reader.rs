//! TODO

/// A `no_std`-compatible reader.
pub struct Reader<'a> {
    read: usize,
    remaining: &'a [u8],
}

impl<'a> Reader<'a> {
    /// Create a new [`Reader`] for the given slice.
    #[inline]
    #[must_use]
    pub const fn new(slice: &'a [u8]) -> Self { Self { read: 0, remaining: slice } }

    /// Get the total number of bytes read so far.
    #[inline]
    #[must_use]
    pub const fn total_read(&self) -> usize { self.read }

    /// Get the remaining bytes in the reader.
    #[inline]
    #[must_use]
    pub const fn remaining(&self) -> &'a [u8] { self.remaining }

    /// Get the next `len` bytes from the reader.
    ///
    /// # Errors
    ///
    /// Returns an error if the reader doesn't have enough bytes remaining.
    pub fn get(&mut self, len: usize) -> Result<&'a [u8], ReaderError> {
        if self.remaining.len() < len {
            Err(ReaderError::EndOfInput(len - self.remaining.len()))
        } else {
            let (result, remaining) = self.remaining.split_at(len);
            self.read += len;
            self.remaining = remaining;
            Ok(result)
        }
    }

    /// Get the next `N` bytes from the reader.
    ///
    /// # Errors
    ///
    /// Returns an error if the reader doesn't have enough bytes remaining.
    pub fn get_array<const N: usize>(&mut self) -> Result<&'a [u8; N], ReaderError> {
        if let Some((result, remaining)) = self.remaining.split_first_chunk::<N>() {
            self.read += N;
            self.remaining = remaining;
            Ok(result)
        } else {
            Err(ReaderError::EndOfInput(N - self.remaining.len()))
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Errors that can occur while using a [`Reader`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReaderError {
    /// The [`Reader`] reached the end but expected more data.
    EndOfInput(usize),
}
