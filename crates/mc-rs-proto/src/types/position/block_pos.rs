use bevy_math::IVec3;
use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, Transcode)]
pub struct BlockPos(pub IVec3);

impl BlockPos {
    pub const ZERO: Self = Self(IVec3::ZERO);

    pub fn new(x: i32, y: i32, z: i32) -> Self { Self(IVec3::new(x, y, z)) }
}
