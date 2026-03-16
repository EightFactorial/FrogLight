//! TODO

use alloc::boxed::Box;
use core::{
    error::Error,
    fmt::{self, Display},
};

mod alloc_impl;
pub use alloc_impl::*;

mod core_impl;

#[cfg(feature = "glam")]
mod glam_impl;

#[cfg(feature = "uuid")]
mod uuid_impl;
#[cfg(feature = "uuid")]
pub use uuid_impl::*;

/// A trait for arguments that can be parsed from a string.
pub trait ArgumentParser: Sized + 'static {
    /// Data required to parse the argument from a string.
    type Data: Clone + Send + Sync + Sized + 'static;

    /// Parse the argument from an input string and some parser data.
    ///
    /// # Errors
    ///
    /// Returns an error if the input string could not be parsed.
    fn parse<'a>(input: &'a str, data: &Self::Data) -> Result<(Self, &'a str), ArgumentParseError>;
}

// -------------------------------------------------------------------------------------------------

/// An error that can occur when parsing an argument from a string.
#[derive(Debug)]
pub enum ArgumentParseError {
    /// The input did not match the expected format.
    InputMismatch,
    /// The input did not meet required constraints
    /// (e.g. a number was out of range).
    InputInvalid,
    /// Some other error occurred while parsing the argument.
    Other(Box<dyn Error + Send + Sync>),
}

impl ArgumentParseError {
    /// Create a new [`ArgumentParseError`] from any error.
    #[inline]
    #[must_use]
    pub fn other<E: Error + Send + Sync + 'static>(err: E) -> Self { Self::Other(Box::new(err)) }
}

impl Error for ArgumentParseError {}
impl Display for ArgumentParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgumentParseError::InputMismatch => {
                write!(f, "input did not match expected format")
            }
            ArgumentParseError::InputInvalid => {
                write!(f, "input did not meet required constraints")
            }
            ArgumentParseError::Other(err) => Display::fmt(err, f),
        }
    }
}
