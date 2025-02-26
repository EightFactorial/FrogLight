//! TODO

use std::borrow::Cow;

use bevy_app::{App, Plugin};
use bevy_ecs::reflect::AppTypeRegistry;
use froglight_common::Identifier;
use smol_str::SmolStr;

pub mod parse;
pub use parse::*;
use uuid::Uuid;

mod integer;
mod other;
mod string;

/// A [`Plugin`] for registering argument parsers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArgumentParserPlugin;

impl Plugin for ArgumentParserPlugin {
    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        let registry = &mut *world.resource::<AppTypeRegistry>().write();

        registry.register_type_data::<String, ReflectArgumentParser>();
        registry.register_type_data::<Cow<'static, str>, ReflectArgumentParser>();

        registry.register::<SmolStr>();
        registry.register_type_data::<SmolStr, ReflectArgumentParser>();

        registry.register::<Identifier>();
        registry.register_type_data::<Identifier, ReflectArgumentParser>();

        registry.register_type_data::<bool, ReflectArgumentParser>();
        registry.register_type_data::<i8, ReflectArgumentParser>();
        registry.register_type_data::<i16, ReflectArgumentParser>();
        registry.register_type_data::<i32, ReflectArgumentParser>();
        registry.register_type_data::<i64, ReflectArgumentParser>();
        registry.register_type_data::<i128, ReflectArgumentParser>();
        registry.register_type_data::<isize, ReflectArgumentParser>();
        registry.register_type_data::<u8, ReflectArgumentParser>();
        registry.register_type_data::<u16, ReflectArgumentParser>();
        registry.register_type_data::<u32, ReflectArgumentParser>();
        registry.register_type_data::<u64, ReflectArgumentParser>();
        registry.register_type_data::<u128, ReflectArgumentParser>();
        registry.register_type_data::<usize, ReflectArgumentParser>();

        // registry.register_type_data::<f16, ReflectArgumentParser>();
        registry.register_type_data::<f32, ReflectArgumentParser>();
        registry.register_type_data::<f64, ReflectArgumentParser>();
        // registry.register_type_data::<f128, ReflectArgumentParser>();

        registry.register::<Uuid>();
        registry.register_type_data::<Uuid, ReflectArgumentParser>();
    }
}
