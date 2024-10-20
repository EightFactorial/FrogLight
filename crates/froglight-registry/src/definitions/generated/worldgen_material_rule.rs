//! @generated by `froglight-generator` #b0e1aa4

use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogRegistry)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum WorldgenMaterialRuleKey {
    #[frog(key = "minecraft:bandlands")]
    Bandlands,
    #[frog(key = "minecraft:block")]
    Block,
    #[frog(key = "minecraft:sequence")]
    Sequence,
    #[frog(key = "minecraft:condition")]
    Condition,
}
