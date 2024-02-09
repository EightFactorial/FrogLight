//! Protocol 763
//!
//! Used by Minecraft 1.20.0 - 1.20.1

use bevy_reflect::Reflect;

use crate::traits::Version;

/// Protocol 763
///
/// Used by Minecraft 1.20.0 - 1.20.1
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct V1_20_0;

impl Version for V1_20_0 {
    const PROTOCOL_VERSION: i32 = 763;
}
