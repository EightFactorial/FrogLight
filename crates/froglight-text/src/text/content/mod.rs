//! TODO

mod keybind;
pub use keybind::KeybindComponent;

mod nbt;
pub use nbt::ValueComponent;

mod score;
pub use score::ScoreComponent;

mod selector;
pub use selector::SelectorComponent;

mod text;
pub use text::TextComponent;

mod translate;
pub use translate::TranslateComponent;

/// The content of a [`FormattedText`].
#[derive(Debug, Clone, PartialEq, derive_more::From)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect), reflect(Debug, Clone, PartialEq))]
pub enum TextContent {
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
