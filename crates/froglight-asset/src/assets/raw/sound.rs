//! [`SoundDefinitionMap`] and related types.

use bevy_app::App;
use bevy_asset::{Asset, AssetApp, Handle, ReflectAsset, ReflectHandle};
use bevy_derive::{Deref, DerefMut};
use bevy_reflect::{prelude::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use bevy_utils::HashMap;
use serde::{Deserialize, Serialize};

use crate::assets::SerdeJsonLoader;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_asset::<SoundDefinitionMap>();
    app.init_asset_loader::<SerdeJsonLoader<SoundDefinitionMap>>();

    app.register_type::<SoundDefinitionMap>()
        .register_type::<Handle<SoundDefinitionMap>>()
        .register_type_data::<Handle<SoundDefinitionMap>, ReflectHandle>();
}

/// A map of sound definitions.
#[derive(
    Debug, Default, Clone, PartialEq, Deref, DerefMut, Serialize, Deserialize, Asset, Reflect,
)]
#[reflect(Default, Serialize, Deserialize, Asset)]
#[serde(transparent)]
pub struct SoundDefinitionMap(HashMap<String, SoundDefinition>);

/// A definition inside a [`SoundDefinitionMap`].
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Default, Serialize, Deserialize)]
pub struct SoundDefinition {
    /// Whether the sound should replace existing sounds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,

    /// The subtitle to display when playing the sound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,

    /// The sounds in this definition.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sounds: Vec<DefinitionEntry>,
}

impl SoundDefinition {
    /// The default value for the `replace` field.
    pub const DEFAULT_REPLACE: bool = false;

    /// Whether the sound should replace existing sounds.
    #[must_use]
    pub fn replace(&self) -> bool { self.replace.unwrap_or(Self::DEFAULT_REPLACE) }
}

/// An entry in a [`SoundDefinition`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DefinitionEntry {
    /// A path to an audio file.
    SoundPath(String),
    /// A sound file.
    SoundFile(DefinitionFile),
}

impl DefinitionEntry {
    /// The name of the sound.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            DefinitionEntry::SoundPath(path) => path,
            DefinitionEntry::SoundFile(file) => &file.name,
        }
    }

    /// The type of sound this definition represents.
    #[must_use]
    pub fn sound_type(&self) -> DefinitionType {
        match self {
            DefinitionEntry::SoundPath(_) => DefinitionType::File,
            DefinitionEntry::SoundFile(file) => file.sound_type(),
        }
    }
}

/// A sound file.
///
/// If this file points to another [`DefinitionFile`],
/// override any fields that are `None`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
pub struct DefinitionFile {
    /// The name or path of the sound.
    pub name: String,

    /// The volume of the sound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<f32>,

    /// The pitch of the sound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f32>,

    /// The weight of the sound when randomly selecting a sound to play.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,

    /// Whether the sound should be streamed from disk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// The distance at which the sound starts to decrease in volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attenuation_distance: Option<i32>,

    /// Whether the sound should be preloaded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preload: Option<bool>,

    /// The type of sound this definition represents.
    ///
    /// If this is a [`DefinitionType::File`], the name should be a path to an
    /// audio file.
    ///
    /// If this is a [`DefinitionType::Event`], the name should be the name of
    /// another [`SoundDefinition`].
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<DefinitionType>,
}

impl DefinitionFile {
    /// The default value for the `volume` field.
    pub const DEFAULT_VOLUME: f32 = 1.0;
    /// The default value for the `pitch` field.
    pub const DEFAULT_PITCH: f32 = 1.0;
    /// The default value for the `weight` field.
    pub const DEFAULT_WEIGHT: i32 = 1;
    /// The default value for the `stream` field.
    pub const DEFAULT_STREAM: bool = false;
    /// The default value for the `attenuation_distance` field.
    pub const DEFAULT_ATTENUATION_DISTANCE: i32 = 16;
    /// The default value for the `preload` field.
    pub const DEFAULT_PRELOAD: bool = false;

    /// The volume of the sound.
    #[must_use]
    pub fn volume(&self) -> f32 { self.volume.unwrap_or(Self::DEFAULT_VOLUME) }

    /// The pitch of the sound.
    #[must_use]
    pub fn pitch(&self) -> f32 { self.pitch.unwrap_or(Self::DEFAULT_PITCH) }

    /// The weight of the sound when randomly selecting a sound to play.
    #[must_use]
    pub fn weight(&self) -> i32 { self.weight.unwrap_or(Self::DEFAULT_WEIGHT) }

    /// Whether the sound should be streamed from disk.
    #[must_use]
    pub fn stream(&self) -> bool { self.stream.unwrap_or(Self::DEFAULT_STREAM) }

    /// The distance at which the sound starts to decrease in volume.
    #[must_use]
    pub fn attenuation_distance(&self) -> i32 {
        self.attenuation_distance.unwrap_or(Self::DEFAULT_ATTENUATION_DISTANCE)
    }

    /// Whether the sound should be preloaded.
    #[must_use]
    pub fn preload(&self) -> bool { self.preload.unwrap_or(Self::DEFAULT_PRELOAD) }

    /// The type of sound this definition represents.
    ///
    /// If this is a [`DefinitionType::File`], the name should be a path to an
    /// audio file.
    ///
    /// If this is a [`DefinitionType::Event`], the name should be the name of
    /// another [`SoundDefinition`].
    #[must_use]
    pub fn sound_type(&self) -> DefinitionType { self.type_.unwrap_or_default() }
}

/// The type of sound a definition represents.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
#[reflect(Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DefinitionType {
    /// A sound file.
    #[default]
    File,
    /// A sound event.
    Event,
}
