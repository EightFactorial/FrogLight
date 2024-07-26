#![allow(clippy::used_underscore_binding)]

use bevy_asset::{Asset, ReflectAsset};
use bevy_derive::{Deref, DerefMut};
use bevy_reflect::{prelude::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use bevy_utils::HashMap;
use froglight_common::ResourceKey;
use serde::{Deserialize, Serialize};

/// A list of images to create an atlas from.
#[derive(
    Debug, Default, Clone, PartialEq, Reflect, Asset, Serialize, Deserialize, Deref, DerefMut,
)]
#[reflect(Default, Asset, Serialize, Deserialize)]
pub struct ResourceAtlasDefinition {
    /// A list of sources to create the atlas from.
    pub sources: Vec<AtlasDefinitionEntry>,
}

/// An entry in a resource atlas definition.
#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum AtlasDefinitionEntry {
    Directory(AtlasDirectory),
    Single(AtlasSingle),
    Filter(AtlasFilter),
    Unstitch(AtlasUnstitch),
    #[serde(rename = "paletted_permutations")]
    PalettedPermutations(AtlasPalettedPermutations),
}

/// A directory to include in the atlas.
#[derive(Debug, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct AtlasDirectory {
    /// The source directory.
    pub source: ResourceKey,
    /// The prefix to add to the resource path.
    pub prefix: String,
}

/// A single file to include in the atlas.
#[derive(Debug, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct AtlasSingle {
    /// The resource to include.
    pub resource: ResourceKey,
    /// An optional sprite name, defaults to the resource name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sprite")]
    pub sprite_name: Option<String>,
}

/// A filter to apply to the atlas.
///
/// Only resources that match the filter will be kept.
#[derive(Debug, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct AtlasFilter {
    /// A regex to filter resource namespaces by.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "namespace")]
    pub namespace_filter: Option<String>,
    /// A regex to filter resource paths by.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "path")]
    pub path_filter: Option<String>,
}

/// Regions of other images to include in the atlas.
#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct AtlasUnstitch {
    pub resource: ResourceKey,
    pub divisor_x: f32,
    pub divisor_y: f32,
    pub regions: Vec<AtlasUnstitchRegion>,
}

/// A region of another image to include in the atlas.
#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct AtlasUnstitchRegion {
    pub sprite: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Generated permutations of a paletted texture to include in the atlas.
#[derive(Debug, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct AtlasPalettedPermutations {
    textures: Vec<ResourceKey>,
    palette_key: ResourceKey,
    permutations: HashMap<String, ResourceKey>,
}
