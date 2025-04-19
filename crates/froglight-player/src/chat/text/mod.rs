//! Text parsing and formatting.

use std::borrow::Cow;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::From;

pub mod component;
use component::{
    KeybindComponent, ScoreComponent, SelectorComponent, TextComponent, TranslateComponent,
    ValueComponent,
};

pub mod formatting;
pub use formatting::{TextColor, TextFormatting};

mod compound;
#[cfg(feature = "serde")]
mod serialize;

/// A formatted text message.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(no_field_bounds, Debug, PartialEq, Hash))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Deserialize, Serialize))]
pub struct FormattedText {
    /// The content of the message.
    pub content: FormattedContent,
    /// The formatting of the message.
    pub formatting: TextFormatting,

    /// Children message components.
    ///
    /// These are appended to the parent and inherit its formatting.
    pub children: Vec<FormattedText>,
}

// -------------------------------------------------------------------------------------------------

/// The content of a [`FormattedText`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, From)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
pub enum FormattedContent {
    /// A plain-text component.
    Text(TextComponent),
    /// A translation component.
    Translation(TranslateComponent),
    /// A score component.
    Score(ScoreComponent),
    /// A selector component.
    Selector(SelectorComponent),
    /// A keybind component.
    Keybind(KeybindComponent),
    /// An Nbt component.
    Nbt(ValueComponent),
}

// -------------------------------------------------------------------------------------------------

/// A reference to a [`FormattedText`] message.
///
/// Used to avoid cloning the message while applying custom formatting.
#[derive(Debug, Clone, PartialEq, Eq, Hash, derive_more::Deref)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize))]
pub struct FormattedTextRef<'a> {
    /// The original message.
    #[deref]
    pub text: &'a FormattedText,
    /// The formatting to apply to the message.
    pub formatting: Cow<'a, TextFormatting>,
}

impl<'a> FormattedTextRef<'a> {
    /// Create a new [`FormattedTextRef`] from a [`FormattedText`],
    /// keeping the original formatting.
    #[inline]
    #[must_use]
    pub const fn new(text: &'a FormattedText) -> Self {
        Self { text, formatting: Cow::Borrowed(&text.formatting) }
    }

    /// Create a new [`FormattedTextRef`] from a [`FormattedText`]
    /// and a custom [`TextFormatting`].
    #[inline]
    #[must_use]
    pub const fn new_with(text: &'a FormattedText, formatting: &'a TextFormatting) -> Self {
        Self { text, formatting: Cow::Borrowed(formatting) }
    }

    /// Apply a new [`TextFormatting`] to a [`FormattedTextRef`].
    #[inline]
    #[must_use]
    pub fn with(mut self, formatting: impl Into<Cow<'a, TextFormatting>>) -> Self {
        self.formatting = formatting.into();
        self
    }
}

impl<'a> From<&'a FormattedText> for FormattedTextRef<'a> {
    #[inline]
    fn from(text: &'a FormattedText) -> Self { Self::new(text) }
}
