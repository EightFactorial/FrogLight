use glam::{
    I16Vec2, I16Vec3, I16Vec4, I64Vec2, I64Vec3, I64Vec4, IVec2, IVec3, IVec4, U16Vec2, U16Vec3,
    U16Vec4, U64Vec2, U64Vec3, U64Vec4, UVec2, UVec3, UVec4,
};

use crate::protocol::{FrogVarRead, ReadError};

macro_rules! impl_var_read {
    (vec2 $($name:ident),*) => {
        $(
            impl FrogVarRead for $name {
                #[inline]
                fn fg_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
                where
                    Self: Sized,
                {
                    Ok(Self::new(
                        FrogVarRead::fg_var_read(buf)?,
                        FrogVarRead::fg_var_read(buf)?,
                    ))
                }
            }

        )*
    };
    (vec3 $($name:ident),*) => {
        $(
            impl FrogVarRead for $name {
                #[inline]
                fn fg_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
                where
                    Self: Sized,
                {
                    Ok(Self::new(
                        FrogVarRead::fg_var_read(buf)?,
                        FrogVarRead::fg_var_read(buf)?,
                        FrogVarRead::fg_var_read(buf)?,
                    ))
                }
            }

        )*
    };
    (vec4 $($name:ident),*) => {
        $(
            impl FrogVarRead for $name {
                #[inline]
                fn fg_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
                where
                    Self: Sized,
                {
                    Ok(Self::new(
                        FrogVarRead::fg_var_read(buf)?,
                        FrogVarRead::fg_var_read(buf)?,
                        FrogVarRead::fg_var_read(buf)?,
                        FrogVarRead::fg_var_read(buf)?,
                    ))
                }
            }

        )*
    }
}

impl_var_read!(vec2 I16Vec2, IVec2, I64Vec2, U16Vec2, UVec2, U64Vec2);
impl_var_read!(vec3 I16Vec3, IVec3, I64Vec3, U16Vec3, UVec3, U64Vec3);
impl_var_read!(vec4 I16Vec4, IVec4, I64Vec4, U16Vec4, UVec4, U64Vec4);
