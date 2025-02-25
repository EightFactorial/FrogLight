//! TODO

use bevy_ecs::world::World;
use bevy_reflect::func::ArgValue;

mod reflect;
pub use reflect::ReflectArgumentParser;

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
