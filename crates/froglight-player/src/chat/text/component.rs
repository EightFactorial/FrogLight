//! TODO

use std::borrow::Cow;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::FormattedText;

/// A plain text component.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TextComponent {
    /// The text of the [`TextComponent`].
    pub text: Cow<'static, str>,
}

// -------------------------------------------------------------------------------------------------

/// A translated text component.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TranslateComponent {
    /// The translation key of the [`TranslateComponent`].
    pub translate: Cow<'static, str>,
    /// The fallback message to use if the translation key is not found.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "str::is_empty"))]
    pub fallback: Cow<'static, str>,

    /// Arguments to be used with the translated message.
    #[cfg_attr(
        feature = "serde",
        serde(default, rename = "with", skip_serializing_if = "Vec::is_empty")
    )]
    pub arguments: Vec<FormattedText>,
}

// -------------------------------------------------------------------------------------------------

/// A score component.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct ScoreComponent {
    /// The score objective to display.
    pub score: ScoreObjectiveComponent,
}

/// A scoreboard objective.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct ScoreObjectiveComponent {
    /// The name of the score holder or a selector.
    pub name: Cow<'static, str>,
    /// The name of the score objective.
    pub objective: Cow<'static, str>,
}

// -------------------------------------------------------------------------------------------------

/// An entity selector component.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct SelectorComponent {
    /// The selector used to identify the entities.
    pub selector: Cow<'static, str>,
    // /// The separator used when multiple selections are present.
    // pub separator: FormattedText,
}

// -------------------------------------------------------------------------------------------------

/// A keybind component.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct KeybindComponent {
    /// The identifier of the keybind to display.
    pub keybind: Cow<'static, str>,
}

// -------------------------------------------------------------------------------------------------

/// A NBT value component.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct ValueComponent {
    /// The source of the data.
    pub source: Option<ValueComponentSource>,
}

/// A NBT value component source.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
pub struct ValueComponentSource {
    /// The source of the data.
    pub source: ValueSourceKind,
    /// The path used to look up data from the source.
    pub path: Cow<'static, str>,

    /// Whether to interpret the returned NBT values as [`FormattedText`]s.
    pub interpret: bool,
    // /// The separator to use when displaying multiple tags.
    // pub separator: FormattedText,
}

/// The type and location of the data source.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
pub enum ValueSourceKind {
    /// Coordinates to the block entity which contains the data.
    Block(Cow<'static, str>),
    /// A selector used to query entities for data.
    Entity(Cow<'static, str>),
    /// The path to the command storage which contains the data.
    Storage(Cow<'static, str>),
}
