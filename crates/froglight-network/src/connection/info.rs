use std::net::SocketAddr;

use compact_str::CompactString;

/// Information about a connection.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConnectionInfo {
    /// The address of the server.
    pub address: Option<CompactString>,
    /// The ip and port of the server.
    pub socket: SocketAddr,
}
