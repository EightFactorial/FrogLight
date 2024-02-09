//! Protocol 765
//!
//! Used by Minecraft 1.20.3 - 1.20.4

use bevy_reflect::Reflect;

use crate::traits::Version;

/// Protocol 765
///
/// Used by Minecraft 1.20.3 - 1.20.4
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct V1_20_3;

impl Version for V1_20_3 {
    const PROTOCOL_VERSION: i32 = 765;
}
