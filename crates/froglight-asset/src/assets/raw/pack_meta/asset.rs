//! [`ResourcePackMeta`], and other related types.
#![allow(clippy::used_underscore_binding)]

use bevy_app::App;
use bevy_asset::{Asset, AssetApp, Handle, ReflectAsset, ReflectHandle};
use bevy_reflect::{prelude::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use bevy_render::texture::Image;
use bevy_utils::HashMap;
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use crate::assets::ResourcePack;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_asset::<ResourcePackMeta>();

    app.register_type::<ResourcePackMeta>()
        .register_type::<Handle<ResourcePackMeta>>()
        .register_type_data::<Handle<ResourcePackMeta>, ReflectHandle>();
}

/// Metadata about a [`ResourcePack`].
///
/// Created from a [`PackMcMeta`] and an optional icon.
#[derive(Debug, Default, Clone, PartialEq, Eq, Asset, Reflect)]
#[reflect(Default, Asset)]
pub struct ResourcePackMeta {
    /// The [`ResourcePack`]'s icon.
    pub icon: Option<Handle<Image>>,

    /// The [`ResourcePack`]'s metadata.
    pub mcmeta: PackMcMeta,
}

impl From<PackMcMeta> for ResourcePackMeta {
    fn from(mcmeta: PackMcMeta) -> Self { ResourcePackMeta { icon: None, mcmeta } }
}

/// Metadata about a [`ResourcePack`].
///
/// Read from the `pack.mcmeta` file.
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(Default, Serialize, Deserialize)]
pub struct PackMcMeta {
    /// The pack's information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pack: Option<PackInformation>,

    /// The pack's filters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[reflect(ignore)]
    pub filter: Option<PackFilters>,

    /// The pack's overlays.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[reflect(ignore)]
    pub overlays: Option<PackOverlays>,

    /// The pack's supported languages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<HashMap<String, PackLanguage>>,
}

/// Information about a [`ResourcePack`].
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(Default, Serialize, Deserialize)]
pub struct PackInformation {
    /// The pack's description.
    #[reflect(ignore)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<serde_json::Value>,

    /// The pack's format id number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pack_format: Option<i32>,

    /// The pack's supported formats.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supported_formats: Option<SupportedFormats>,
}

/// A list of formats supported by a [`ResourcePack`].
#[derive(Debug, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SupportedFormats {
    /// A single format.
    Single(i32),
    /// A range of supported formats.
    Range(i32, i32),
    /// A range of supported formats with specified bounds.
    RangeCompound {
        /// The minimum supported format version.
        min_inclusive: i32,
        /// The maximum supported format version.
        max_inclusive: i32,
    },
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

/// A language supported by a [`ResourcePack`].
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(Default, Serialize, Deserialize)]
pub struct PackLanguage {
    /// The language's name.
    pub name: String,
    /// The language's region.
    pub region: String,
    /// Whether the language is bidirectional.
    pub bidirectional: bool,
}
