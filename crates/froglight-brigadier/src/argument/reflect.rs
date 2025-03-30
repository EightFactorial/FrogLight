use bevy_ecs::world::World;
use bevy_reflect::{FromType, PartialReflect, func::ArgValue};

use super::{ArgumentError, ArgumentParser};

/// Reflection data holding an [`ArgumentParser`] function.
#[derive(Clone, Copy)]
#[expect(clippy::type_complexity)]
pub struct ReflectArgumentParser {
    /// The parser function.
    parser: for<'a> fn(
        &dyn PartialReflect,
        &'a str,
        &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError>,
}

impl<T: ArgumentParser> FromType<T> for ReflectArgumentParser {
    fn from_type() -> Self {
        ReflectArgumentParser {
            parser: |reflect: &dyn PartialReflect, arguments, world| {
                let parser = reflect
                    .try_downcast_ref()
                    .expect("Invalid type provided to `ReflectArgumentParser`");
                T::parse_input(parser, arguments, world)
            },
        }
    }
}

impl ReflectArgumentParser {
    /// Parse the command arguments.
    ///
    /// # Panics
    /// Panics if the parser failed to downcast into the expected type.
    ///
    /// # Errors
    /// Returns an error if the argument failed to parse.
    #[inline]
    pub fn parse<'a>(
        &self,
        parser: &dyn PartialReflect,
        arguments: &'a str,
        world: &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
        (self.parser)(parser, arguments, world)
    }
}
