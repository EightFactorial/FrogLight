use core::{
    marker::PhantomData,
    sync::atomic::{AtomicBool, AtomicI32, Ordering},
};
use std::sync::Arc;

use aes::{
    Aes128,
    cipher::{BlockModeDecrypt, BlockModeEncrypt, InOutBuf, KeyIvInit},
};
#[cfg(feature = "futures-lite")]
use async_compression::futures::bufread::{ZlibDecoder, ZlibEncoder};
use cfb8::{Decryptor, Encryptor};
#[cfg(feature = "futures-lite")]
use futures_lite::{AsyncReadExt, io::Cursor};

use crate::connection::{Runtime, RuntimeRead, RuntimeWrite};

/// An encrypted connection that uses a specific [`Runtime`].
pub struct Encrypted<R: Runtime<C>, C: Send> {
    connection: C,
    compression: AtomicI32,
    enabled: AtomicBool,
    encryptor: Encryptor<Aes128>,
    decryptor: Decryptor<Aes128>,
    _phantom: PhantomData<R>,
}

impl<R: Runtime<C>, C: Send> Encrypted<R, C> {
    /// Create a new [`Encrypted`] connection.
    ///
    /// Has encryption disabled by default.
    #[must_use]
    pub fn new(connection: C) -> Self {
        Self {
            connection,
            compression: AtomicI32::new(i32::MIN),
            enabled: AtomicBool::new(false),
            encryptor: Encryptor::new(&[0; _].into(), &[0; _].into()),
            decryptor: Decryptor::new(&[0; _].into(), &[0; _].into()),
            _phantom: PhantomData,
        }
    }

    /// Change the [`Runtime`] of this [`Encrypted`] connection.
    #[inline]
    #[must_use]
    pub fn with_runtime<R2: Runtime<C>>(self) -> Encrypted<R2, C> {
        Encrypted {
            connection: self.connection,
            compression: self.compression,
            enabled: self.enabled,
            encryptor: self.encryptor,
            decryptor: self.decryptor,
            _phantom: PhantomData,
        }
    }

    /// Get a reference to the compression threshold.
    #[inline]
    #[must_use]
    pub const fn compression(&self) -> &AtomicI32 { &self.compression }

    /// Get a reference to whether encryption is enabled.
    #[inline]
    #[must_use]
    pub const fn enabled(&self) -> &AtomicBool { &self.enabled }

    /// Get a reference to the underlying raw connection.
    #[inline]
    #[must_use]
    pub const fn as_raw(&self) -> &C { &self.connection }

    /// Get a mutable reference to the underlying raw connection.
    #[inline]
    #[must_use]
    pub const fn as_raw_mut(&mut self) -> &mut C { &mut self.connection }

    /// Get a mutable reference to the [`Encryptor`].
    #[inline]
    #[must_use]
    pub const fn encryptor(&mut self) -> &mut Encryptor<Aes128> { &mut self.encryptor }

    /// Get a mutable reference to the [`Decryptor`].
    #[inline]
    #[must_use]
    pub const fn decryptor(&mut self) -> &mut Decryptor<Aes128> { &mut self.decryptor }

    /// Split this [`Encrypted`] connection into an [`EncryptorMut`] and a
    /// [`DecryptorMut`].
    #[must_use]
    pub fn into_split(self) -> (DecryptorMut<R, R::Read>, EncryptorMut<R, R::Write>) {
        let (read, write) = R::into_split(self.connection);
        let compression = Arc::new(self.compression);
        let enabled = Arc::new(self.enabled);

        (
            DecryptorMut {
                connection: read,
                #[cfg(feature = "futures-lite")]
                scratch: Vec::new(),
                compression: Arc::clone(&compression),
                enabled: Arc::clone(&enabled),
                decryptor: self.decryptor,
                _phantom: PhantomData,
            },
            EncryptorMut {
                connection: write,
                #[cfg(feature = "futures-lite")]
                scratch: Vec::new(),
                compression,
                enabled,
                encryptor: self.encryptor,
                _phantom: PhantomData,
            },
        )
    }
}

// -------------------------------------------------------------------------------------------------

/// A reference to an [`Encryptor`] that uses a specific [`Runtime`].
pub struct EncryptorMut<R: RuntimeWrite<C>, C: Send> {
    connection: C,
    #[cfg(feature = "futures-lite")]
    scratch: Vec<u8>,
    compression: Arc<AtomicI32>,
    enabled: Arc<AtomicBool>,
    encryptor: Encryptor<Aes128>,
    _phantom: PhantomData<R>,
}

/// A reference to a [`Decryptor`] that uses a specific [`Runtime`].
pub struct DecryptorMut<R: RuntimeRead<C>, C: Send> {
    connection: C,
    #[cfg(feature = "futures-lite")]
    scratch: Vec<u8>,
    compression: Arc<AtomicI32>,
    enabled: Arc<AtomicBool>,
    decryptor: Decryptor<Aes128>,
    _phantom: PhantomData<R>,
}

impl<R: RuntimeWrite<C>, C: Send> EncryptorMut<R, C> {
    /// Get a reference to the compression threshold.
    #[inline]
    #[must_use]
    pub fn compression(&self) -> &AtomicI32 { &self.compression }

    /// Get a reference to whether encryption is enabled.
    #[inline]
    #[must_use]
    pub fn enabled(&self) -> &AtomicBool { &self.enabled }

    /// Get a reference to the underlying raw connection.
    #[inline]
    #[must_use]
    pub const fn as_raw(&self) -> &C { &self.connection }

    /// Get a mutable reference to the underlying raw connection.
    #[inline]
    #[must_use]
    pub const fn as_raw_mut(&mut self) -> &mut C { &mut self.connection }

    /// Get a mutable reference to the [`Encryptor`].
    #[inline]
    #[must_use]
    pub const fn encryptor(&mut self) -> &mut Encryptor<Aes128> { &mut self.encryptor }

    /// Writes all bytes from `buf` to the underlying connection.
    ///
    /// If encryption is enabled, the data will be encrypted in-place.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the underlying connection fails.
    pub async fn write_all(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        if self.enabled.load(Ordering::Relaxed) {
            let (head, tail) = InOutBuf::from(&mut *buf).into_chunks();
            debug_assert!(tail.is_empty(), "InOutBuf tail should be empty!");
            self.encryptor.encrypt_blocks_inout(head);
        }
        R::write_all(&mut self.connection, buf).await
    }

    /// Compresses `buf` if compression is enabled and the length of `buf`
    /// is greater than the compression threshold.
    ///
    /// Also adds a length prefix if compression is enabled.
    ///
    /// # Errors
    ///
    /// Returns an error if compression fails.
    #[cfg(feature = "futures-lite")]
    pub async fn compress<'a>(&'a mut self, buf: &'a mut [u8]) -> std::io::Result<&'a mut [u8]> {
        let threshold = self.compression().load(Ordering::Relaxed);
        if threshold.is_positive() {
            self.scratch.clear();

            let prefix = if threshold <= buf.len().try_into().unwrap_or(i32::MAX) {
                // Compress the buffer and write it to the scratch space.
                let mut compressor = ZlibEncoder::new(Cursor::new(buf));
                compressor.read_to_end(&mut self.scratch).await?
            } else {
                // No compression, copy the buffer to the scratch space.
                self.scratch.extend_from_slice(buf);
                0
            };

            // Add the length prefix.
            write_slice_prefix(prefix, &mut self.scratch);

            Ok(self.scratch.as_mut_slice())
        } else {
            Ok(buf)
        }
    }
}

impl<R: RuntimeRead<C>, C: Send> DecryptorMut<R, C> {
    /// Get a reference to the compression threshold.
    #[inline]
    #[must_use]
    pub fn compression(&self) -> &AtomicI32 { &self.compression }

    /// Get a reference to whether encryption is enabled.
    #[inline]
    #[must_use]
    pub fn enabled(&self) -> &AtomicBool { &self.enabled }

    /// Get a reference to the underlying raw connection.
    #[inline]
    #[must_use]
    pub const fn as_raw(&self) -> &C { &self.connection }

    /// Get a mutable reference to the underlying raw connection.
    #[inline]
    #[must_use]
    pub const fn as_raw_mut(&mut self) -> &mut C { &mut self.connection }

    /// Get a mutable reference to the [`Decryptor`].
    #[inline]
    #[must_use]
    pub const fn decryptor(&mut self) -> &mut Decryptor<Aes128> { &mut self.decryptor }

    /// Reads the exact number of bytes required to fill `buf`.
    ///
    /// If encryption is enabled, the data will be decrypted in-place.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the underlying connection fails.
    pub async fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        R::read_exact(&mut self.connection, buf).await?;
        if self.enabled.load(Ordering::Relaxed) {
            let (head, tail) = InOutBuf::from(buf).into_chunks();
            debug_assert!(tail.is_empty(), "InOutBuf tail should be empty!");
            self.decryptor.decrypt_blocks_inout(head);
        }
        Ok(())
    }

    /// Decompresses `buf` if decompression is enabled and the length of `buf`
    /// is greater than the compression threshold.
    ///
    /// Also removes the length prefix if compression is enabled.
    ///
    /// # Errors
    ///
    /// Returns an error there is no length prefix, or if decompression fails.
    #[cfg(feature = "futures-lite")]
    pub async fn decompress<'a>(&'a mut self, mut buf: &'a [u8]) -> std::io::Result<&'a [u8]> {
        let threshold = self.compression().load(Ordering::Relaxed);
        if threshold.is_positive() {
            // Remove the length prefix from the buffer.
            if buf.first().is_some_and(|&b| b == 0) {
                buf = &buf[1..];
            } else {
                buf = read_prefixed_slice(buf).ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Failed to read prefixed slice for decompression",
                    )
                })?;
            }

            // Decompress if the buffer length exceeds the threshold.
            if threshold <= buf.len().try_into().unwrap_or(i32::MAX) {
                self.scratch.clear();
                let mut decompressor = ZlibDecoder::new(Cursor::new(buf));
                decompressor.read_to_end(&mut self.scratch).await?;
                return Ok(self.scratch.as_slice());
            }
        }

        Ok(buf)
    }
}

/// Reads a length-prefixed slice from the given buffer.
#[must_use]
#[cfg(feature = "futures-lite")]
fn read_prefixed_slice(buf: &[u8]) -> Option<&[u8]> {
    let mut byte: u8;
    let mut number = 0usize;
    let mut index = 0;
    while index < 5 {
        byte = *buf.get(index)?;
        number |= usize::from(byte & 0b0111_1111) << (7 * index);
        if byte & 0b1000_0000 != 0 {
            index += 1;
        } else {
            break;
        }
    }
    buf.get(index..index + number)
}

/// Writes a length prefix into the given buffer.
#[cfg(feature = "futures-lite")]
#[allow(clippy::cast_possible_truncation, reason = "Bitwise operations")]
pub(crate) fn write_slice_prefix(mut prefix: usize, buf: &mut Vec<u8>) {
    let mut count: usize = 0;
    let mut byte = [0];
    while (prefix != 0 || count == 0) && count < 5 {
        byte[0] = (prefix & 0b0111_1111) as u8;
        prefix = (prefix >> 7) & (usize::MAX >> 6);
        if prefix != 0 {
            byte[0] |= 0b1000_0000;
        }

        count += 1;
        buf.push(byte[0]);
    }
    buf.rotate_right(count);
}
