use core::{
    marker::PhantomData,
    sync::atomic::{AtomicBool, AtomicI32},
};
use std::sync::{Arc, atomic::Ordering};

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
pub struct Encrypted<R: Runtime<C>, C> {
    connection: C,
    compression: AtomicI32,
    enabled: AtomicBool,
    encryptor: Encryptor<Aes128>,
    decryptor: Decryptor<Aes128>,
    _phantom: PhantomData<R>,
}

impl<R: Runtime<C>, C> Encrypted<R, C> {
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
pub struct EncryptorMut<R: RuntimeWrite<C>, C> {
    connection: C,
    #[cfg(feature = "futures-lite")]
    scratch: Vec<u8>,
    compression: Arc<AtomicI32>,
    enabled: Arc<AtomicBool>,
    encryptor: Encryptor<Aes128>,
    _phantom: PhantomData<R>,
}

/// A reference to a [`Decryptor`] that uses a specific [`Runtime`].
pub struct DecryptorMut<R: RuntimeRead<C>, C> {
    connection: C,
    #[cfg(feature = "futures-lite")]
    scratch: Vec<u8>,
    compression: Arc<AtomicI32>,
    enabled: Arc<AtomicBool>,
    decryptor: Decryptor<Aes128>,
    _phantom: PhantomData<R>,
}

impl<R: RuntimeWrite<C>, C> EncryptorMut<R, C> {
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

    /// Compresses `buf` if its length is greater than the compression
    /// threshold.
    ///
    /// # Errors
    ///
    /// Returns an error if compression fails.
    #[cfg(feature = "futures-lite")]
    #[expect(clippy::cast_sign_loss, reason = "Checked if positive before casting")]
    pub async fn compress<'a>(&'a mut self, buf: &'a [u8]) -> std::io::Result<&'a [u8]> {
        let threshold = self.compression().load(Ordering::Relaxed);
        if threshold.is_positive() && threshold as usize <= buf.len() {
            self.scratch.clear();
            let mut compressor = ZlibEncoder::new(Cursor::new(buf));
            let len = compressor.read_to_end(&mut self.scratch).await?;
            Ok(&self.scratch[..len])
        } else {
            Ok(buf)
        }
    }
}

impl<R: RuntimeRead<C>, C> DecryptorMut<R, C> {
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

    /// Decompresses `buf` if its length is greater than the compression
    /// threshold.
    ///
    /// # Errors
    ///
    /// Returns an error if decompression fails.
    #[cfg(feature = "futures-lite")]
    #[expect(clippy::cast_sign_loss, reason = "Checked if positive before casting")]
    pub async fn decompress<'a>(&'a mut self, buf: &'a [u8]) -> std::io::Result<&'a [u8]> {
        let threshold = self.compression().load(Ordering::Relaxed);
        if threshold.is_positive() && threshold as usize <= buf.len() {
            self.scratch.clear();
            let mut decompressor = ZlibDecoder::new(Cursor::new(buf));
            let len = decompressor.read_to_end(&mut self.scratch).await?;
            Ok(&self.scratch[..len])
        } else {
            Ok(buf)
        }
    }
}
