use core::{
    marker::PhantomData,
    sync::atomic::{AtomicBool, AtomicI32},
};
use std::sync::Arc;

use aes::{Aes128, cipher::KeyIvInit};
use cfb8::{Decryptor, Encryptor};

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
                compression: Arc::clone(&compression),
                enabled: Arc::clone(&enabled),
                decryptor: self.decryptor,
                _phantom: PhantomData,
            },
            EncryptorMut {
                connection: write,
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
    compression: Arc<AtomicI32>,
    enabled: Arc<AtomicBool>,
    encryptor: Encryptor<Aes128>,
    _phantom: PhantomData<R>,
}

/// A reference to a [`Decryptor`] that uses a specific [`Runtime`].
pub struct DecryptorMut<R: RuntimeRead<C>, C> {
    connection: C,
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

    /// Get a reference to the underlying connection.
    #[inline]
    #[must_use]
    pub const fn as_raw(&self) -> &C { &self.connection }

    /// Get a mutable reference to the underlying connection.
    #[inline]
    #[must_use]
    pub const fn as_raw_mut(&mut self) -> &mut C { &mut self.connection }

    /// Get a mutable reference to the [`Encryptor`].
    #[inline]
    #[must_use]
    pub const fn encryptor(&mut self) -> &mut Encryptor<Aes128> { &mut self.encryptor }
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

    /// Get a reference to the underlying connection.
    #[inline]
    #[must_use]
    pub const fn as_raw(&self) -> &C { &self.connection }

    /// Get a mutable reference to the underlying connection.
    #[inline]
    #[must_use]
    pub const fn as_raw_mut(&mut self) -> &mut C { &mut self.connection }

    /// Get a mutable reference to the [`Decryptor`].
    #[inline]
    #[must_use]
    pub const fn decryptor(&mut self) -> &mut Decryptor<Aes128> { &mut self.decryptor }
}
