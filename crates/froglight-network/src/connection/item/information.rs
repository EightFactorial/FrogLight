use std::net::SocketAddr;

use compact_str::CompactString;

/// Information about a connection.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConnectionInformation {
    /// The address of the remote host.
    pub address: Option<CompactString>,
    /// The socket address of the remote host.
    pub socket: SocketAddr,
}
