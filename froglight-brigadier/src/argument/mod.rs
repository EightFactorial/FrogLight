//! TODO

use alloc::{borrow::Cow, boxed::Box};
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
    fn parse<'a>(
        input: &'a str,
        data: &Self::Data,
    ) -> Result<(Self, &'a str), ArgumentParseError<'a>>;
}

// -------------------------------------------------------------------------------------------------

/// An error that can occur when parsing an argument from a string.
#[derive(Debug)]
pub enum ArgumentParseError<'a> {
    /// The input did not match the expected format.
    InputMismatch,
    /// The input did not meet required constraints
    /// (e.g. a number was out of range).
    InputInvalid,
    /// The input contained extra data after the last argument.
    ExtraInput(Cow<'a, str>),

    /// Some other unknown error occurred.
    Unknown,
    /// Some other error occurred while parsing the argument.
    Other(Box<dyn Error + Send + Sync>),
}

impl ArgumentParseError<'_> {
    /// Create a new [`ArgumentParseError`] from any error.
    #[inline]
    #[must_use]
    pub fn other<E: Error + Send + Sync + 'static>(err: E) -> Self { Self::Other(Box::new(err)) }

    /// Take ownership of the error,
    /// converting any borrowed data into owned data.
    #[must_use]
    pub fn into_owned(self) -> ArgumentParseError<'static> {
        use ArgumentParseError::{ExtraInput, InputInvalid, InputMismatch, Other, Unknown};

        match self {
            InputMismatch => InputMismatch,
            InputInvalid => InputInvalid,
            ExtraInput(input) => ExtraInput(Cow::Owned(input.into_owned())),
            Unknown => Unknown,
            Other(err) => Other(err),
        }
    }
}

impl Error for ArgumentParseError<'_> {}
impl Display for ArgumentParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgumentParseError::InputMismatch => {
                write!(f, "input did not match expected format")
            }
            ArgumentParseError::InputInvalid => {
                write!(f, "input did not meet required constraints")
            }
            ArgumentParseError::ExtraInput(input) => {
                write!(f, "input contained extra data: \"{input}\"")
            }
            ArgumentParseError::Unknown => {
                write!(f, "an unknown error occurred")
            }
            ArgumentParseError::Other(err) => Display::fmt(err, f),
        }
    }
}
