use glam::{
    DVec2, DVec3, DVec4, I16Vec2, I16Vec3, I16Vec4, I64Vec2, I64Vec3, I64Vec4, IVec2, IVec3, IVec4,
    U16Vec2, U16Vec3, U16Vec4, U64Vec2, U64Vec3, U64Vec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec3A,
    Vec4,
};

use crate::protocol::{FrogWrite, WriteError};

macro_rules! impl_write_glam {
    ($ty:ty, $parts:ty) => {
        impl FrogWrite for $ty {
            #[inline]
            fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
                let mut values: $parts = bytemuck::cast(*self);
                values.iter_mut().for_each(|v| *v = v.to_be());

                buf.write_all(&bytemuck::cast::<$parts, [u8; std::mem::size_of::<$parts>()]>(
                    values,
                ))
                .map_err(WriteError::Io)
            }
        }
    };
}

impl_write_glam!(Vec2, [u32; 2]);
impl_write_glam!(Vec3, [u32; 3]);
impl_write_glam!(Vec4, [u32; 4]);
impl_write_glam!(DVec2, [u64; 2]);
impl_write_glam!(DVec3, [u64; 3]);
impl_write_glam!(DVec4, [u64; 4]);

impl_write_glam!(I16Vec2, [i16; 2]);
impl_write_glam!(I16Vec3, [i16; 3]);
impl_write_glam!(I16Vec4, [i16; 4]);
impl_write_glam!(U16Vec2, [u16; 2]);
impl_write_glam!(U16Vec3, [u16; 3]);
impl_write_glam!(U16Vec4, [u16; 4]);

impl_write_glam!(IVec2, [i32; 2]);
impl_write_glam!(IVec3, [i32; 3]);
impl_write_glam!(IVec4, [i32; 4]);
impl_write_glam!(UVec2, [u32; 2]);
impl_write_glam!(UVec3, [u32; 3]);
impl_write_glam!(UVec4, [u32; 4]);

impl_write_glam!(I64Vec2, [i64; 2]);
impl_write_glam!(I64Vec3, [i64; 3]);
impl_write_glam!(I64Vec4, [i64; 4]);
impl_write_glam!(U64Vec2, [u64; 2]);
impl_write_glam!(U64Vec3, [u64; 3]);
impl_write_glam!(U64Vec4, [u64; 4]);

impl FrogWrite for Vec3A {
    #[inline]
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        Vec3::from(*self).fg_write(buf)
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(1024))]

    #[test]
    fn proto_write_vec2(data in proptest::array::uniform2(proptest::num::f32::ANY)) {
        assert_eq!(data.fg_to_bytes(), Vec2::from(data).fg_to_bytes());
    }

    #[test]
    fn proto_write_vec3(data in proptest::array::uniform3(proptest::num::f32::ANY)) {
        assert_eq!(data.fg_to_bytes(), Vec3::from(data).fg_to_bytes());
    }

    #[test]
    fn proto_write_vec4(data in proptest::array::uniform4(proptest::num::f32::ANY)) {
        assert_eq!(data.fg_to_bytes(), Vec4::from(data).fg_to_bytes());
    }

    #[test]
    fn proto_write_dvec2(data in proptest::array::uniform2(proptest::num::f64::ANY)) {
        assert_eq!(data.fg_to_bytes(), DVec2::from(data).fg_to_bytes());
    }

    #[test]
    fn proto_write_dvec3(data in proptest::array::uniform3(proptest::num::f64::ANY)) {
        assert_eq!(data.fg_to_bytes(), DVec3::from(data).fg_to_bytes());
    }

    #[test]
    fn proto_write_dvec4(data in proptest::array::uniform4(proptest::num::f64::ANY)) {
        assert_eq!(data.fg_to_bytes(), DVec4::from(data).fg_to_bytes());
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(256))]

        #[test]
    fn proto_write_i16vec2(data in proptest::array::uniform2(proptest::num::i16::ANY)) {
        assert_eq!(data.fg_to_bytes(), I16Vec2::from(data).fg_to_bytes());
    }

    #[test]
    fn proto_write_i16vec3(data in proptest::array::uniform3(proptest::num::i16::ANY)) {
        assert_eq!(data.fg_to_bytes(), I16Vec3::from(data).fg_to_bytes());
    }

    #[test]
    fn proto_write_i16vec4(data in proptest::array::uniform4(proptest::num::i16::ANY)) {
        assert_eq!(data.fg_to_bytes(), I16Vec4::from(data).fg_to_bytes());
    }

    #[test]
    fn proto_write_u16vec2(data in proptest::array::uniform2(proptest::num::u16::ANY)) {
        assert_eq!(data.fg_to_bytes(), U16Vec2::from(data).fg_to_bytes());
    }

    #[test]
    fn proto_write_u16vec3(data in proptest::array::uniform3(proptest::num::u16::ANY)) {
        assert_eq!(data.fg_to_bytes(), U16Vec3::from(data).fg_to_bytes());
    }

    #[test]
    fn proto_write_u16vec4(data in proptest::array::uniform4(proptest::num::u16::ANY)) {
        assert_eq!(data.fg_to_bytes(), U16Vec4::from(data).fg_to_bytes());
    }

    #[test]
    fn proto_write_ivec2(data in proptest::array::uniform2(proptest::num::i32::ANY)) {
        assert_eq!(data.fg_to_bytes(), IVec2::from(data).fg_to_bytes());
    }

    #[test]
    fn proto_write_ivec3(data in proptest::array::uniform3(proptest::num::i32::ANY)) {
        assert_eq!(data.fg_to_bytes(), IVec3::from(data).fg_to_bytes());
    }

    #[test]
    fn proto_write_ivec4(data in proptest::array::uniform4(proptest::num::i32::ANY)) {
        assert_eq!(data.fg_to_bytes(), IVec4::from(data).fg_to_bytes());
    }

    #[test]
    fn proto_write_uvec2(data in proptest::array::uniform2(proptest::num::u32::ANY)) {
        assert_eq!(data.fg_to_bytes(), UVec2::from(data).fg_to_bytes());
    }

    #[test]
    fn proto_write_uvec3(data in proptest::array::uniform3(proptest::num::u32::ANY)) {
        assert_eq!(data.fg_to_bytes(), UVec3::from(data).fg_to_bytes());
    }

    #[test]
    fn proto_write_uvec4(data in proptest::array::uniform4(proptest::num::u32::ANY)) {
        assert_eq!(data.fg_to_bytes(), UVec4::from(data).fg_to_bytes());
    }

}
