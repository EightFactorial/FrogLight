use froglight_protocol::io::{ReadError, WriteError};
use thiserror::Error;

/// An error that occurs when creating or using a connection.
#[derive(Debug, Error)]
pub enum ConnectionError {
    /// An I/O error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// An error occurred while reading a packet.
    #[error(transparent)]
    PacketReadError(#[from] ReadError),
    /// An error occurred while writing a packet.
    #[error(transparent)]
    PacketWriteError(#[from] WriteError),
    /// Got an unexpected packet.
    /// Valid address records were found, but a connection could be established.
    #[error("No connection could be established")]
    NoConnection,
    /// No `A` or `AAAA` address records were found.
    #[error("No address records were found")]
    NoAddressRecords,
    /// Got an unexpected packet.
    #[error("Got an unexpected packet, expected `{0}`")]
    UnexpectedPacket(&'static str),
}
