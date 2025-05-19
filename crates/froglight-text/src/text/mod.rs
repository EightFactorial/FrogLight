//! A representation of text, including formatting

#[cfg(not(feature = "std"))]
use alloc::{borrow::Cow, vec::Vec};
#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::Deref;

pub mod content;
use content::{TextComponent, TextContent};

pub mod interaction;
use interaction::TextInteraction;

pub mod style;
use style::TextStyle;

#[cfg(feature = "serde")]
mod serde;

/// A formatted text message.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(no_field_bounds, Debug, Clone, PartialEq))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Deserialize, Serialize))]
pub struct FormattedText {
    /// The content of the message.
    pub content: TextContent,
    /// The style of the message.
    pub style: TextStyle,
    /// The interactability of the message.
    pub interaction: TextInteraction,

    /// Children that inherit the parent's formatting.
    pub children: Vec<FormattedText>,
}

impl FormattedText {
    /// Create a new [`FormattedText`] with the given text.
    #[must_use]
    pub const fn new(text: &'static str) -> Self {
        Self {
            content: TextContent::Text(TextComponent { text: Cow::Borrowed(text) }),
            style: TextStyle::EMPTY,
            interaction: TextInteraction::DEFAULT,
            children: Vec::new(),
        }
    }

    /// Create a new [`FormattedText`] from the given text.
    #[must_use]
    pub fn from_string(text: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content: TextContent::Text(TextComponent { text: text.into() }),
            style: TextStyle::EMPTY,
            interaction: TextInteraction::DEFAULT,
            children: Vec::new(),
        }
    }

    /// Set the [`TextStyle`] of the [`FormattedText`].
    #[inline]
    #[must_use]
    pub fn with_style(mut self, style: impl Into<TextStyle>) -> Self {
        self.style = style.into();
        self
    }

    /// Set the [`TextInteraction`] of the [`FormattedText`].
    #[inline]
    #[must_use]
    pub fn with_interaction(mut self, interaction: impl Into<TextInteraction>) -> Self {
        self.interaction = interaction.into();
        self
    }

    /// Set the children of the [`FormattedText`].
    #[inline]
    #[must_use]
    pub fn with_children(mut self, children: impl IntoIterator<Item = Self>) -> Self {
        self.children = children.into_iter().collect();
        self
    }

    /// Add a child to the [`FormattedText`].
    #[inline]
    pub fn with_child(&mut self, child: Self) -> &mut Self {
        self.children.push(child);
        self
    }
}

// -------------------------------------------------------------------------------------------------

/// A [`FormattedText`] with custom formatting and interactions.
///
/// Used to apply custom properties to a [`FormattedText`]
/// without modifying the original.
#[derive(Debug, Clone, Copy, PartialEq, Deref)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize))]
pub struct FormattedTextRef<'t, 'a> {
    /// A reference to the original [`FormattedText`].
    #[deref]
    pub original: &'t FormattedText,

    /// A custom style to apply to the text.
    pub style: &'a TextStyle,
    /// Custom interactions to apply to the text.
    pub interaction: &'a TextInteraction,
}

impl<'t, 'a> FormattedTextRef<'t, 'a> {
    /// Create a new [`FormattedTextRef`] from a [`FormattedText`].
    ///
    /// Uses the original text's style and interaction by default.
    #[inline]
    #[must_use]
    pub const fn new(text: &'t FormattedText) -> Self
    where 't: 'a {
        Self { original: text, style: &text.style, interaction: &text.interaction }
    }

    /// Update the [`TextStyle`] applied to the [`FormattedTextRef`].
    #[inline]
    #[must_use]
    pub const fn with_style(mut self, style: &'a TextStyle) -> Self {
        self.style = style;
        self
    }

    /// Update the [`TextInteraction`] applied to the [`FormattedTextRef`].
    #[inline]
    #[must_use]
    pub const fn with_interaction(mut self, interaction: &'a TextInteraction) -> Self {
        self.interaction = interaction;
        self
    }
}

impl<'t> From<&'t FormattedText> for FormattedTextRef<'t, 't> {
    #[inline]
    fn from(text: &'t FormattedText) -> Self { Self::new(text) }
}
