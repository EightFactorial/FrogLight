use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Transcode)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };

    pub fn new(x: i32, y: i32, z: i32) -> Self { Self { x, y, z } }
}

impl From<bevy_math::IVec3> for BlockPos {
    fn from(bevy_math::IVec3 { x, y, z }: bevy_math::IVec3) -> Self { Self { x, y, z } }
}

impl From<BlockPos> for bevy_math::IVec3 {
    fn from(BlockPos { x, y, z }: BlockPos) -> Self { Self::new(x, y, z) }
}

impl From<(i32, i32, i32)> for BlockPos {
    fn from((x, y, z): (i32, i32, i32)) -> Self { Self { x, y, z } }
}

impl From<BlockPos> for (i32, i32, i32) {
    fn from(BlockPos { x, y, z }: BlockPos) -> Self { (x, y, z) }
}

impl From<[i32; 3]> for BlockPos {
    fn from([x, y, z]: [i32; 3]) -> Self { Self { x, y, z } }
}

impl From<BlockPos> for [i32; 3] {
    fn from(BlockPos { x, y, z }: BlockPos) -> Self { [x, y, z] }
}
