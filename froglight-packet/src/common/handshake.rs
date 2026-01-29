//! TODO

use alloc::string::{String, ToString};
use core::{
    fmt::{Debug, Display},
    net::SocketAddr,
};

#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
#[cfg(feature = "bevy")]
use bevy_reflect::std_traits::ReflectDefault;
#[cfg(feature = "facet")]
use facet::Facet;
#[cfg(feature = "facet")]
use facet_minecraft as mc;
use froglight_common::version::Version;

/// The content of a handshake packet.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet))]
pub struct HandshakeContent {
    /// The client's protocol version.
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub protocol: u32,
    /// The server's address.
    pub address: String,
    /// The server's port.
    pub port: u16,
    /// The client's intent.
    pub intent: ConnectionIntent,
}

impl HandshakeContent {
    /// Create a new [`HandshakeContent`] with the given
    /// address, port, and [`ConnectionIntent`].
    #[must_use]
    pub fn new<V: Version>(address: impl ToString, port: u16, intent: ConnectionIntent) -> Self {
        Self::new_raw(V::PROTOCOL_ID, address.to_string(), port, intent)
    }

    /// Create a new [`HandshakeContent`] with the given
    /// [`SocketAddr`] and [`ConnectionIntent`].
    #[inline]
    #[must_use]
    pub fn new_socket<V: Version>(socket: SocketAddr, intent: ConnectionIntent) -> Self {
        Self::new::<V>(socket.ip(), socket.port(), intent)
    }

    /// Create a new [`HandshakeContent`] with the given values.
    #[must_use]
    pub const fn new_raw(
        protocol: u32,
        address: String,
        port: u16,
        intent: ConnectionIntent,
    ) -> Self {
        Self { protocol, address, port, intent }
    }
}

// -------------------------------------------------------------------------------------------------

/// The intention of a client connecting to a server.
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet))]
pub enum ConnectionIntent {
    /// The client wants the status of the server.
    Status = 1,
    /// The client wants to login to the server.
    #[default]
    Login = 2,
    /// The client is being transferred from another server.
    Transfer = 3,
}

impl Display for ConnectionIntent {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { Debug::fmt(self, f) }
}
