use froglight_protocol::io::{ReadError, WriteError};
use thiserror::Error;

use crate::resolver::ResolverError;

/// An error that occurs when creating or using a connection.
#[derive(Debug, Error)]
pub enum ConnectionError {
    /// Connection was closed.
    #[error("Connection was closed")]
    ConnectionClosed,
    /// An I/O error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// An error occurred while resolving an address.
    #[error(transparent)]
    ResolverError(#[from] ResolverError),
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
