//! TODO

use alloc::{boxed::Box, string::String};
use core::{error::Error, fmt::Debug};

use bevy_reflect::{PartialReflect, func::ArgValue};

mod numeric;
pub mod string;

/// An argument for a [`GameCommand`](crate::prelude::GameCommand).
pub trait CommandArgument: Debug + Default + Clone + Send + Sync + 'static {
    /// The actual type of the argument.
    type Output: PartialReflect + Sized + 'static;

    /// A parser for the argument.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is invalid for this argument type.
    fn parse_argument<'a>(
        &self,
        input: &'a str,
    ) -> Result<(Self::Output, &'a str), ArgumentParseError>;
}

/// A dyn-compatible [`CommandArgument`].
pub trait CommandArgumentDyn: Send + Sync + 'static {
    /// A parser for the argument.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is invalid for this argument type.
    fn parse_value<'a>(
        &self,
        input: &'a str,
    ) -> Result<(ArgValue<'static>, &'a str), ArgumentParseError>;

    /// Clone this argument as a trait object.
    fn dyn_clone(&self) -> Box<dyn CommandArgumentDyn>;
}

impl<T: CommandArgument> CommandArgumentDyn for T {
    fn parse_value<'a>(
        &self,
        input: &'a str,
    ) -> Result<(ArgValue<'static>, &'a str), ArgumentParseError> {
        self.parse_argument(input).map(|(s, rest)| (ArgValue::Owned(Box::new(s)), rest))
    }

    fn dyn_clone(&self) -> Box<dyn CommandArgumentDyn> { Box::new(self.clone()) }
}

/// An error that can occur when parsing an argument.
#[derive(Debug)]
pub enum ArgumentParseError {
    /// Not necessarily an error,
    /// but indicates that the input was not valid for this argument.
    InputMismatch,
    /// Invalid input for this argument type.
    InputInvalid(String),
    /// Some other error occurred while parsing the argument.
    Other(Box<dyn Error + Sync + Send>),
}

impl ArgumentParseError {
    /// Create a [`ArgumentParseError::Other`] from an error.
    #[inline]
    #[must_use]
    pub fn other<E: Error + Sync + Send + 'static>(err: E) -> Self { Self::Other(Box::new(err)) }
}
