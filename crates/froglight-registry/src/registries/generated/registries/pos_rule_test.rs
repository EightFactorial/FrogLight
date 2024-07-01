//! @generated by `froglight-generator` #8ccbfa2

use bevy_reflect::Reflect;
use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, FrogRegistry)]
pub enum PosRuleTestKey {
    #[frog(key = "minecraft:always_true")]
    AlwaysTrue,
    #[frog(key = "minecraft:linear_pos")]
    LinearPos,
    #[frog(key = "minecraft:axis_aligned_linear_pos")]
    AxisAlignedLinearPos,
}
