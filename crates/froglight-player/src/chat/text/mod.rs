//! Text parsing and formatting.

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

pub mod component;
use component::{
    KeybindComponent, ScoreComponent, SelectorComponent, TextComponent, TranslateComponent,
    ValueComponent,
};

pub mod formatting;
pub use formatting::{TextColor, TextFormatting};

mod compound;
#[cfg(feature = "serde")]
mod serde;

/// A formatted text message.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(no_field_bounds, Debug, PartialEq, Hash))]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
