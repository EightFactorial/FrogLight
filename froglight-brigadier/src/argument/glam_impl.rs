use glam::{
    BVec2, BVec3, BVec4, DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, UVec2, UVec3, UVec4, Vec2, Vec3,
    Vec4,
};

use super::{ArgumentParseError, ArgumentParser};

macro_rules! impl_glam {
    ($(($ty:ty: [$inner:ty; $n:expr])),*) => {
        $(
            impl ArgumentParser for $ty {
                type Data = <$inner as ArgumentParser>::Data;
                fn parse<'a>(input: &'a str, data: &Self::Data) -> Result<(Self, &'a str), ArgumentParseError> {
                    <[$inner; $n] as ArgumentParser>::parse(input, data).map(|(arr, rest)| (Self::from(arr), rest))
                }
            }
        )*
    };
}

impl_glam!(
    (BVec2: [bool; 2]),
    (BVec3: [bool; 3]),
    (BVec4: [bool; 4]),
    (UVec2: [u32; 2]),
    (UVec3: [u32; 3]),
    (UVec4: [u32; 4]),
    (IVec2: [i32; 2]),
    (IVec3: [i32; 3]),
    (IVec4: [i32; 4]),
    (Vec2: [f32; 2]),
    (Vec3: [f32; 3]),
    (Vec4: [f32; 4]),
    (DVec2: [f64; 2]),
    (DVec3: [f64; 3]),
    (DVec4: [f64; 4])
);
