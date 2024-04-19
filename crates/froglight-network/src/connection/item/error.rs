use compact_str::CompactString;
use froglight_protocol::protocol::{ReadError, WriteError};
use thiserror::Error;

/// An error that occurred while connecting.
#[cfg(not(feature = "resolver"))]
#[derive(Debug, Error)]
pub enum ConnectionError {
    /// Connection was closed.
    #[error("Connection was closed")]
    ConnectionClosed,
    /// An IO error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// An error occurred while reading a packet.
    #[error(transparent)]
    PacketReadError(#[from] ReadError),
    /// An error occurred while writing a packet.
    #[error(transparent)]
    PacketWriteError(#[from] WriteError),
    /// Got an unexpected packet.
    #[error("Got an unexpected packet, expected `{0}`")]
    UnexpectedPacket(&'static str),
}

/// An error that occurred while connecting.
#[cfg(feature = "resolver")]
#[derive(Debug, Error)]
pub enum ConnectionError {
    /// No DNS records
    #[error("No DNS records found for `{0}`")]
    NoRecords(CompactString),
    /// Connection was closed.
    #[error("Connection was closed")]
    ConnectionClosed,
    /// An IO error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// A resolver error occurred.
    #[error(transparent)]
    Resolver(#[from] crate::resolver::ResolverError),
    /// An error occurred while reading a packet.
    #[error(transparent)]
    PacketReadError(#[from] ReadError),
    /// An error occurred while writing a packet.
    #[error(transparent)]
    PacketWriteError(#[from] WriteError),
    /// Got an unexpected packet.
    #[error("Got an unexpected packet, expected `{0}`")]
    UnexpectedPacket(&'static str),
}
