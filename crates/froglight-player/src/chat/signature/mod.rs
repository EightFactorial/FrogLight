//! Chat message signing.

use std::time::SystemTime;

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
    type UnsignedMessage: Clone + Sized;
    /// The signed message.
    type SignedMessage: Clone + Sized;

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
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(from_reflect = false, Debug, PartialEq, Component))]
pub struct MessageSignatureCtx {
    account: Uuid,
    session: Uuid,
    message_counter: u64,
    last_signature: [u8; 256],
    salt: u64,

    #[cfg_attr(feature = "bevy", reflect(ignore))]
    timestamp: SystemTime,
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
        rng: &mut R,
    ) -> Result<Self, rsa::Error> {
        let private_key = RsaPrivateKey::new(rng, Self::DEFAULT_KEY_SIZE)?;
        Ok(Self::new_with(account, session, rng.next_u64(), private_key))
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
            message_counter: 0,
            last_signature: [0; 256],
            timestamp: std::time::UNIX_EPOCH,
        }
    }

    /// Get the number of messages signed by this [`MessageSignatureCtx`].
    #[inline]
    #[must_use]
    pub const fn messages(&self) -> u64 { self.message_counter }

    /// Get the [`SystemTime`] when the last message was signed.
    #[inline]
    #[must_use]
    pub const fn timestamp(&self) -> &SystemTime { &self.timestamp }

    /// Sign a message using this [`MessageSignatureCtx`].
    #[inline]
    pub fn sign<V: SignableMessage>(&mut self, message: V::UnsignedMessage) -> V::SignedMessage {
        V::sign_message(message, self)
    }

    /// Verify a signed message using this [`MessageSignatureCtx`].
    #[inline]
    pub fn verify<V: SignableMessage>(&self, message: &V::SignedMessage) -> bool {
        V::verify_message(message, self)
    }
}
