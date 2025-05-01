#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

use bevy_ecs::world::World;
use bevy_reflect::{TypeRegistry, func::ArgValue};
use glam::{
    BVec2, BVec3, BVec4, DVec2, DVec3, DVec4, I64Vec2, I64Vec3, I64Vec4, IVec2, IVec3, IVec4,
    U64Vec2, U64Vec3, U64Vec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
};

use super::ReflectArgumentParser;
use crate::argument::{ArgumentError, ArgumentParser};

/// A macro for implementing the [`ArgumentParser`] trait for glam types.
macro_rules! impl_glam {
    ($count:expr, $field:ty, $ty:ty) => {
        impl ArgumentParser for $ty {
            type Arg = Self;

            fn parse_input<'a>(
                &self,
                mut arguments: &'a str,
                _: &World,
            ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
                let mut values = [<$field>::default(); $count];
                (0..$count).try_for_each(|i| {
                    let (start, end) =
                        arguments.trim_start().split_once(' ').unwrap_or((arguments, ""));
                    values[i] = start.parse::<$field>().map_err(|_| ArgumentError::DoesNotMatch)?;
                    arguments = end;
                    Ok(())
                })?;
                Ok((ArgValue::Owned(Box::new(Self::from(values))), arguments))
            }
        }
    };
}

impl_glam!(2, bool, BVec2);
impl_glam!(3, bool, BVec3);
impl_glam!(4, bool, BVec4);

impl_glam!(2, i32, IVec2);
impl_glam!(3, i32, IVec3);
impl_glam!(4, i32, IVec4);
impl_glam!(2, i64, I64Vec2);
impl_glam!(3, i64, I64Vec3);
impl_glam!(4, i64, I64Vec4);

impl_glam!(2, u32, UVec2);
impl_glam!(3, u32, UVec3);
impl_glam!(4, u32, UVec4);
impl_glam!(2, u64, U64Vec2);
impl_glam!(3, u64, U64Vec3);
impl_glam!(4, u64, U64Vec4);

impl_glam!(2, f32, Vec2);
impl_glam!(3, f32, Vec3);
impl_glam!(4, f32, Vec4);
impl_glam!(2, f64, DVec2);
impl_glam!(3, f64, DVec3);
impl_glam!(4, f64, DVec4);

pub(super) fn register_types(registry: &mut TypeRegistry) {
    registry.register::<BVec2>();
    registry.register_type_data::<BVec2, ReflectArgumentParser>();
    registry.register::<BVec3>();
    registry.register_type_data::<BVec3, ReflectArgumentParser>();
    registry.register::<BVec4>();
    registry.register_type_data::<BVec4, ReflectArgumentParser>();

    registry.register::<IVec2>();
    registry.register_type_data::<IVec2, ReflectArgumentParser>();
    registry.register::<IVec3>();
    registry.register_type_data::<IVec3, ReflectArgumentParser>();
    registry.register::<IVec4>();
    registry.register_type_data::<IVec4, ReflectArgumentParser>();
    registry.register::<I64Vec2>();
    registry.register_type_data::<I64Vec2, ReflectArgumentParser>();
    registry.register::<I64Vec3>();
    registry.register_type_data::<I64Vec3, ReflectArgumentParser>();
    registry.register::<I64Vec4>();
    registry.register_type_data::<I64Vec4, ReflectArgumentParser>();

    registry.register::<UVec2>();
    registry.register_type_data::<UVec2, ReflectArgumentParser>();
    registry.register::<UVec3>();
    registry.register_type_data::<UVec3, ReflectArgumentParser>();
    registry.register::<UVec4>();
    registry.register_type_data::<UVec4, ReflectArgumentParser>();
    registry.register::<U64Vec2>();
    registry.register_type_data::<U64Vec2, ReflectArgumentParser>();
    registry.register::<U64Vec3>();
    registry.register_type_data::<U64Vec3, ReflectArgumentParser>();
    registry.register::<U64Vec4>();
    registry.register_type_data::<U64Vec4, ReflectArgumentParser>();

    registry.register::<Vec2>();
    registry.register_type_data::<Vec2, ReflectArgumentParser>();
    registry.register::<Vec3>();
    registry.register_type_data::<Vec3, ReflectArgumentParser>();
    registry.register::<Vec4>();
    registry.register_type_data::<Vec4, ReflectArgumentParser>();
    registry.register::<DVec2>();
    registry.register_type_data::<DVec2, ReflectArgumentParser>();
    registry.register::<DVec3>();
    registry.register_type_data::<DVec3, ReflectArgumentParser>();
    registry.register::<DVec4>();
    registry.register_type_data::<DVec4, ReflectArgumentParser>();
}
