//! Legacy formatting codes.

use alloc::vec::Vec;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

use super::{PresetColor, TextColor, TextStyle};

/// A legacy formatting code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
pub enum LegacyCode {
    /// Set the text color
    Color(PresetColor),
    /// Obfuscate the text
    Obfuscated,
    /// Bold the text
    Bold,
    /// Strikethrough the text
    Strikethrough,
    /// Underline the text
    Underline,
    /// Italicize the text
    Italic,
    /// Reset all styles
    Reset,
}

impl LegacyCode {
    /// Get the [`LegacyCode`] for a [`char`].
    #[must_use]
    pub const fn try_from_char(code: char) -> Option<Self> {
        match code {
            '0' => Some(Self::Color(PresetColor::Black)),
            '1' => Some(Self::Color(PresetColor::DarkBlue)),
            '2' => Some(Self::Color(PresetColor::DarkGreen)),
            '3' => Some(Self::Color(PresetColor::DarkAqua)),
            '4' => Some(Self::Color(PresetColor::DarkRed)),
            '5' => Some(Self::Color(PresetColor::DarkPurple)),
            '6' => Some(Self::Color(PresetColor::Gold)),
            '7' => Some(Self::Color(PresetColor::Gray)),
            '8' => Some(Self::Color(PresetColor::DarkGray)),
            '9' => Some(Self::Color(PresetColor::Blue)),
            'a' => Some(Self::Color(PresetColor::Green)),
            'b' => Some(Self::Color(PresetColor::Aqua)),
            'c' => Some(Self::Color(PresetColor::Red)),
            'd' => Some(Self::Color(PresetColor::LightPurple)),
            'e' => Some(Self::Color(PresetColor::Yellow)),
            'f' => Some(Self::Color(PresetColor::White)),
            'k' => Some(Self::Obfuscated),
            'l' => Some(Self::Bold),
            'm' => Some(Self::Strikethrough),
            'n' => Some(Self::Underline),
            'o' => Some(Self::Italic),
            'r' => Some(Self::Reset),
            _ => None,
        }
    }

    /// Get the [`char`] for a [`LegacyCode`].
    #[must_use]
    pub const fn as_char(self) -> char {
        match self {
            Self::Color(PresetColor::Black) => '0',
            Self::Color(PresetColor::DarkBlue) => '1',
            Self::Color(PresetColor::DarkGreen) => '2',
            Self::Color(PresetColor::DarkAqua) => '3',
            Self::Color(PresetColor::DarkRed) => '4',
            Self::Color(PresetColor::DarkPurple) => '5',
            Self::Color(PresetColor::Gold) => '6',
            Self::Color(PresetColor::Gray) => '7',
            Self::Color(PresetColor::DarkGray) => '8',
            Self::Color(PresetColor::Blue) => '9',
            Self::Color(PresetColor::Green) => 'a',
            Self::Color(PresetColor::Aqua) => 'b',
            Self::Color(PresetColor::Red) => 'c',
            Self::Color(PresetColor::LightPurple) => 'd',
            Self::Color(PresetColor::Yellow) => 'e',
            Self::Color(PresetColor::White) => 'f',
            Self::Obfuscated => 'k',
            Self::Bold => 'l',
            Self::Strikethrough => 'm',
            Self::Underline => 'n',
            Self::Italic => 'o',
            Self::Reset => 'r',
        }
    }

    /// Get the [`LegacyCode`]s for a [`TextStyle`].
    ///
    /// If the style has a color that can't be represented
    /// by a legacy code it will silently be ignored.
    #[must_use]
    pub fn from_style_lossy(style: &TextStyle) -> Vec<Self> {
        let color = style.color.and_then(TextColor::try_as_preset);
        Self::from_style_inner(style, color)
    }

    /// Get the [`LegacyCode`]s for a [`TextStyle`].
    ///
    /// Returns `None` if the style has a color that
    /// can't be represented by a legacy code.
    #[must_use]
    pub fn try_from_style(style: &TextStyle) -> Option<Vec<Self>> {
        let mut color: Option<PresetColor> = None;
        if let Some(style_color) = style.color {
            color = Some(style_color.try_as_preset()?);
        }
        Some(Self::from_style_inner(style, color))
    }

    fn from_style_inner(style: &TextStyle, color: Option<PresetColor>) -> Vec<Self> {
        let mut codes = Vec::with_capacity(2);

        // If any of the options are being disabled, reset all styles.
        if style.obfuscated.is_some_and(|b| !b)
            || style.bold.is_some_and(|b| !b)
            || style.strikethrough.is_some_and(|b| !b)
            || style.underlined.is_some_and(|b| !b)
            || style.italic.is_some_and(|b| !b)
        {
            codes.push(Self::Reset);
        }

        if let Some(color) = color {
            codes.push(Self::Color(color));
        }

        if style.obfuscated.is_some_and(|b| b) {
            codes.push(Self::Obfuscated);
        }
        if style.bold.is_some_and(|b| b) {
            codes.push(Self::Bold);
        }
        if style.strikethrough.is_some_and(|b| b) {
            codes.push(Self::Strikethrough);
        }
        if style.underlined.is_some_and(|b| b) {
            codes.push(Self::Underline);
        }
        if style.italic.is_some_and(|b| b) {
            codes.push(Self::Italic);
        }

        codes
    }

    /// Convert a list of [`LegacyCode`]s into a [`TextStyle`].
    #[must_use]
    pub fn as_style(codes: impl Iterator<Item = Self>) -> TextStyle {
        let mut style = TextStyle::EMPTY;
        for code in codes {
            code.apply_to_style(&mut style);
        }
        style
    }

    /// Apply the [`LegacyCode`] to a [`TextStyle`].
    pub fn apply_to_style(self, style: &mut TextStyle) -> &mut TextStyle {
        match self {
            Self::Color(color) => style.color = Some(TextColor::Preset(color)),
            Self::Obfuscated => style.obfuscated = Some(true),
            Self::Bold => style.bold = Some(true),
            Self::Strikethrough => style.strikethrough = Some(true),
            Self::Underline => style.underlined = Some(true),
            Self::Italic => style.italic = Some(true),
            Self::Reset => *style = TextStyle::EMPTY,
        }
        style
    }
}

// -------------------------------------------------------------------------------------------------
//
// TODO: Tests
