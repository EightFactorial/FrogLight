use glam::{
    I16Vec2, I16Vec3, I16Vec4, I64Vec2, I64Vec3, I64Vec4, IVec2, IVec3, IVec4, U16Vec2, U16Vec3,
    U16Vec4, U64Vec2, U64Vec3, U64Vec4, UVec2, UVec3, UVec4,
};

use crate::protocol::{FrogVarWrite, WriteError};

macro_rules! impl_var_write {
    (vec2 $($name:ident),*) => {
        $(
            impl FrogVarWrite for $name {
                #[inline]
                fn fg_var_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
                    self.x.fg_var_write(buf)?;
                    self.y.fg_var_write(buf)
                }
            }
        )*
    };
    (vec3 $($name:ident),*) => {
        $(
            impl FrogVarWrite for $name {
                #[inline]
                fn fg_var_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
                    self.x.fg_var_write(buf)?;
                    self.y.fg_var_write(buf)?;
                    self.z.fg_var_write(buf)
                }
            }
        )*
    };
    (vec4 $($name:ident),*) => {
        $(
            impl FrogVarWrite for $name {
                #[inline]
                fn fg_var_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
                    self.x.fg_var_write(buf)?;
                    self.y.fg_var_write(buf)?;
                    self.z.fg_var_write(buf)?;
                    self.w.fg_var_write(buf)
                }
            }
        )*
    }
}

impl_var_write!(vec2 I16Vec2, IVec2, I64Vec2, U16Vec2, UVec2, U64Vec2);
impl_var_write!(vec3 I16Vec3, IVec3, I64Vec3, U16Vec3, UVec3, U64Vec3);
impl_var_write!(vec4 I16Vec4, IVec4, I64Vec4, U16Vec4, UVec4, U64Vec4);
