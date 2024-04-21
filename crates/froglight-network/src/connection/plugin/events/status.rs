use std::time::Duration;

use bevy_ecs::{entity::Entity, event::Event};
use froglight_protocol::{packet::ServerStatus, traits::Version};

/// A request to check the status of a server.
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct StatusRequest {
    /// The address of the server to check the status of.
    #[cfg(feature = "resolver")]
    pub address: compact_str::CompactString,
    /// The address of the server to check the status of.
    #[cfg(not(feature = "resolver"))]
    pub address: std::net::SocketAddr,

    /// The entity that requested the status.
    pub entity: Entity,

    /// The [`Version::ID`] of the connection.
    pub version_id: i32,
}

impl StatusRequest {
    /// Create a new [`StatusRequest`] with the given address.
    #[must_use]
    #[cfg(not(feature = "resolver"))]
    pub fn new<V: Version>(address: impl Into<std::net::SocketAddr>, entity: Entity) -> Self {
        Self { address: address.into(), entity, version_id: V::ID }
    }

    /// Create a new [`StatusRequest`] with the given address and version ID.
    #[must_use]
    #[cfg(not(feature = "resolver"))]
    pub fn new_with_id(
        address: impl Into<std::net::SocketAddr>,
        entity: Entity,
        version_id: i32,
    ) -> Self {
        Self { address: address.into(), entity, version_id }
    }

    /// Create a new [`StatusRequest`] with the given address.
    #[must_use]
    #[cfg(feature = "resolver")]
    pub fn new<V: Version>(address: impl Into<compact_str::CompactString>, entity: Entity) -> Self {
        Self { address: address.into(), entity, version_id: V::ID }
    }

    /// Create a new [`StatusRequest`] with the given address and version ID.
    #[must_use]
    #[cfg(feature = "resolver")]
    pub fn new_with_id(
        address: impl Into<compact_str::CompactString>,
        entity: Entity,
        version_id: i32,
    ) -> Self {
        Self { address: address.into(), entity, version_id }
    }

    /// Check if the request is for the given version.
    #[must_use]
    pub fn is_version<V: Version>(&self) -> bool { self.version_id == V::ID }
}

/// A response to a [`StatusRequest`] event.
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct StatusResponse {
    /// The status of the server.
    pub status: ServerStatus,
    /// How long the server took to respond.
    pub duration: Duration,
}

impl StatusResponse {
    /// Create a new [`StatusResponse`] with the given status and duration.
    #[must_use]
    pub const fn new(status: ServerStatus, duration: Duration) -> Self { Self { status, duration } }
}
