//! [`TextComponent`], [`TranslateComponent`], and other [`FormattedText`]
//! components.

#[cfg(not(feature = "std"))]
use alloc::{borrow::Cow, vec::Vec};
#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
use froglight_nbt::{nbt::mappings::ByteBool, prelude::*};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::FormattedText;

/// A plain text component.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogNbt)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TextComponent {
    /// The text of the [`TextComponent`].
    #[frog(tag = "string")]
    pub text: Cow<'static, str>,
}

// -------------------------------------------------------------------------------------------------

/// A translated text component.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TranslateComponent {
    /// The translation key of the [`TranslateComponent`].
    pub translate: Cow<'static, str>,
    /// The fallback message to use if the translation key is not found.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub fallback: Option<Cow<'static, str>>,

    /// Arguments to be used with the translated message.
    #[cfg_attr(
        feature = "serde",
        serde(default, rename = "with", skip_serializing_if = "Vec::is_empty")
    )]
    pub arguments: Vec<FormattedText>,
}

// -------------------------------------------------------------------------------------------------

/// A score component.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogNbt)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct ScoreComponent {
    /// The score objective to display.
    pub score: ScoreObjectiveComponent,
}

/// A scoreboard objective.
#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogNbt)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct ScoreObjectiveComponent {
    /// The name of the score holder or a selector.
    #[frog(tag = "string")]
    pub name: Cow<'static, str>,
    /// The name of the score objective.
    #[frog(tag = "string")]
    pub objective: Cow<'static, str>,
}

// -------------------------------------------------------------------------------------------------

/// An entity selector component.
#[derive(Debug, Clone, PartialEq, FrogNbt)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct SelectorComponent {
    /// The selector used to identify the entities.
    #[frog(tag = "string")]
    pub selector: Cow<'static, str>,
    // /// The separator used when multiple selections are present.
    // #[frog(default = default_separator, tag = Compound)]
    // pub separator: FormattedText,
}

// -------------------------------------------------------------------------------------------------

/// A keybind component.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogNbt)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct KeybindComponent {
    /// The identifier of the keybind to display.
    #[frog(tag = "string")]
    pub keybind: Cow<'static, str>,
}

// -------------------------------------------------------------------------------------------------

/// A NBT value component.
#[derive(Debug, Clone, PartialEq, Deref, DerefMut, From, Into, FrogNbt)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct ValueComponent {
    /// The source of the data.
    #[frog(default, skip_if = Option::is_none)]
    pub source: Option<ValueComponentSource>,
}

/// A NBT value component source.
#[derive(Debug, Clone, PartialEq, FrogNbt)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
pub struct ValueComponentSource {
    /// The source of the data.
    pub source: ValueSourceKind,
    /// The path used to look up data from the source.
    #[frog(tag = "string")]
    pub path: Cow<'static, str>,

    /// Whether to interpret the returned NBT values as [`FormattedText`]s.
    #[frog(tag = "byte", with = ByteBool)]
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

impl FromTag for ValueSourceKind {
    fn from_tag(_tag: &NbtTag) -> Result<Self, NbtError> { todo!() }
}
impl IntoTag for ValueSourceKind {
    fn into_tag(&self) -> Result<NbtTag, NbtError> { todo!() }
}
