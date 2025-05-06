//! Chat message signing.

use core::time::Duration;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::version::Version;
use rsa::{RsaPrivateKey, rand_core::CryptoRngCore};
use uuid::Uuid;

/// A trait for [`Version`]s that can sign chat messages.
pub trait SignableMessage: Version {
    /// The unsigned message.
    type UnsignedMessage: Sized;
    /// The signed message.
    type SignedMessage: Sized;

    /// Sign a chat message.
    fn sign_message(
        message: Self::UnsignedMessage,
        ctx: &mut MessageSignatureCtx,
    ) -> Self::SignedMessage;

    /// Verify a signed chat message.
    fn verify_message(message: &Self::SignedMessage, ctx: &MessageSignatureCtx) -> bool;
}

// -------------------------------------------------------------------------------------------------

/// Context for signing chat messages.
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(from_reflect = false, PartialEq, Component))]
pub struct MessageSignatureCtx {
    account: Uuid,
    session: Uuid,
    salt: u64,

    timestamp: Duration,
    previous: Vec<(usize, [u8; 256])>,

    #[cfg_attr(feature = "bevy", reflect(ignore))]
    private_key: RsaPrivateKey,
}

impl MessageSignatureCtx {
    /// Default key size for the [`RsaPrivateKey`].
    pub const DEFAULT_KEY_SIZE: usize = 2048;

    /// Create a new [`MessageSignatureCtx`].
    ///
    /// # Errors
    /// Returns an error if the RSA key generation fails.
    pub fn new<R: CryptoRngCore>(
        account: Uuid,
        session: Uuid,
        salt: Option<u64>,
        rng: &mut R,
    ) -> Result<Self, rsa::Error> {
        let private_key = RsaPrivateKey::new(rng, Self::DEFAULT_KEY_SIZE)?;
        Ok(Self::new_with(account, session, salt.unwrap_or_else(|| rng.next_u64()), private_key))
    }

    /// Create a new [`MessageSignatureCtx`] using the given salt and key.
    #[inline]
    #[must_use]
    pub fn new_with(account: Uuid, session: Uuid, salt: u64, private_key: RsaPrivateKey) -> Self {
        Self {
            account,
            session,
            salt,
            private_key,
            previous: Vec::new(),
            timestamp: Duration::ZERO,
        }
    }

    /// Get the number of messages previously signed by this
    /// [`MessageSignatureCtx`].
    #[inline]
    #[must_use]
    pub const fn message_index(&self) -> usize { self.previous.len() }

    /// Get the lengths and signatures of all previous messages signed by this
    /// [`MessageSignatureCtx`].
    #[inline]
    #[must_use]
    pub const fn messages(&self) -> &[(usize, [u8; 256])] { self.previous.as_slice() }

    /// Get the [`Duration`] from the unix epoch when the last message was
    /// signed.
    #[inline]
    #[must_use]
    pub const fn timestamp(&self) -> &Duration { &self.timestamp }

    /// Separate out the components of this [`MessageSignatureCtx`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> (Uuid, Uuid, u64, RsaPrivateKey) {
        (self.account, self.session, self.salt, self.private_key)
    }

    /// Sign a message using this [`MessageSignatureCtx`].
    #[inline]
    #[must_use]
    pub fn sign<V: SignableMessage>(&mut self, message: V::UnsignedMessage) -> V::SignedMessage {
        V::sign_message(message, self)
    }

    /// Verify a signed message using this [`MessageSignatureCtx`].
    ///
    /// Returns `true` if the message is valid and signed by this context.
    #[inline]
    #[must_use]
    pub fn verify<V: SignableMessage>(&self, message: &V::SignedMessage) -> bool {
        V::verify_message(message, self)
    }
}
