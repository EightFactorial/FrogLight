//! @generated by `froglight-generator` #b0e1aa4

use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogRegistry)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum WorldgenStructureProcessorKey {
    #[frog(key = "minecraft:block_ignore")]
    BlockIgnore,
    #[frog(key = "minecraft:block_rot")]
    BlockRot,
    #[frog(key = "minecraft:gravity")]
    Gravity,
    #[frog(key = "minecraft:jigsaw_replacement")]
    JigsawReplacement,
    #[frog(key = "minecraft:rule")]
    Rule,
    #[frog(key = "minecraft:nop")]
    Nop,
    #[frog(key = "minecraft:block_age")]
    BlockAge,
    #[frog(key = "minecraft:blackstone_replace")]
    BlackstoneReplace,
    #[frog(key = "minecraft:lava_submerged_block")]
    LavaSubmergedBlock,
    #[frog(key = "minecraft:protected_blocks")]
    ProtectedBlocks,
    #[frog(key = "minecraft:capped")]
    Capped,
}
