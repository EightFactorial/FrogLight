//! TODO

use bevy_ecs::world::World;
use bevy_reflect::{FromType, func::ArgValue};

/// A trait for parsing arguments from a string.
pub trait ArgumentParser: 'static {
    /// The type of argument to parse.
    type Arg: Sized;
    /// Parse the string for an argument,
    /// returning the remaining string and the argument.
    ///
    /// # Errors
    /// Returns an error if the argument is invalid.
    fn parse_input<'a>(
        arguments: &'a str,
        world: &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError>;
}

/// Reflection data holding a parser function.
#[derive(Clone, Copy)]
#[expect(clippy::type_complexity)]
pub(crate) struct ReflectArgumentParser {
    /// The parser function.
    parser: for<'a> fn(&'a str, &World) -> Result<(ArgValue<'a>, &'a str), ArgumentError>,
}

impl ReflectArgumentParser {
    /// Parse the command arguments.
    #[inline]
    pub(crate) fn parse<'a>(
        self,
        arguments: &'a str,
        world: &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
        (self.parser)(arguments, world)
    }
}
impl<T: ArgumentParser> FromType<T> for ReflectArgumentParser {
    fn from_type() -> Self { ReflectArgumentParser { parser: T::parse_input } }
}

/// An error that occurred while parsing an argument.
#[derive(Debug, thiserror::Error)]
pub enum ArgumentError {
    /// The argument does not match the expected type.
    #[error("Argument does not match expected type")]
    DoesNotMatch,

    /// An invalid argument was provided.
    #[error("Invalid argument at position {0}")]
    InvalidArgument(usize),
}
