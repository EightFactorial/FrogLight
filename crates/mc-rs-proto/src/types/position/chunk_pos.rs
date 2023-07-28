use bevy_ecs::prelude::Component;
use mc_rs_macros::Transcode;

use crate::buffer::{VarDecode, VarEncode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Transcode)]
pub struct ChunkSectionPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ChunkSectionPos {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };

    pub fn new(x: i32, y: i32, z: i32) -> Self { Self { x, y, z } }
}

impl From<bevy_math::IVec3> for ChunkSectionPos {
    fn from(bevy_math::IVec3 { x, y, z }: bevy_math::IVec3) -> Self { Self { x, y, z } }
}

impl From<ChunkSectionPos> for bevy_math::IVec3 {
    fn from(ChunkSectionPos { x, y, z }: ChunkSectionPos) -> Self { Self::new(x, y, z) }
}

impl From<(i32, i32, i32)> for ChunkSectionPos {
    fn from((x, y, z): (i32, i32, i32)) -> Self { Self { x, y, z } }
}

impl From<ChunkSectionPos> for (i32, i32, i32) {
    fn from(ChunkSectionPos { x, y, z }: ChunkSectionPos) -> Self { (x, y, z) }
}

impl From<[i32; 3]> for ChunkSectionPos {
    fn from([x, y, z]: [i32; 3]) -> Self { Self { x, y, z } }
}

impl From<ChunkSectionPos> for [i32; 3] {
    fn from(ChunkSectionPos { x, y, z }: ChunkSectionPos) -> Self { [x, y, z] }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Transcode)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    pub const ZERO: Self = Self { x: 0, z: 0 };

    pub fn new(x: i32, z: i32) -> Self { Self { x, z } }
}

impl From<bevy_math::IVec2> for ChunkPos {
    fn from(bevy_math::IVec2 { x, y }: bevy_math::IVec2) -> Self { Self { x, z: y } }
}

impl From<ChunkPos> for bevy_math::IVec2 {
    fn from(ChunkPos { x, z }: ChunkPos) -> Self { Self::new(x, z) }
}

impl From<(i32, i32)> for ChunkPos {
    fn from((x, z): (i32, i32)) -> Self { Self { x, z } }
}

impl From<ChunkPos> for (i32, i32) {
    fn from(ChunkPos { x, z }: ChunkPos) -> Self { (x, z) }
}

impl From<[i32; 2]> for ChunkPos {
    fn from([x, z]: [i32; 2]) -> Self { Self { x, z } }
}

impl From<ChunkPos> for [i32; 2] {
    fn from(ChunkPos { x, z }: ChunkPos) -> Self { [x, z] }
}

impl VarEncode for ChunkPos {
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        self.x.var_encode(buf)?;
        self.z.var_encode(buf)
    }
}

impl VarDecode for ChunkPos {
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        Ok(Self {
            x: i32::var_decode(buf)?,
            z: i32::var_decode(buf)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Transcode)]
pub struct ChunkBlockPos {
    pub x: u8,
    pub y: i32,
    pub z: u8,
}

impl ChunkBlockPos {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };

    pub fn new(x: u8, y: i32, z: u8) -> Self { Self { x, y, z } }
}
