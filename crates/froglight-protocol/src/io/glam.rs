use std::io::{Cursor, Write};

use bevy_math::{
    DVec2, DVec3, DVec4, I16Vec2, I16Vec3, I16Vec4, I64Vec2, I64Vec3, I64Vec4, IVec2, IVec3, IVec4,
    U16Vec2, U16Vec3, U16Vec4, U64Vec2, U64Vec3, U64Vec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec3A,
    Vec4,
};

use super::{FrogRead, FrogVarRead, FrogVarWrite, FrogWrite, ReadError, WriteError};

macro_rules! impl_vec2 {
    ($($name:ident),*) => {
        $(
            impl FrogRead for $name {
                #[inline]
                fn fg_read(buf: &mut Cursor<&[u8]>) -> Result<Self, ReadError>
                where
                    Self: Sized,
                {
                    Ok(Self::new(
                        FrogRead::fg_read(buf)?,
                        FrogRead::fg_read(buf)?,
                    ))
                }
            }
            impl FrogWrite for $name {
                #[inline]
                fn fg_write(&self, buf: &mut (impl Write + ?Sized)) -> Result<(), WriteError> {
                    self.x.fg_write(buf)?;
                    self.y.fg_write(buf)
                }
            }
        )*
    };
    (var $($name:ident),*) => {
        $(
            impl FrogVarRead for $name {
                #[inline]
                fn fg_var_read(buf: &mut Cursor<&[u8]>) -> Result<Self, ReadError>
                where
                    Self: Sized,
                {
                    Ok(Self::new(
                        FrogVarRead::fg_var_read(buf)?,
                        FrogVarRead::fg_var_read(buf)?,
                    ))
                }
            }
            impl FrogVarWrite for $name {
                #[inline]
                fn fg_var_write(&self, buf: &mut (impl Write + ?Sized)) -> Result<(), WriteError> {
                    self.x.fg_var_write(buf)?;
                    self.y.fg_var_write(buf)
                }
            }
        )*
    };
}
impl_vec2!(Vec2, DVec2, I16Vec2, IVec2, I64Vec2, U16Vec2, UVec2, U64Vec2);
impl_vec2!(var I16Vec2, IVec2, I64Vec2, U16Vec2, UVec2, U64Vec2);

macro_rules! impl_vec3 {
    ($($name:ident),*) => {
        $(
            impl FrogRead for $name {
                #[inline]
                fn fg_read(buf: &mut Cursor<&[u8]>) -> Result<Self, ReadError>
                where
                    Self: Sized,
                {
                    Ok(Self::new(
                        FrogRead::fg_read(buf)?,
                        FrogRead::fg_read(buf)?,
                        FrogRead::fg_read(buf)?,
                    ))
                }
            }
            impl FrogWrite for $name {
                #[inline]
                fn fg_write(&self, buf: &mut (impl Write + ?Sized)) -> Result<(), WriteError> {
                    self.x.fg_write(buf)?;
                    self.y.fg_write(buf)?;
                    self.z.fg_write(buf)
                }
            }
        )*
    };
    (var $($name:ident),*) => {
        $(
            impl FrogVarRead for $name {
                #[inline]
                fn fg_var_read(buf: &mut Cursor<&[u8]>) -> Result<Self, ReadError>
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
            impl FrogVarWrite for $name {
                #[inline]
                fn fg_var_write(&self, buf: &mut (impl Write + ?Sized)) -> Result<(), WriteError> {
                    self.x.fg_var_write(buf)?;
                    self.y.fg_var_write(buf)?;
                    self.z.fg_var_write(buf)
                }
            }
        )*
    };
}
impl_vec3!(Vec3, Vec3A, DVec3, I16Vec3, IVec3, I64Vec3, U16Vec3, UVec3, U64Vec3);
impl_vec3!(var I16Vec3, IVec3, I64Vec3, U16Vec3, UVec3, U64Vec3);

macro_rules! impl_vec4 {
    ($($name:ident),*) => {
        $(
            impl FrogRead for $name {
                #[inline]
                fn fg_read(buf: &mut Cursor<&[u8]>) -> Result<Self, ReadError>
                where
                    Self: Sized,
                {
                    Ok(Self::new(
                        FrogRead::fg_read(buf)?,
                        FrogRead::fg_read(buf)?,
                        FrogRead::fg_read(buf)?,
                        FrogRead::fg_read(buf)?,
                    ))
                }
            }
            impl FrogWrite for $name {
                #[inline]
                fn fg_write(&self, buf: &mut (impl Write + ?Sized)) -> Result<(), WriteError> {
                    self.x.fg_write(buf)?;
                    self.y.fg_write(buf)?;
                    self.z.fg_write(buf)?;
                    self.w.fg_write(buf)
                }
            }

        )*
    };
    (var $($name:ident),*) => {
        $(
            impl FrogVarRead for $name {
                #[inline]
                fn fg_var_read(buf: &mut Cursor<&[u8]>) -> Result<Self, ReadError>
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
            impl FrogVarWrite for $name {
                #[inline]
                fn fg_var_write(&self, buf: &mut (impl Write + ?Sized)) -> Result<(), WriteError> {
                    self.x.fg_var_write(buf)?;
                    self.y.fg_var_write(buf)?;
                    self.z.fg_var_write(buf)?;
                    self.w.fg_var_write(buf)
                }
            }
        )*
    }
}
impl_vec4!(Vec4, DVec4, I16Vec4, IVec4, I64Vec4, U16Vec4, UVec4, U64Vec4);
impl_vec4!(var I16Vec4, IVec4, I64Vec4, U16Vec4, UVec4, U64Vec4);
