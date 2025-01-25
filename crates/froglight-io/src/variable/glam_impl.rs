#[cfg(test)]
use std::io::Cursor;
use std::io::{Read, Write};

use glam::{
    i16::{I16Vec2, I16Vec3, I16Vec4},
    i32::{IVec2, IVec3, IVec4},
    i64::{I64Vec2, I64Vec3, I64Vec4},
    u16::{U16Vec2, U16Vec3, U16Vec4},
    u32::{UVec2, UVec3, UVec4},
    u64::{U64Vec2, U64Vec3, U64Vec4},
};
#[cfg(test)]
use proptest::prelude::*;

use super::{FrogVarRead, FrogVarWrite};
use crate::standard::{ReadError, WriteError};

macro_rules! impl_glam {
    (@step $base:ty, $index:expr,) => {};
    (@step $base:ty, $index:expr, $head:ty, $($tail:ty,)*) => {
        impl FrogVarRead for $head {
            #[inline]
            fn frog_var_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
                <[$base; $index]>::frog_var_read(buffer).map(Self::from)
            }
        }

        impl FrogVarWrite for $head {
            #[inline]
            fn frog_var_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
                <[$base; $index]>::from(*self).frog_var_write(buffer)
            }
            #[inline]
            fn frog_var_len(&self) -> usize { <[$base; $index]>::from(*self).frog_var_len() }
        }

        impl_glam!(@step $base, $index+1, $($tail,)*);
    };
    ($base:ty, $($types:ty),*) => {
        impl_glam!(@step $base, 2, $($types,)*);
    };
}

impl_glam!(u16, U16Vec2, U16Vec3, U16Vec4);
impl_glam!(i16, I16Vec2, I16Vec3, I16Vec4);
impl_glam!(u32, UVec2, UVec3, UVec4);
impl_glam!(i32, IVec2, IVec3, IVec4);
impl_glam!(u64, U64Vec2, U64Vec3, U64Vec4);
impl_glam!(i64, I64Vec2, I64Vec3, I64Vec4);

#[cfg(test)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn proto_glam_varint_u16vec2(data in proptest::array::uniform2(proptest::num::u16::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(U16Vec2::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_var_len(), buffer.len());
    }
    #[test]
    fn proto_glam_varint_u16vec3(data in proptest::array::uniform3(proptest::num::u16::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(U16Vec3::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_var_len(), buffer.len());
    }
    #[test]
    fn proto_glam_varint_u16vec4(data in proptest::array::uniform4(proptest::num::u16::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(U16Vec4::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_var_len(), buffer.len());
    }

    #[test]
    fn proto_glam_varint_i16vec2(data in proptest::array::uniform2(proptest::num::i16::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(I16Vec2::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_var_len(), buffer.len());
    }
    #[test]
    fn proto_glam_varint_i16vec3(data in proptest::array::uniform3(proptest::num::i16::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(I16Vec3::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_var_len(), buffer.len());
    }
    #[test]
    fn proto_glam_varint_i16vec4(data in proptest::array::uniform4(proptest::num::i16::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(I16Vec4::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_var_len(), buffer.len());
    }

    #[test]
    fn proto_glam_varint_uvec2(data in proptest::array::uniform2(proptest::num::u32::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(UVec2::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_var_len(), buffer.len());
    }
    #[test]
    fn proto_glam_varint_uvec3(data in proptest::array::uniform3(proptest::num::u32::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(UVec3::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_var_len(), buffer.len());
    }
    #[test]
    fn proto_glam_varint_uvec4(data in proptest::array::uniform4(proptest::num::u32::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(UVec4::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_var_len(), buffer.len());
    }

    #[test]
    fn proto_glam_varint_ivec2(data in proptest::array::uniform2(proptest::num::i32::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(IVec2::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_var_len(), buffer.len());
    }
    #[test]
    fn proto_glam_varint_ivec3(data in proptest::array::uniform3(proptest::num::i32::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(IVec3::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_var_len(), buffer.len());
    }
    #[test]
    fn proto_glam_varint_ivec4(data in proptest::array::uniform4(proptest::num::i32::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(IVec4::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_var_len(), buffer.len());
    }

    #[test]
    fn proto_glam_varint_u64vec2(data in proptest::array::uniform2(proptest::num::u64::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(U64Vec2::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_var_len(), buffer.len());
    }
    #[test]
    fn proto_glam_varint_u64vec3(data in proptest::array::uniform3(proptest::num::u64::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(U64Vec3::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_var_len(), buffer.len());
    }
    #[test]
    fn proto_glam_varint_u64vec4(data in proptest::array::uniform4(proptest::num::u64::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(U64Vec4::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_var_len(), buffer.len());
    }

    #[test]
    fn proto_glam_varint_i64vec2(data in proptest::array::uniform2(proptest::num::i64::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(I64Vec2::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_var_len(), buffer.len());
    }
    #[test]
    fn proto_glam_varint_i64vec3(data in proptest::array::uniform3(proptest::num::i64::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(I64Vec3::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_var_len(), buffer.len());
    }
    #[test]
    fn proto_glam_varint_i64vec4(data in proptest::array::uniform4(proptest::num::i64::ANY)) {
        let buffer: Vec<u8> = data.frog_to_var_buf().unwrap();
        assert_eq!(I64Vec4::frog_var_read(&mut Cursor::new(&buffer)).unwrap(), data.into());
        assert_eq!(data.frog_var_len(), buffer.len());
    }
}
