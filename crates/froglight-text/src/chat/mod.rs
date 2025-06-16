//! Chat message signing.

use alloc::vec::Vec;
use core::time::Duration;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::version::Version;
use rsa::{RsaPrivateKey, rand_core::CryptoRng};
use uuid::Uuid;

/// A trait for [`Version`]s that can sign chat messages.
pub trait SignableMessage: Version {
    /// The unsigned message.
    type UnsignedMessage: Sized;
    /// The signed message.
    type SignedMessage: Sized;

    /// Sign a chat message.
    fn sign_message<M: MessageTimestamp>(
        message: Self::UnsignedMessage,
        salt: u64,
        timer: &M,
        ctx: &mut MessageSignatureCtx,
    ) -> Self::SignedMessage;

    /// Verify a signed chat message.
    fn verify_message<M: MessageTimestamp>(
        message: &Self::SignedMessage,
        timer: &M,
        ctx: &MessageSignatureCtx,
    ) -> bool;
}

// -------------------------------------------------------------------------------------------------

/// Context for signing chat messages.
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(from_reflect = false, PartialEq, Component))]
pub struct MessageSignatureCtx {
    account: Uuid,
    session: Uuid,
    previous: Vec<PreviousSignature>,

    #[cfg_attr(feature = "bevy", reflect(ignore))]
    private_key: RsaPrivateKey,
}

/// Signature information about a previous message.
///
/// Used to verify future messages sent by the same user.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
pub struct PreviousSignature {
    /// The timestamp of the message
    pub timestamp: Duration,
    /// The hash of the message
    pub hash: [u8; 256],
    /// The length of the message
    pub length: usize,
    /// The salt used to sign the message
    pub salt: u64,
}

impl MessageSignatureCtx {
    /// Default key size for the [`RsaPrivateKey`].
    pub const DEFAULT_KEY_SIZE: usize = 2048;

    /// Create a new [`MessageSignatureCtx`].
    ///
    /// # Errors
    /// Returns an error if the RSA key generation fails.
    pub fn new<R: CryptoRng>(
        account: Uuid,
        session: Uuid,
        rng: &mut R,
    ) -> Result<Self, rsa::Error> {
        let private_key = RsaPrivateKey::new::<R>(rng, Self::DEFAULT_KEY_SIZE)?;
        Ok(Self::new_with(account, session, private_key))
    }

    /// Create a new [`MessageSignatureCtx`] using the given salt and key.
    #[inline]
    #[must_use]
    pub fn new_with(account: Uuid, session: Uuid, private_key: RsaPrivateKey) -> Self {
        Self { account, session, private_key, previous: Vec::new() }
    }

    /// Separate out the components of this [`MessageSignatureCtx`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> (Uuid, Uuid, RsaPrivateKey) {
        (self.account, self.session, self.private_key)
    }

    /// Get information about previously signed messages.
    #[inline]
    #[must_use]
    pub const fn history(&self) -> &[PreviousSignature] { self.previous.as_slice() }

    /// Sign a message using this [`MessageSignatureCtx`].
    ///
    /// Uses the given [`CryptoRng`] implementation to
    /// generate a random salt.
    #[inline]
    #[must_use]
    pub fn sign<V: SignableMessage, M: MessageTimestamp, R: CryptoRng>(
        &mut self,
        message: V::UnsignedMessage,
        timer: &M,
        rng: &mut R,
    ) -> V::SignedMessage {
        self.sign_with_salt::<V, M>(message, timer, rng.next_u64())
    }

    /// Sign a message using this [`MessageSignatureCtx`] and the given salt.
    #[inline]
    #[must_use]
    pub fn sign_with_salt<V: SignableMessage, M: MessageTimestamp>(
        &mut self,
        message: V::UnsignedMessage,
        timer: &M,
        salt: u64,
    ) -> V::SignedMessage {
        V::sign_message::<M>(message, salt, timer, self)
    }

    /// Verify a signed message using this [`MessageSignatureCtx`].
    ///
    /// Returns `true` if the message is valid and signed by this context.
    #[inline]
    #[must_use]
    pub fn verify<V: SignableMessage, M: MessageTimestamp>(
        &self,
        message: &V::SignedMessage,
        timer: &M,
    ) -> bool {
        V::verify_message::<M>(message, timer, self)
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait for types that can provide a timestamp.
pub trait MessageTimestamp {
    /// Get the [`Duration`] from the unix epoch.
    fn timestamp(&self) -> Duration;
}
