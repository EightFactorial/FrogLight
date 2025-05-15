//! [`TextStyle`] and [`TextColor`]

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::prelude::Identifier;
use froglight_nbt::{
    nbt::mappings::{ByteBoolOption, TagOption},
    prelude::*,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod color;
pub use color::{IntegerColor, PresetColor, TextColor};

/// The style of a [`FormattedText`](super::FormattedText) component.
#[derive(Debug, Default, Clone, PartialEq, Eq, FrogNbt)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Deserialize, Serialize))]
pub struct TextStyle {
    /// The font of the text.
    #[frog(default, tag = "string", skip_if = Option::is_none)]
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub font: Option<Identifier>,
    /// The color of the text.
    #[frog(default, with = TagOption, skip_if = Option::is_none)]
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub color: Option<TextColor>,
    /// Whether the text is bold.
    #[frog(default, tag = "byte", with = ByteBoolOption, skip_if = Option::is_none)]
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub bold: Option<bool>,
    /// Whether the text is italic.
    #[frog(default, tag = "byte", with = ByteBoolOption, skip_if = Option::is_none)]
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub italic: Option<bool>,
    /// Whether the text is underlined.
    #[frog(default, tag = "byte", with = ByteBoolOption, skip_if = Option::is_none)]
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub underlined: Option<bool>,
    /// Whether the text is strikedthrough.
    #[frog(default, tag = "byte", with = ByteBoolOption, skip_if = Option::is_none)]
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub strikethrough: Option<bool>,
    /// Whether the text is obfuscated.
    #[frog(default, tag = "byte", with = ByteBoolOption, skip_if = Option::is_none)]
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub obfuscated: Option<bool>,
}

impl TextStyle {
    /// An empty [`TextStyle`].
    ///
    /// Inherits all properties from the parent.
    pub const EMPTY: Self = Self {
        font: None,
        color: None,
        bold: None,
        italic: None,
        underlined: None,
        strikethrough: None,
        obfuscated: None,
    };
    /// The default [`TextStyle`] of a text's root component.
    pub const ROOT: Self = Self {
        font: Some(Identifier::const_new("minecraft:text")),
        color: Some(TextColor::ROOT),
        bold: Some(false),
        italic: Some(false),
        underlined: Some(false),
        strikethrough: Some(false),
        obfuscated: Some(false),
    };

    /// Returns `true` if all fields are `None`.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.font.is_none()
            && self.color.is_none()
            && self.bold.is_none()
            && self.italic.is_none()
            && self.underlined.is_none()
            && self.strikethrough.is_none()
            && self.obfuscated.is_none()
    }

    /// Set the font of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub fn with_font(mut self, font: impl Into<Identifier>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Set the [`TextColor`] of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub fn with_color(mut self, color: impl Into<TextColor>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Set the bold property of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub fn with_bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
        self
    }

    /// Set the italic property of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub fn with_italic(mut self, italic: bool) -> Self {
        self.italic = Some(italic);
        self
    }

    /// Set the underlined property of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub fn with_underlined(mut self, underlined: bool) -> Self {
        self.underlined = Some(underlined);
        self
    }

    /// Set the strikethrough property of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub fn with_strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = Some(strikethrough);
        self
    }

    /// Set the obfuscated property of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub fn with_obfuscated(mut self, obfuscated: bool) -> Self {
        self.obfuscated = Some(obfuscated);
        self
    }

    /// Create a new [`TextStyle`] that inherits
    /// any missing fields from another.
    #[must_use]
    pub fn inherit(&self, other: &Self) -> Self {
        Self {
            font: self.font.clone().or_else(|| other.font.clone()),
            color: self.color.clone().or_else(|| other.color.clone()),
            bold: self.bold.or(other.bold),
            italic: self.italic.or(other.italic),
            underlined: self.underlined.or(other.underlined),
            strikethrough: self.strikethrough.or(other.strikethrough),
            obfuscated: self.obfuscated.or(other.obfuscated),
        }
    }

    /// Create a new [`TextStyle`] that only contains
    /// the differences between two styles.
    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        /// Returns `Some(a)` if `a` and `b` are different,
        /// or `Some(val)` if only one is `Some`.
        fn opt_neq<'a, T: PartialEq>(a: &'a Option<T>, b: &'a Option<T>) -> Option<&'a T> {
            match (a, b) {
                (Some(a), Some(b)) if a != b => Some(a),
                (Some(val), None) | (None, Some(val)) => Some(val),
                _ => None,
            }
        }

        Self {
            font: opt_neq(&self.font, &other.font).cloned(),
            color: opt_neq(&self.color, &other.color).cloned(),
            bold: opt_neq(&self.bold, &other.bold).copied(),
            italic: opt_neq(&self.italic, &other.italic).copied(),
            underlined: opt_neq(&self.underlined, &other.underlined).copied(),
            strikethrough: opt_neq(&self.strikethrough, &other.strikethrough).copied(),
            obfuscated: opt_neq(&self.obfuscated, &other.obfuscated).copied(),
        }
    }
}
