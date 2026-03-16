//! TODO

use alloc::boxed::Box;
use core::error::Error;

use bevy_reflect::{PartialReflect, func::ArgValue};

mod core_impl;
// pub use core_impl::*;

mod alloc_impl;
pub use alloc_impl::*;

#[cfg(feature = "uuid")]
mod uuid_impl;
#[cfg(feature = "uuid")]
pub use uuid_impl::*;

/// A trait for types that can be parsed from a string.
pub trait ArgumentParser: Send + Sync + Sized {
    /// Parse a value of this type from the string,
    /// returning any remaining unparsed string.
    ///
    /// # Errors
    ///
    /// TODO
    fn parse(input: &str) -> Result<(Self, &str), ArgumentParseError>;
}

/// A trait for types that can parse [`ArgValue`]s from a string.
pub trait ArgumentParserObject: Send + Sync + 'static {
    /// Parse a value from the string,
    /// returning any remaining unparsed string.
    ///
    /// # Errors
    ///
    /// TODO
    fn parse_dyn<'a>(
        &self,
        input: &'a str,
    ) -> Result<(ArgValue<'static>, &'a str), ArgumentParseError>;
}

/// An extension trait for [`ArgumentParserObject`].
pub trait ArgumentParserObjectExt: ArgumentParserObject {
    /// The type of value that this parser produces.
    type Output: PartialReflect;
}

// -------------------------------------------------------------------------------------------------

/// An error that can occur when parsing an argument from a string.
#[derive(Debug)]
pub enum ArgumentParseError {
    /// The input did not match the expected format.
    InputMismatch,
    /// Some other error occurred while parsing the argument.
    Other(Box<dyn Error + Send + Sync>),
}

impl ArgumentParseError {
    /// Create a new [`ArgumentParseError`] from any error.
    #[inline]
    #[must_use]
    pub fn other<E: Error + Send + Sync + 'static>(err: E) -> Self { Self::Other(Box::new(err)) }
}
