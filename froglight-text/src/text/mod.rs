//! TODO
#![expect(missing_docs, reason = "WIP")]

use alloc::vec::Vec;

mod formatting;
pub use formatting::TextFormatting;

mod interaction;
pub use interaction::TextInteraction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextComponent<'a> {
    content: TextContentValue<'a>,
    formatting: TextFormatting<'a>,
    interaction: TextInteraction<'a>,
    children: Vec<TextComponent<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextContentValue<'a> {
    tagged: bool,
    value: TextContent<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextContent<'a> {
    // "text"
    Text(&'a ()),
    // "translatable"
    Translate(),
    // "score"
    Scoreboard(),
    // "selector"
    EntityName(),
    // "keybind"
    Keybind(),
    // "nbt"
    Nbt(),
    // "object"
    Object(),
}
