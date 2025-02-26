use std::borrow::Cow;

use bevy_reflect::TypeRegistry;
use froglight_common::Identifier;
use smol_str::SmolStr;

use super::ReflectArgumentParser;

mod integer;
mod other;
mod string;

pub(super) fn register_types(registry: &mut TypeRegistry) {
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

    registry.register_type_data::<String, ReflectArgumentParser>();

    registry.register::<Cow<'static, str>>();
    registry.register_type_data::<Cow<'static, str>, ReflectArgumentParser>();

    registry.register::<SmolStr>();
    registry.register_type_data::<SmolStr, ReflectArgumentParser>();

    registry.register::<Identifier>();
    registry.register_type_data::<Identifier, ReflectArgumentParser>();
}
