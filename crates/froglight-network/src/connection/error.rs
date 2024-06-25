use froglight_protocol::protocol::{ReadError, WriteError};
use thiserror::Error;

/// An error that occurred while connecting.
#[derive(Debug, Error)]
pub enum ConnectionError {
    /// No DNS records
    #[error("No DNS records found for `{0}`")]
    #[cfg(feature = "resolver")]
    NoRecords(String),
    /// A resolver error occurred.
    #[error(transparent)]
    #[cfg(feature = "resolver")]
    Resolver(#[from] crate::resolver::AsyncStdResolveError),
    /// Connection was closed.
    #[error("Connection was closed")]
    ConnectionClosed,
    /// Server sent an error.
    #[error("Server sent an error: `{0}`")]
    ServerError(String),
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
