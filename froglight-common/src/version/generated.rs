//! Version types
//! 
//! This file is automatically @generated, do not edit it manually.

/// Minecraft 26.1
///
/// See the [Minecraft Wiki](https://minecraft.wiki/w/Java_Edition_26.1) for more details.
#[cfg(feature = "v26_1")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
pub struct V26_1;

#[cfg(feature = "v26_1")]
impl super::Version for V26_1 {
    const DATA_VERSION: u32 = 4790;
    const PROTOCOL_ID: u32 = 775;
    const RESOURCE_VERSION: u32 = 84;
}
/// Minecraft 26.2
///
/// See the [Minecraft Wiki](https://minecraft.wiki/w/Java_Edition_26.2) for more details.
#[cfg(feature = "v26_2")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
pub struct V26_2;

#[cfg(feature = "v26_2")]
impl super::Version for V26_2 {
    const DATA_VERSION: u32 = 4883;
    const PROTOCOL_ID: u32 = 1073742130;
    const RESOURCE_VERSION: u32 = 85;
}
