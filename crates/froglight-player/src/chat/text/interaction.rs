//! [`InteractComponent`]

use std::borrow::Cow;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
use froglight_common::prelude::Identifier;
use froglight_nbt::nbt::NbtCompound;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Actions to take when interacting with a [`FormattedText`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct InteractComponent {
    /// Text to insert when the component is interacted with.
    pub insertion: Cow<'static, str>,
    /// An action to perform when the component is clicked.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub click: Option<TextClickInteract>,
    /// An action to perform when the component is hovered over.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub hover: Option<TextHoverInteract>,
}

/// An interaction to perform when the [`FormattedText`] is clicked.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TextClickInteract {
    /// The action type
    pub action: TextClickAction,
}

/// An action to perform when the [`FormattedText`] is clicked.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
pub enum TextClickAction {
    /// A URL to open in the browser.
    OpenUrl(Cow<'static, str>),
    /// A file to open on the computer.
    OpenFile(Cow<'static, str>),
    /// A chat command to send to the server.
    RunCommand(Cow<'static, str>),
    /// Fill in a field in the chat command.
    SuggestCommand(Cow<'static, str>),
    /// Change to a page in a written book.
    ChangePage(Cow<'static, str>),
    /// Copy the text to the clipboard.
    CopyToClipboard(Cow<'static, str>),
}

#[cfg(feature = "serde")]
impl serde::Serialize for TextClickInteract {
    fn serialize<S>(&self, _ser: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        todo!()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TextClickInteract {
    fn deserialize<D>(_de: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        todo!()
    }
}

/// An interaction to perform when the [`FormattedText`] is hovered over.
#[derive(Debug, Clone, PartialEq, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TextHoverInteract {
    /// The action type
    pub action: TextHoverAction,
}

/// An action to perform when the [`FormattedText`] is hovered over.
#[derive(Debug, Clone, PartialEq, From)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
pub enum TextHoverAction {
    /// Show a text message
    ShowText(Cow<'static, str>),
    /// Show an item
    ShowItem(TextHoverItem),
    /// Show an entity
    ShowEntity(TextHoverEntity),
}

#[cfg(feature = "serde")]
impl serde::Serialize for TextHoverInteract {
    fn serialize<S>(&self, _ser: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        todo!()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TextHoverInteract {
    fn deserialize<D>(_de: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        todo!()
    }
}

/// An item action to perform when the [`FormattedText`] is hovered over.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TextHoverItem {
    /// The item's identifier
    pub id: Identifier,
    /// Optionally, the number of items in the stack
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub count: Option<u32>,
    /// Additional NBT components
    pub components: NbtCompound,
}

/// An entity action to perform when the [`FormattedText`] is hovered over.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TextHoverEntity {
    /// An optional name to display
    pub name: Option<Cow<'static, str>>,
    /// The entity's type
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub kind: Identifier,
    /// The entity's [`Uuid`]
    pub id: Uuid,
}
