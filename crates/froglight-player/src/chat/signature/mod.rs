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
    fn sign(message: Self::UnsignedMessage, ctx: &mut MessageSignatureCtx) -> Self::SignedMessage;
    /// Verify a signed chat message.
    fn verify(message: &Self::SignedMessage, ctx: &MessageSignatureCtx) -> bool;
}

// -------------------------------------------------------------------------------------------------

/// Context for signing chat messages.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(from_reflect = false, Debug, PartialEq, Component))]
pub struct MessageSignatureCtx {
    account: Uuid,
    session: Uuid,
    last_signature: [u8; 256],
    message_counter: u64,
    salt: u64,

    #[cfg_attr(feature = "bevy", reflect(ignore))]
    timestamp: SystemTime,
    #[cfg_attr(feature = "bevy", reflect(ignore))]
    private_key: RsaPrivateKey,
}

impl MessageSignatureCtx {
    /// Create a new [`MessageSignatureCtx`].
    ///
    /// # Errors
    /// Returns an error if the RSA key generation fails.
    pub fn new<R: CryptoRngCore>(
        account: Uuid,
        session: Uuid,
        rng: &mut R,
    ) -> Result<Self, rsa::Error> {
        Ok(Self {
            account,
            session,
            last_signature: [0; 256],
            message_counter: 0,
            salt: rng.next_u64(),
            timestamp: std::time::UNIX_EPOCH,
            private_key: RsaPrivateKey::new(rng, 2048)?,
        })
    }

    /// Get the number of messages signed by this [`MessageSignatureCtx`].
    #[must_use]
    pub fn messages(&self) -> u64 { self.message_counter }

    /// Get the [`SystemTime`] when the last message was signed.
    #[must_use]
    pub fn timestamp(&self) -> &SystemTime { &self.timestamp }

    /// Sign a message using this [`MessageSignatureCtx`].
    #[inline]
    pub fn sign<V: SignableMessage>(&mut self, message: V::UnsignedMessage) -> V::SignedMessage {
        V::sign(message, self)
    }

    /// Verify a signed message using this [`MessageSignatureCtx`].
    #[inline]
    pub fn verify<V: SignableMessage>(&self, message: &V::SignedMessage) -> bool {
        V::verify(message, self)
    }
}
