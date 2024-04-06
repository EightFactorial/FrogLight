use thiserror::Error;

/// An error that occurred while connecting.
#[cfg(not(feature = "resolver"))]
#[derive(Debug, Error)]
pub enum ConnectionError {
    /// An IO error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// An error that occurred while connecting.
#[cfg(feature = "resolver")]
#[derive(Debug, Error)]
pub enum ConnectionError {
    /// An IO error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// A resolver error occurred.
    #[error(transparent)]
    Resolver(#[from] crate::resolver::ResolverError),
}
