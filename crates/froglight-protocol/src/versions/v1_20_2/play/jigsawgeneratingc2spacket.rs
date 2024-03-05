use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct JigsawGeneratingC2SPacket {
    pub pos: (),
    pub max_depth: (),
    pub keep_jigsaws: (),
}
