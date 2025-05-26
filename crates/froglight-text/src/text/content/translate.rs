#[cfg(not(feature = "std"))]
use alloc::{borrow::Cow, vec::Vec};
#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_nbt::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::text::FormattedText;

/// A translated text component.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
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

impl FromCompound for TranslateComponent {
    fn from_compound(_: &NbtCompound) -> Result<Self, NbtError> { todo!() }
}

impl IntoCompound for TranslateComponent {
    fn into_compound(&self) -> Result<NbtCompound, NbtError> { todo!() }
}

// -------------------------------------------------------------------------------------------------
//
// TODO: Tests
