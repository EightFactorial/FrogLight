//! [`BossBarAction`] and related types.

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_nbt::io::NbtWrapper;
use froglight_text::text::FormattedText;

/// An action to be performed on a boss bar.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub enum BossBarAction {
    /// Add a new boss bar.
    Add(BossBarSettings),
    /// Remove the boss bar.
    Remove,
    /// Update the health of a boss bar.
    Health(f32),
    /// Update the title of a boss bar.
    Title(NbtWrapper<FormattedText>),
    /// Update the style of a boss bar.
    Style(BossBarStyle),
    /// Update the flags of a boss bar.
    Flags(u8),
}

// -------------------------------------------------------------------------------------------------

#[expect(missing_docs)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct BossBarSettings {
    pub title: NbtWrapper<FormattedText>,
    pub health: f32,
    pub style: BossBarStyle,
    pub flags: u8,
}

// -------------------------------------------------------------------------------------------------

/// The color and number of segments in the boss bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct BossBarStyle {
    /// The color of the boss bar.
    pub color: BossBarColor,
    /// The number of segments in the boss bar.
    pub dividers: BossBarSegments,
}

/// The color of the boss bar.
#[expect(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub enum BossBarColor {
    Pink,
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    White,
}

/// The number of segments in the boss bar.
#[expect(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub enum BossBarSegments {
    _0,
    _6,
    _10,
    _12,
    _20,
}
