use alloc::{string::String, vec::Vec};
use core::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    result::Result,
};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use foldhash::fast::RandomState;
use froglight_common::digest::{HexDigest, Sha1, ShaDigest};
use indexmap::IndexMap;
use rsa::{
    RsaPrivateKey, RsaPublicKey,
    pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey},
};
use uuid::Uuid;

/// A player's profile.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Component))]
pub struct PlayerProfile {
    /// The player's username.
    pub username: String,
    /// The player's UUID.
    pub uuid: Uuid,

    /// The player's skin URL.
    pub skin: Option<String>,
    /// The player's cape URL.
    pub cape: Option<String>,

    /// The player's properties.
    #[cfg_attr(feature = "bevy", reflect(ignore))]
    pub properties: IndexMap<String, ProfileProperty, RandomState>,
}

/// A property of a [`PlayerProfile`].
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProfileProperty(serde_json::Value);

impl ProfileProperty {
    /// Create a new [`ProfileProperty`] from a [`serde_json::Value`].
    #[inline]
    #[must_use]
    pub const fn new(value: serde_json::Value) -> Self { Self(value) }

    /// Return the inner [`serde_json::Value`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> serde_json::Value { self.0 }
}

impl Deref for ProfileProperty {
    type Target = serde_json::Value;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for ProfileProperty {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<serde_json::Value> for ProfileProperty {
    #[inline]
    fn from(value: serde_json::Value) -> Self { Self::new(value) }
}
impl From<ProfileProperty> for serde_json::Value {
    #[inline]
    fn from(property: ProfileProperty) -> Self { property.into_inner() }
}

// -------------------------------------------------------------------------------------------------

/// The API's public keys.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Resource, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, Resource, opaque))]
pub struct ApiPublicKeys {
    /// A list of keys for verifying profile properties.
    pub profile: Vec<RsaPublicKey>,
    /// A list of keys for verifying player certificates.
    pub certificate: Vec<RsaPublicKey>,
}

// -------------------------------------------------------------------------------------------------

/// A unique identifier for a server.
///
/// Used by the API to identify servers.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Resource, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, Resource))]
pub struct ServerId(String);

impl ServerId {
    /// Create a [`ServerId`] from a base server ID (typically an empty string)
    /// and the server's public and private keys.
    ///
    /// # Errors
    ///
    /// TODO
    pub fn new(base_id: &str, keys: &RsaPrivateKey) -> Result<Self, rsa::Error> {
        let private = keys.to_pkcs1_der()?;
        let public = keys.to_public_key().to_pkcs1_der()?;

        Ok(Self::new_from_raw(base_id, private.as_bytes(), public.as_bytes()))
    }

    /// Create a [`ServerId`] from a base server ID (typically an empty string)
    /// and the server's public and private keys.
    #[must_use]
    pub fn new_from_raw(base_id: &str, private: &[u8], public: &[u8]) -> Self {
        let mut hasher = Sha1::new();
        hasher.update(base_id);
        hasher.update(private);
        hasher.update(public);

        Self(HexDigest::digest_using(hasher))
    }

    /// Get a reference to the inner string.
    #[must_use]
    pub const fn as_str(&self) -> &str { self.0.as_str() }

    /// Consume the [`ServerId`] and return the inner string.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> String { self.0 }
}

impl Debug for ServerId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { Debug::fmt(&self.0, f) }
}
impl Display for ServerId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Deref for ServerId {
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
