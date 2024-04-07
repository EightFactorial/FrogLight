use froglight_macros::FrogReadWrite;

use crate::common::{BlockPosition, UnsizedByteBuffer};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct UpdateStructureBlockC2SPacket {
    pub position: BlockPosition,
    // TODO: Implement this
    pub data: UnsizedByteBuffer,
    // pub b: Enum,
    // pub c: Enum,
    // pub d: String,
    // pub e: u8,
    // pub f: u8,
    // pub g: u8,
    // pub h: u8,
    // pub i: u8,
    // pub j: u8,
    // pub k: Enum,
    // pub l: Enum,
    // pub m: String,
    // pub n: f32,
    // pub o: u64,
    // pub p: u8,
}
