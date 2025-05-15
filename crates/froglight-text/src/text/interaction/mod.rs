//! [`InteractComponent`]

#[cfg(not(feature = "std"))]
use alloc::borrow::Cow;
#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod click;
pub use click::TextClickInteract;

pub mod hover;
pub use hover::TextHoverInteract;

/// Actions to take when interacting with a [`FormattedText`].
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TextInteraction {
    /// Text to insert when the component is interacted with.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub insertion: Option<Cow<'static, str>>,
    /// An action to perform when the component is clicked.
    #[cfg_attr(
        feature = "serde",
        serde(default, rename = "clickEvent", skip_serializing_if = "Option::is_none")
    )]
    pub click: Option<TextClickInteract>,
    /// An action to perform when the component is hovered over.
    #[cfg_attr(
        feature = "serde",
        serde(default, rename = "hoverEvent", skip_serializing_if = "Option::is_none")
    )]
    pub hover: Option<TextHoverInteract>,
}

impl TextInteraction {
    /// The default [`TextInteraction`].
    pub const DEFAULT: Self = Self { insertion: None, click: None, hover: None };

    /// Returns `true` if all fields are `None`.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.insertion.is_none() && self.click.is_none() && self.hover.is_none()
    }

    /// Update the [`InteractComponent`] with the given insertion text.
    #[inline]
    #[must_use]
    pub fn with_insert(mut self, insert: impl Into<Cow<'static, str>>) -> Self {
        self.insertion = Some(insert.into());
        self
    }

    /// Update the [`InteractComponent`] with the given [`TextClickInteract`].
    #[inline]
    #[must_use]
    pub fn with_click(mut self, click: impl Into<TextClickInteract>) -> Self {
        self.click = Some(click.into());
        self
    }

    /// Update the [`InteractComponent`] with the given [`TextHoverInteract`].
    #[inline]
    #[must_use]
    pub fn with_hover(mut self, hover: impl Into<TextHoverInteract>) -> Self {
        self.hover = Some(hover.into());
        self
    }
}
