#[cfg(not(feature = "std"))]
use alloc::string::String;

/// An error that can occur when converting between a Nbt and Snbt.
#[derive(Debug, thiserror::Error)]
pub enum SnbtError {
    /// Unexpected data was left over after parsing.
    #[error("Unexpected data remaining: \"{0}\"")]
    UnexpectedData(String),
    /// An error occurred while parsing content formats.
    #[error("Invalid format: expected '{0}', found '{1}'")]
    InvalidFormat(char, char),
    /// An error occurred while parsing a type from a string.
    #[error("Failed to parse type: \"{0}\"")]
    FromString(&'static str),
}
