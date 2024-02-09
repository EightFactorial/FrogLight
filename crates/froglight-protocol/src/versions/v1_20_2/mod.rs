//! Protocol 764
//!
//! Used by Minecraft 1.20.2

use bevy_reflect::Reflect;

use crate::traits::Version;

/// Protocol 764
///
/// Used by Minecraft 1.20.2
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct V1_20_2;

impl Version for V1_20_2 {
    const PROTOCOL_VERSION: i32 = 764;
}
