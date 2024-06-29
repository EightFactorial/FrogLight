//! @generated by `froglight-generator` #53af970

use bevy_reflect::Reflect;
use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, FrogRegistry)]
pub enum NumberFormatTypeKey {
    #[frog(key = "minecraft:blank")]
    Blank,
    #[frog(key = "minecraft:styled")]
    Styled,
    #[frog(key = "minecraft:fixed")]
    Fixed,
}
