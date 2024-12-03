use std::net::SocketAddr;

use compact_str::{CompactString, ToCompactString};

/// Information about a connection.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
pub struct ConnectionInformation {
    /// The address of the remote host.
    ///
    /// If the server ever asks for the address the client is connected to, this
    /// is the address that will be sent.
    pub address: Option<CompactString>,

    /// The socket address of the remote host.
    ///
    /// This is the address that the connection is established with.
    pub socket: SocketAddr,
}

impl ConnectionInformation {
    /// Create a new [`ConnectionInformation`] with the given socket address.
    ///
    /// Does not set the address.
    #[must_use]
    pub fn new(socket: SocketAddr) -> Self { Self { address: None, socket } }

    /// Set the address of the connection.
    #[must_use]
    pub fn with_address(mut self, address: impl Into<CompactString>) -> Self {
        self.address = Some(address.into());
        self
    }

    /// Gets the address of the connection.
    ///
    /// If the address is not set, the socket address is returned.
    #[must_use]
    pub fn get_address(&self) -> CompactString {
        self.address.clone().unwrap_or_else(|| self.socket.to_compact_string())
    }

    /// Gets the port of the connection.
    ///
    /// If there is no port or it cannot be parsed,
    /// the port of the socket address is returned.
    #[must_use]
    pub fn get_port(&self) -> u16 {
        self.get_address()
            .split_once(':')
            .and_then(|(_addr, port)| port.parse::<u16>().ok())
            .unwrap_or_else(|| self.socket.port())
    }
}
