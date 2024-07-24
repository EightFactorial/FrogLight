#![allow(clippy::used_underscore_binding)]

use bevy_asset::{Asset, Handle, ReflectAsset};
use bevy_derive::{Deref, DerefMut};
use bevy_reflect::{prelude::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use bevy_render::texture::Image;
use bevy_utils::HashMap;
use serde::{Deserialize, Serialize};

/// Metadata about a [`ResourcePack`].
///
/// Read from the `pack.mcmeta` and `pack.png` files.
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect, Asset)]
#[reflect(Default, Asset)]
pub struct ResourcePackMeta {
    /// The [`ResourcePack`]'s icon.
    pub icon: Option<Handle<Image>>,

    /// Metadata about the [`ResourcePack`].
    pub mcmeta: PackMcMeta,
}

/// Metadata about a [`ResourcePack`].
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(Default, Serialize, Deserialize)]
pub struct PackMcMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pack: Option<PackInformation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[reflect(ignore)]
    pub filter: Option<PackFilters>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[reflect(ignore)]
    pub overlays: Option<PackOverlays>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<PackLanguages>,
}

/// Information about a [`ResourcePack`].
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(Default, Serialize, Deserialize)]
pub struct PackInformation {
    #[reflect(ignore)]
    pub description: serde_json::Value,
    pub pack_format: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supported_formats: Option<SupportedFormats>,
}

/// A list of formats supported by a [`ResourcePack`].
#[derive(Debug, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SupportedFormats {
    Single(i32),
    Range(i32, i32),
    RangeCompound { min_inclusive: i32, max_inclusive: i32 },
}

/// A list of filters for a [`ResourcePack`].
///
/// TODO: Support filters.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackFilters {}

/// A list of [`ResourcePack`] overlays.
///
/// TODO: Support overlays.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackOverlays {}

/// A list of languages supported by a [`ResourcePack`].
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize, Deref, DerefMut,
)]
#[reflect(Default, Serialize, Deserialize)]
pub struct PackLanguages {
    #[serde(flatten)]
    languages: HashMap<String, PackLanguage>,
}

/// A language supported by a [`ResourcePack`].
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(Default, Serialize, Deserialize)]
pub struct PackLanguage {
    pub name: String,
    pub region: String,
    pub bidirectional: bool,
}
