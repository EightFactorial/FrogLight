use std::io::BufRead;

use glam::{
    DVec2, DVec3, DVec4, I16Vec2, I16Vec3, I16Vec4, I64Vec2, I64Vec3, I64Vec4, IVec2, IVec3, IVec4,
    U16Vec2, U16Vec3, U16Vec4, U64Vec2, U64Vec3, U64Vec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec3A,
    Vec4,
};

use crate::protocol::{FrogRead, ReadError};

macro_rules! impl_read_glam {
    ($ty:ty, $parts:ty) => {
        impl FrogRead for $ty {
            fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError> {
                let position = usize::try_from(buf.position()).expect("Cursor position too large");
                buf.consume(std::mem::size_of::<$ty>());

                if let Some(slice) =
                    &buf.get_ref().get(position..position + std::mem::size_of::<$ty>())
                {
                    let mut values: $parts = bytemuck::pod_read_unaligned(slice);
                    values.iter_mut().for_each(|v| *v = v.to_be());
                    Ok(bytemuck::must_cast(values))
                } else {
                    Err(ReadError::EndOfBuffer(
                        std::mem::size_of::<$ty>(),
                        buf.get_ref().len() - position,
                    ))
                }
            }
        }
    };
}

impl_read_glam!(Vec2, [u32; 2]);
impl_read_glam!(Vec3, [u32; 3]);
impl_read_glam!(Vec4, [u32; 4]);
impl_read_glam!(DVec2, [u64; 2]);
impl_read_glam!(DVec3, [u64; 3]);
impl_read_glam!(DVec4, [u64; 4]);

impl_read_glam!(I16Vec2, [i16; 2]);
impl_read_glam!(I16Vec3, [i16; 3]);
impl_read_glam!(I16Vec4, [i16; 4]);
impl_read_glam!(U16Vec2, [u16; 2]);
impl_read_glam!(U16Vec3, [u16; 3]);
impl_read_glam!(U16Vec4, [u16; 4]);

impl_read_glam!(IVec2, [i32; 2]);
impl_read_glam!(IVec3, [i32; 3]);
impl_read_glam!(IVec4, [i32; 4]);
impl_read_glam!(UVec2, [u32; 2]);
impl_read_glam!(UVec3, [u32; 3]);
impl_read_glam!(UVec4, [u32; 4]);

impl_read_glam!(I64Vec2, [i64; 2]);
impl_read_glam!(I64Vec3, [i64; 3]);
impl_read_glam!(I64Vec4, [i64; 4]);
impl_read_glam!(U64Vec2, [u64; 2]);
impl_read_glam!(U64Vec3, [u64; 3]);
impl_read_glam!(U64Vec4, [u64; 4]);

impl FrogRead for Vec3A {
    #[inline]
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        Ok(Vec3A::from(Vec3::fg_read(buf)?))
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(1024))]

    #[test]
    fn proto_read_vec2(data in proptest::array::uniform2(proptest::num::f32::ANY)) {
        let bytes = data.iter().flat_map(|&v| v.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());

        match (Vec2::from(data), Vec2::fg_read(&mut cursor)) {
            (expected, Ok(read)) => {
                if expected.x.is_finite() {
                    assert!((expected.x - read.x).abs() < f32::EPSILON, "Expected: `{expected}`, Read: `{read}`");
                }
                if expected.y.is_finite() {
                    assert!((expected.y - read.y).abs() < f32::EPSILON, "Expected: `{expected}`, Read: `{read}`");
                }
                assert_eq!(bytes.len(), usize::try_from(cursor.position()).unwrap());
            }
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }

    #[test]
    fn proto_read_vec3(data in proptest::array::uniform3(proptest::num::f32::ANY)) {
        let bytes = data.iter().flat_map(|&v| v.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());

        match (Vec3::from(data), Vec3::fg_read(&mut cursor)) {
            (expected, Ok(read)) => {
                if expected.x.is_finite() {
                    assert!((expected.x - read.x).abs() < f32::EPSILON, "Expected: `{expected}`, Read: `{read}`");
                }
                if expected.y.is_finite() {
                    assert!((expected.y - read.y).abs() < f32::EPSILON, "Expected: `{expected}`, Read: `{read}`");
                }
                if expected.z.is_finite() {
                    assert!((expected.z - read.z).abs() < f32::EPSILON, "Expected: `{expected}`, Read: `{read}`");
                }
                assert_eq!(bytes.len(), usize::try_from(cursor.position()).unwrap());
            }
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }

    #[test]
    fn proto_read_vec4(data in proptest::array::uniform4(proptest::num::f32::ANY)) {
        let bytes = data.iter().flat_map(|&v| v.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());

        match (Vec4::from(data), Vec4::fg_read(&mut cursor)) {
            (expected, Ok(read)) => {
                if expected.x.is_finite() {
                    assert!((expected.x - read.x).abs() < f32::EPSILON, "Expected: `{expected}`, Read: `{read}`");
                }
                if expected.y.is_finite() {
                    assert!((expected.y - read.y).abs() < f32::EPSILON, "Expected: `{expected}`, Read: `{read}`");
                }
                if expected.z.is_finite() {
                    assert!((expected.z - read.z).abs() < f32::EPSILON, "Expected: `{expected}`, Read: `{read}`");
                }
                if expected.w.is_finite() {
                    assert!((expected.w - read.w).abs() < f32::EPSILON, "Expected: `{expected}`, Read: `{read}`");
                }
                assert_eq!(bytes.len(), usize::try_from(cursor.position()).unwrap());
            },
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }

    #[test]
    fn proto_read_dvec2(data in proptest::array::uniform2(proptest::num::f64::ANY)) {
        let bytes = data.iter().flat_map(|&v| v.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());

        match (DVec2::from(data), DVec2::fg_read(&mut cursor)) {
            (expected, Ok(read)) => {
                if expected.x.is_finite() {
                    assert!((expected.x - read.x).abs() < f64::EPSILON, "Expected: `{expected}`, Read: `{read}`");
                }
                if expected.y.is_finite() {
                    assert!((expected.y - read.y).abs() < f64::EPSILON, "Expected: `{expected}`, Read: `{read}`");
                }
                assert_eq!(bytes.len(), usize::try_from(cursor.position()).unwrap());
            }
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }

    #[test]
    fn proto_read_dvec3(data in proptest::array::uniform3(proptest::num::f64::ANY)) {
        let bytes = data.iter().flat_map(|&v| v.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());

        match (DVec3::from(data), DVec3::fg_read(&mut cursor)) {
            (expected, Ok(read)) => {
                if expected.x.is_finite() {
                    assert!((expected.x - read.x).abs() < f64::EPSILON, "Expected: `{expected}`, Read: `{read}`");
                }
                if expected.y.is_finite() {
                    assert!((expected.y - read.y).abs() < f64::EPSILON, "Expected: `{expected}`, Read: `{read}`");
                }
                if expected.z.is_finite() {
                    assert!((expected.z - read.z).abs() < f64::EPSILON, "Expected: `{expected}`, Read: `{read}`");
                }
                assert_eq!(bytes.len(), usize::try_from(cursor.position()).unwrap());
            }
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }

    #[test]
    fn proto_read_dvec4(data in proptest::array::uniform4(proptest::num::f64::ANY)) {
        let bytes = data.iter().flat_map(|&v| v.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());

        match (DVec4::from(data), DVec4::fg_read(&mut cursor)) {
            (expected, Ok(read)) => {
                if expected.x.is_finite() {
                    assert!((expected.x - read.x).abs() < f64::EPSILON, "Expected: `{expected}`, Read: `{read}`");
                }
                if expected.y.is_finite() {
                    assert!((expected.y - read.y).abs() < f64::EPSILON, "Expected: `{expected}`, Read: `{read}`");
                }
                if expected.z.is_finite() {
                    assert!((expected.z - read.z).abs() < f64::EPSILON, "Expected: `{expected}`, Read: `{read}`");
                }
                if expected.w.is_finite() {
                    assert!((expected.w - read.w).abs() < f64::EPSILON, "Expected: `{expected}`, Read: `{read}`");
                }
                assert_eq!(bytes.len(), usize::try_from(cursor.position()).unwrap());
            },
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(256))]

    #[test]
    fn proto_read_i16vec2(data in proptest::array::uniform2(proptest::num::i16::ANY)) {
        let bytes = data.iter().flat_map(|&v| v.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());

        match (I16Vec2::from(data), I16Vec2::fg_read(&mut cursor)) {
            (expected, Ok(read)) => {
                assert_eq!(expected, read);
                assert_eq!(bytes.len(), usize::try_from(cursor.position()).unwrap());
            },
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }

    #[test]
    fn proto_read_uvec3(data in proptest::array::uniform3(proptest::num::u32::ANY)) {
        let bytes = data.iter().flat_map(|&v| v.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());

        match (UVec3::from(data), UVec3::fg_read(&mut cursor)) {
            (expected, Ok(read)) => {
                assert_eq!(expected, read);
                assert_eq!(bytes.len(), usize::try_from(cursor.position()).unwrap());
            },
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }

    #[test]
    fn proto_read_i64vec4(data in proptest::array::uniform4(proptest::num::i64::ANY)) {
        let bytes = data.iter().flat_map(|&v| v.to_be_bytes()).collect::<Vec<_>>();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());

        match (I64Vec4::from(data), I64Vec4::fg_read(&mut cursor)) {
            (expected, Ok(read)) => {
                assert_eq!(expected, read);
                assert_eq!(bytes.len(), usize::try_from(cursor.position()).unwrap());
            },
            (expected, err) => panic!("Expected: `{expected}`, Error: `{err:?}`"),
        }
    }
}
