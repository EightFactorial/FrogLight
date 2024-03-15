use thiserror::Error;

/// An error that occurs when creating or using a connection.
#[derive(Debug, Error)]
pub enum ConnectionError {
    /// An I/O error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Valid address records were found, but a connection could be established.
    #[error("No connection could be established")]
    NoConnection,
    /// No `A` or `AAAA` address records were found.
    #[error("No address records were found")]
    NoAddressRecords,
}
