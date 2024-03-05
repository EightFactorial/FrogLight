use thiserror::Error;

/// An error that occurs when creating or using a connection.
#[derive(Debug, Error)]
pub enum ConnectionError {
    /// An I/O error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
