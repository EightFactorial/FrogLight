use std::net::SocketAddr;

use compact_str::{CompactString, ToCompactString};

/// Information about a connection.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    /// Gets the address of the connection.
    ///
    /// If the address is not set, the socket address is returned.
    #[must_use]
    pub fn get_address(&self) -> CompactString {
        self.address.clone().unwrap_or_else(|| self.socket.to_compact_string())
    }

    /// Gets the port of the connection.
    ///
    /// If the port cannot be parsed, `25565` is returned.
    #[must_use]
    pub fn get_port(&self) -> u16 {
        self.get_address().split_once(':').and_then(|(_, port)| port.parse().ok()).unwrap_or(25565)
    }
}
