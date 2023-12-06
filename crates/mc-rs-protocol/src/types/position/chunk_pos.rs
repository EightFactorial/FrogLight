use bevy_ecs::prelude::Component;
use bevy_math::IVec2;
use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

use crate::buffer::{VarDecode, VarEncode};

/// A chunk position.
///
/// This is a chunk's position in the world, not how many blocks it is offset from the origin.
///
/// Due to internally using an [IVec2], `y` and `z` are interchangable.
#[derive(
    Default, Clone, Copy, PartialEq, Eq, Hash, Component, Deref, DerefMut, From, Into, Transcode,
)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ChunkPos(pub IVec2);

impl ChunkPos {
    pub const ZERO: Self = Self(IVec2::ZERO);

    pub const fn new(x: i32, z: i32) -> Self { Self(IVec2::new(x, z)) }

    pub const fn around(&self) -> [Self; 4] {
        [
            Self::new(self.0.x - 1, self.0.y),
            Self::new(self.0.x + 1, self.0.y),
            Self::new(self.0.x, self.0.y - 1),
            Self::new(self.0.x, self.0.y + 1),
        ]
    }
}

impl std::fmt::Debug for ChunkPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkPos")
            .field("x", &self.0.x)
            .field("z", &self.0.y)
            .finish()
    }
}

impl From<(i32, i32)> for ChunkPos {
    fn from((x, z): (i32, i32)) -> Self { Self(IVec2::new(x, z)) }
}

impl From<ChunkPos> for (i32, i32) {
    fn from(ChunkPos(IVec2 { x, y }): ChunkPos) -> Self { (x, y) }
}

impl From<[i32; 2]> for ChunkPos {
    fn from([x, z]: [i32; 2]) -> Self { Self(IVec2::new(x, z)) }
}

impl From<ChunkPos> for [i32; 2] {
    fn from(ChunkPos(IVec2 { x, y }): ChunkPos) -> Self { [x, y] }
}

impl VarEncode for ChunkPos {
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        self.0.var_encode(buf)
    }
}

impl VarDecode for ChunkPos {
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        Ok(Self(IVec2::var_decode(buf)?))
    }
}
