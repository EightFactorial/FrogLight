use core::{error, fmt};

/// An error that occurs when creating an [`Identifier`].
#[derive(Debug, Clone, Copy)]
pub enum IdentifierError {
    /// The string has no namespace separator (`:`).
    RequiresNamespace,
    /// The string either starts with, ends with,
    /// or contains more than one namespace separator (`:`).
    Invalid,
}

impl IdentifierError {
    /// Returns a static string describing the error.
    #[must_use]
    pub const fn describe(&self) -> &'static str {
        match self {
            IdentifierError::RequiresNamespace => "identifier requires a namespace",
            IdentifierError::Invalid => "invalid identifier",
        }
    }
}

impl fmt::Display for IdentifierError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.describe()) }
}

impl error::Error for IdentifierError {}
