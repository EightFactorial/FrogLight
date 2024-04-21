use bevy_ecs::{entity::Entity, event::Event};
use froglight_protocol::traits::Version;

use crate::connection::ConnectionError;

/// A request to establish a connection to a server.
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct ConnectionRequest {
    /// The address of the server to connect to.
    #[cfg(feature = "resolver")]
    pub address: compact_str::CompactString,
    /// The address of the server to connect to.
    #[cfg(not(feature = "resolver"))]
    pub address: std::net::SocketAddr,

    /// The entity that requested the connection.
    pub entity: Entity,

    /// The [`Version::ID`] of the connection.
    pub version_id: i32,
}

impl ConnectionRequest {
    /// Create a new [`ConnectionRequest`] with the given address.
    #[must_use]
    #[cfg(not(feature = "resolver"))]
    pub fn new<V: Version>(address: impl Into<std::net::SocketAddr>, entity: Entity) -> Self {
        Self { address: address.into(), entity, version_id: V::ID }
    }

    /// Create a new [`ConnectionRequest`] with the given address and version
    /// ID.
    #[must_use]
    #[cfg(not(feature = "resolver"))]
    pub fn new_with_id(
        address: impl Into<std::net::SocketAddr>,
        entity: Entity,
        version_id: i32,
    ) -> Self {
        Self { address: address.into(), entity, version_id }
    }

    /// Create a new [`ConnectionRequest`] with the given address.
    #[must_use]
    #[cfg(feature = "resolver")]
    pub fn new<V: Version>(address: impl Into<compact_str::CompactString>, entity: Entity) -> Self {
        Self { address: address.into(), entity, version_id: V::ID }
    }

    /// Create a new [`ConnectionRequest`] with the given address and version
    /// ID.
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

/// A notification that a connection has been disconnected.
#[derive(Debug, Event)]
pub struct ConnectionDisconnect {
    /// The entity that was disconnected.
    pub entity: Entity,
    /// The reason for the disconnection.
    pub reason: String,
    /// The error that caused the disconnection.
    pub error: ConnectionError,
}
