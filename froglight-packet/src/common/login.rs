//! TODO

use alloc::string::String;

#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
#[cfg(feature = "facet")]
use facet::Facet;
use uuid::Uuid;

/// The content of a login hello packet.
///
/// Sent to the server to initiate the login process.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet))]
pub struct LoginHelloContent {
    /// The player's username.
    pub username: String,
    /// The player's UUID.
    pub uuid: Uuid,
}

impl LoginHelloContent {
    /// Create a new [`LoginHelloContent`].
    #[inline]
    #[must_use]
    pub const fn new(username: String, uuid: Uuid) -> Self { Self { username, uuid } }

    /// Get the player's username.
    #[inline]
    #[must_use]
    pub const fn username(&self) -> &str { self.username.as_str() }

    /// Get the player's UUID.
    #[inline]
    #[must_use]
    pub const fn uuid(&self) -> Uuid { self.uuid }
}

// -------------------------------------------------------------------------------------------------

/// The content of a login complete packet.
///
/// Sent by the server to indicate the login is complete.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet))]
pub struct LoginCompleteContent {}
