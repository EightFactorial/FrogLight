use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ChunkRenderDistanceCenterPacket {
    #[frog(var)]
    pub chunk_x: i32,
    #[frog(var)]
    pub chunk_z: i32,
}
