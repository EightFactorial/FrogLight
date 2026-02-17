//! A player's [`Username`].
#![allow(clippy::unsafe_derive_deserialize, reason = "Triggered by deriving `facet` and `serde`")]

use alloc::string::String;
use core::{
    borrow::Borrow,
    fmt::Display,
    ops::{Deref, DerefMut},
};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use md5::{Digest, Md5, digest::Update};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use uuid::{Builder, Uuid};

/// A player's username.
///
/// The name of the player that is displayed in-game.
///
/// May be changed for things like nicknames.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect, Component))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Component))]
#[cfg_attr(feature = "bevy", reflect(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct Username(String);

impl Display for Username {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.0.fmt(f) }
}

impl Username {
    /// Create a [`Username`] from a [`String`].
    #[inline]
    #[must_use]
    pub const fn new(username: String) -> Self { Self(username) }

    /// Create a [`Username`] from a [`str`].
    #[inline]
    #[must_use]
    pub fn new_from<T: AsRef<str>>(username: T) -> Self { Self(String::from(username.as_ref())) }

    /// Convert a [`Username`] into a [`String`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> String { self.0 }

    /// Extracts a string slice containing the entire `Username`.
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &str { self.0.as_str() }

    /// Computes the offline [`Uuid`] for this username.
    ///
    /// ## Warning
    ///
    /// This is the **offline** UUID, and is only valid in offline contexts!
    ///
    /// Be very sure that this is what you need before using it.
    #[must_use]
    pub fn offline_uuid(&self) -> Uuid {
        Md5::new()
            .chain("OfflinePlayer:")
            .chain(&self.0)
            .finalize()
            .first_chunk::<16>()
            .map_or(Uuid::nil(), |&data| Builder::from_md5_bytes(data).into_uuid())
    }
}

// -------------------------------------------------------------------------------------------------

impl AsRef<String> for Username {
    #[inline]
    fn as_ref(&self) -> &String { &self.0 }
}
impl AsRef<str> for Username {
    #[inline]
    fn as_ref(&self) -> &str { self.0.as_str() }
}

impl Borrow<String> for Username {
    #[inline]
    fn borrow(&self) -> &String { &self.0 }
}
impl Borrow<str> for Username {
    #[inline]
    fn borrow(&self) -> &str { self.0.as_str() }
}

impl Deref for Username {
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for Username {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<String> for Username {
    #[inline]
    fn from(username: String) -> Self { Self(username) }
}
impl From<Username> for String {
    #[inline]
    fn from(username: Username) -> Self { username.0 }
}
