//! TODO

use bevy_app::{App, Plugin};
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_reflect::func::ArgValue;

mod reflect;
pub use reflect::ReflectArgumentParser;

mod brigadier_impl;
#[cfg(feature = "glam")]
mod glam_impl;
mod std_impl;

/// A [`Plugin`] for registering argument parsers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArgumentParserPlugin;

impl Plugin for ArgumentParserPlugin {
    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        let mut registry = world.resource::<AppTypeRegistry>().write();

        brigadier_impl::register_types(&mut registry);
        #[cfg(feature = "glam")]
        glam_impl::register_types(&mut registry);
        std_impl::register_types(&mut registry);

        #[cfg(feature = "uuid")]
        {
            registry.register::<uuid::Uuid>();
            registry.register_type_data::<uuid::Uuid, ReflectArgumentParser>();
        }
    }
}

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
