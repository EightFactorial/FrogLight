//! TODO

use alloc::sync::Arc;
use core::{
    fmt::Debug,
    sync::atomic::{AtomicBool, AtomicI32, Ordering},
};

use aes::{
    Aes128,
    cipher::{BlockModeDecrypt, BlockModeEncrypt, InOutBuf, KeyIvInit},
};
use async_lock::Mutex;
use cfb8::{Decryptor, Encryptor};

/// The configuration used by a [`Connection`](super::Connection).
///
/// Determines whether the connection is compressed and/or encrypted.
#[derive(Debug, Clone)]
pub struct ConnConfig {
    compression: Arc<AtomicI32>,
    crypto: Arc<ConnEncryption>,
}

impl Default for ConnConfig {
    fn default() -> Self {
        Self {
            compression: Arc::new(AtomicI32::new(i32::MIN)),
            crypto: Arc::new(ConnEncryption::default()),
        }
    }
}

impl ConnConfig {
    /// Create a new [`ConnConfig`] with the default settings.
    #[inline]
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Create a new [`ConnConfig`] with custom settings.
    #[must_use]
    pub fn new_from(compression: i32, encryption: ConnEncryption) -> Self {
        Self { compression: Arc::new(AtomicI32::new(compression)), crypto: Arc::new(encryption) }
    }

    /// Get the compression threshold.
    #[inline]
    #[must_use]
    pub fn get_compression(&self) -> i32 { self.compression.load(Ordering::Relaxed) }

    /// Set the compression threshold.
    #[inline]
    pub fn set_compression(&self, threshold: i32) {
        self.compression.store(threshold, Ordering::Relaxed);
    }

    /// Get a reference to the [`ConnEncryption`].
    #[inline]
    #[must_use]
    pub fn crypto(&self) -> &ConnEncryption { &self.crypto }
}

// -------------------------------------------------------------------------------------------------

/// The encryption state of a connection.
#[derive(Default)]
pub struct ConnEncryption {
    /// Whether encryption is enabled.
    enabled: AtomicBool,

    /// The [`Encryptor`] used to encrypt packets.
    encryptor: Mutex<Option<Encryptor<Aes128>>>,
    /// The [`Decryptor`] used to decrypt packets.
    decryptor: Mutex<Option<Decryptor<Aes128>>>,
}

impl ConnEncryption {
    /// Returns `true` if encryption is enabled.
    #[must_use]
    pub fn enabled(&self) -> bool { self.enabled.load(Ordering::Relaxed) }

    /// Set the cipher key for encryption and decryption.
    ///
    /// This will immediately start using the provided key.
    pub fn set_cipher_key(&self, key: [u8; 16]) {
        let encryptor = Encryptor::new_from_slices(&key, &key)
            .unwrap_or_else(|_| unreachable!("Key is guaranteed to be the correct length"));
        let decryptor = Decryptor::new_from_slices(&key, &key)
            .unwrap_or_else(|_| unreachable!("Key is guaranteed to be the correct length"));

        self.enabled.store(true, Ordering::Relaxed);
        *self.encryptor.lock_blocking() = Some(encryptor);
        *self.decryptor.lock_blocking() = Some(decryptor);
    }

    /// Encrypts a slice of bytes into another slice.
    ///
    /// # Panics
    /// This will panic if the input and output slices
    /// are not of the same length.
    pub async fn encrypt_into(&self, input: &[u8], output: &mut [u8]) {
        if self.enabled.load(Ordering::Relaxed)
            && let Some(encryptor) = self.encryptor.lock().await.as_mut()
        {
            let (head, tail) = InOutBuf::new(input, output).unwrap().into_chunks();
            debug_assert!(tail.is_empty(), "InOutBuf tail should be empty!");
            encryptor.encrypt_blocks_inout(head);
        }
    }

    /// Encrypt a slice of bytes in-place.
    pub async fn encrypt_inplace(&self, buf: &mut [u8]) {
        if self.enabled.load(Ordering::Relaxed)
            && let Some(encryptor) = self.encryptor.lock().await.as_mut()
        {
            let (head, tail) = InOutBuf::from(buf).into_chunks();
            debug_assert!(tail.is_empty(), "InOutBuf tail should be empty!");
            encryptor.encrypt_blocks_inout(head);
        }
    }

    /// Decrypts a slice of bytes into another slice.
    ///
    /// # Panics
    /// This will panic if the input and output slices
    /// are not of the same length.
    pub async fn decrypt_into(&self, input: &[u8], output: &mut [u8]) {
        if self.enabled.load(Ordering::Relaxed)
            && let Some(decryptor) = self.decryptor.lock().await.as_mut()
        {
            let (head, tail) = InOutBuf::new(input, output).unwrap().into_chunks();
            debug_assert!(tail.is_empty(), "InOutBuf tail should be empty!");
            decryptor.decrypt_blocks_inout(head);
        }
    }

    /// Decrypt a slice of bytes in-place.
    pub async fn decrypt_inplace(&self, buf: &mut [u8]) {
        if self.enabled.load(Ordering::Relaxed)
            && let Some(decryptor) = self.decryptor.lock().await.as_mut()
        {
            let (head, tail) = InOutBuf::from(buf).into_chunks();
            debug_assert!(tail.is_empty(), "InOutBuf tail should be empty!");
            decryptor.decrypt_blocks_inout(head);
        }
    }
}

impl Debug for ConnEncryption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.enabled.load(Ordering::Relaxed) {
            f.debug_struct("ConnEncryption").field("enabled", &true).finish_non_exhaustive()
        } else {
            f.debug_struct("ConnEncryption").field("enabled", &false).finish()
        }
    }
}
