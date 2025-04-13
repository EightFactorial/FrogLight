//! Text parsing and formatting.

use smol_str::SmolStr;

mod formatting;
pub use formatting::{TextColor, TextFormatting};

mod json;
mod nbt;

/// A formatted text message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    /// The content of the text component.
    pub content: TextContent,
    /// Extra text components.
    ///
    /// These are appended to the parent text component
    /// and inherit its formatting.
    pub extra: Vec<Text>,
    /// The formatting of the text.
    pub formatting: TextFormatting,
}

// -------------------------------------------------------------------------------------------------

/// The content of a [`Text`] component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextContent {
    /// A plain-text component.
    Text {
        /// The plain text.
        text: SmolStr,
    },
    /// A translation component.
    Translation {
        /// The translation identifier.
        translate: SmolStr,
        /// An optional fallback string.
        fallback: Option<SmolStr>,
        /// The translation arguments.
        with: Vec<Text>,
    },
    /// A score component.
    Score {
        /// The name of the score holder.
        name: SmolStr,
        /// The objective to display the score of.
        objective: SmolStr,
    },
    /// A selector component.
    Selector {
        /// The selector.
        selector: SmolStr,
        /// An optional separator between multiple selected entities.
        separator: Option<SmolStr>,
    },
    /// A keybind component.
    Keybind {
        /// The keybind identifier.
        keybind: SmolStr,
    },
    /// An Nbt component.
    Nbt {},
}
