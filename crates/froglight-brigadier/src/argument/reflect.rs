use bevy_ecs::world::World;
use bevy_reflect::{FromType, func::ArgValue};

use super::{ArgumentError, ArgumentParser};

/// Reflection data holding an [`ArgumentParser`] function.
#[derive(Clone, Copy)]
#[expect(clippy::type_complexity)]
pub struct ReflectArgumentParser {
    /// The parser function.
    parser: for<'a> fn(&'a str, &World) -> Result<(ArgValue<'a>, &'a str), ArgumentError>,
}
impl<T: ArgumentParser> FromType<T> for ReflectArgumentParser {
    fn from_type() -> Self { ReflectArgumentParser { parser: T::parse_input } }
}

impl ReflectArgumentParser {
    /// Parse the command arguments.
    ///
    /// # Errors
    /// Returns an error if the argument failed to parse.
    #[inline]
    pub fn parse<'a>(
        self,
        arguments: &'a str,
        world: &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
        (self.parser)(arguments, world)
    }
}
