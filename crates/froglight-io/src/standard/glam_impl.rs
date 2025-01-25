#[cfg(test)]
use std::io::Cursor;
use std::io::{Read, Write};

use glam::{
    bool::{BVec2, BVec3, BVec4},
    f32::{Vec2, Vec3, Vec4},
    f64::{DVec2, DVec3, DVec4},
    i16::{I16Vec2, I16Vec3, I16Vec4},
    i32::{IVec2, IVec3, IVec4},
    i64::{I64Vec2, I64Vec3, I64Vec4},
    i8::{I8Vec2, I8Vec3, I8Vec4},
    u16::{U16Vec2, U16Vec3, U16Vec4},
    u32::{UVec2, UVec3, UVec4},
    u64::{U64Vec2, U64Vec3, U64Vec4},
    u8::{U8Vec2, U8Vec3, U8Vec4},
};
#[cfg(test)]
use proptest::prelude::*;

use super::{FrogRead, FrogWrite, ReadError, WriteError};

macro_rules! impl_glam {
    (@step $base:ty, $index:expr,) => {};
    (@step $base:ty, $index:expr, $head:ty, $($tail:ty,)*) => {
        impl FrogRead for $head {
            #[inline]
            fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
                <[$base; $index]>::frog_read(buffer).map(Self::from)
            }
        }

        impl FrogWrite for $head {
            #[inline]
            fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
                <[$base; $index]>::from(*self).frog_write(buffer)
            }
            #[inline]
            fn frog_len(&self) -> usize { std::mem::size_of::<[$base; $index]>() }
        }

        impl_glam!(@step $base, $index+1, $($tail,)*);
    };
    ($base:ty, $($types:ty),*) => {
        impl_glam!(@step $base, 2, $($types,)*);
    };
}

impl_glam!(bool, BVec2, BVec3, BVec4);
impl_glam!(u8, U8Vec2, U8Vec3, U8Vec4);
impl_glam!(i8, I8Vec2, I8Vec3, I8Vec4);
impl_glam!(u16, U16Vec2, U16Vec3, U16Vec4);
impl_glam!(i16, I16Vec2, I16Vec3, I16Vec4);
impl_glam!(u32, UVec2, UVec3, UVec4);
impl_glam!(i32, IVec2, IVec3, IVec4);
impl_glam!(u64, U64Vec2, U64Vec3, U64Vec4);
impl_glam!(i64, I64Vec2, I64Vec3, I64Vec4);
impl_glam!(f32, Vec2, Vec3, Vec4);
impl_glam!(f64, DVec2, DVec3, DVec4);

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn proto_glam_bvec2(data in proptest::array::uniform2(proptest::bool::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(BVec2::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_bvec3(data in proptest::array::uniform3(proptest::bool::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(BVec3::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_bvec4(data in proptest::array::uniform4(proptest::bool::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(BVec4::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_glam_u8vec2(data in proptest::array::uniform2(proptest::num::u8::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(U8Vec2::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_u8vec3(data in proptest::array::uniform3(proptest::num::u8::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(U8Vec3::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_u8vec4(data in proptest::array::uniform4(proptest::num::u8::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(U8Vec4::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_glam_i8vec2(data in proptest::array::uniform2(proptest::num::i8::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(I8Vec2::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_i8vec3(data in proptest::array::uniform3(proptest::num::i8::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(I8Vec3::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_i8vec4(data in proptest::array::uniform4(proptest::num::i8::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(I8Vec4::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_glam_u16vec2(data in proptest::array::uniform2(proptest::num::u16::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(U16Vec2::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_u16vec3(data in proptest::array::uniform3(proptest::num::u16::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(U16Vec3::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_u16vec4(data in proptest::array::uniform4(proptest::num::u16::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(U16Vec4::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_glam_i16vec2(data in proptest::array::uniform2(proptest::num::i16::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(I16Vec2::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_i16vec3(data in proptest::array::uniform3(proptest::num::i16::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(I16Vec3::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_i16vec4(data in proptest::array::uniform4(proptest::num::i16::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(I16Vec4::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_glam_uvec2(data in proptest::array::uniform2(proptest::num::u32::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(UVec2::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_uvec3(data in proptest::array::uniform3(proptest::num::u32::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(UVec3::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_uvec4(data in proptest::array::uniform4(proptest::num::u32::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(UVec4::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_glam_ivec2(data in proptest::array::uniform2(proptest::num::i32::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(IVec2::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_ivec3(data in proptest::array::uniform3(proptest::num::i32::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(IVec3::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_ivec4(data in proptest::array::uniform4(proptest::num::i32::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(IVec4::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_glam_u64vec2(data in proptest::array::uniform2(proptest::num::u64::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(U64Vec2::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_u64vec3(data in proptest::array::uniform3(proptest::num::u64::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(U64Vec3::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_u64vec4(data in proptest::array::uniform4(proptest::num::u64::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(U64Vec4::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_glam_i64vec2(data in proptest::array::uniform2(proptest::num::i64::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(I64Vec2::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_i64vec3(data in proptest::array::uniform3(proptest::num::i64::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(I64Vec3::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_i64vec4(data in proptest::array::uniform4(proptest::num::i64::ANY)) {
        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(I64Vec4::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_glam_vec2(data in proptest::array::uniform2(proptest::num::f32::ANY)) {
        if data.iter().any(|x| x.is_nan()) {
            return Ok(());
        }

        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec2::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_vec3(data in proptest::array::uniform3(proptest::num::f32::ANY)) {
        if data.iter().any(|x| x.is_nan()) {
            return Ok(());
        }

        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec3::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_vec4(data in proptest::array::uniform4(proptest::num::f32::ANY)) {
        if data.iter().any(|x| x.is_nan()) {
            return Ok(());
        }


        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(Vec4::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }

    #[test]
    fn proto_glam_dvec2(data in proptest::array::uniform2(proptest::num::f64::ANY)) {
        if data.iter().any(|x| x.is_nan()) {
            return Ok(());
        }

        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(DVec2::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_dvec3(data in proptest::array::uniform3(proptest::num::f64::ANY)) {
        if data.iter().any(|x| x.is_nan()) {
            return Ok(());
        }

        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(DVec3::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }
    #[test]
    fn proto_glam_dvec4(data in proptest::array::uniform4(proptest::num::f64::ANY)) {
        if data.iter().any(|x| x.is_nan()) {
            return Ok(());
        }

        let buffer: Vec<u8> = data.frog_to_buf().unwrap();
        assert_eq!(DVec4::frog_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_len(), buffer.len());
    }

}
