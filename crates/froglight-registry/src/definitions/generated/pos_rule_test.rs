//! @generated by `froglight-generator` #b0e1aa4

use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogRegistry)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum PosRuleTestKey {
    #[frog(key = "minecraft:always_true")]
    AlwaysTrue,
    #[frog(key = "minecraft:linear_pos")]
    LinearPos,
    #[frog(key = "minecraft:axis_aligned_linear_pos")]
    AxisAlignedLinearPos,
}
