#![allow(clippy::used_underscore_binding)]

use bevy_asset::{Asset, ReflectAsset};
use bevy_reflect::{prelude::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use bevy_utils::HashMap;
use serde::{Deserialize, Serialize};

/// A map of sounds events to sound assets.
///
/// Only contains sounds used in it's namespace.
///
/// Read from the `assets/{namespace}/sounds.json` file.
#[derive(Debug, Default, Clone, PartialEq, Reflect, Asset, Serialize, Deserialize)]
#[reflect(Default, Asset, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NamespaceSoundMap {
    /// The sound events in the namespace.
    pub events: HashMap<String, SoundEventDefinition>,
}

#[derive(Debug, Default, Clone, PartialEq, Reflect, Serialize, Deserialize)]
pub struct SoundEventDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sounds: Option<Vec<SoundDefinition>>,
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SoundDefinition {
    /// A simple sound.
    Simple(String),
    /// A sound with settings.
    WithSettings {
        /// The sound's name.
        name: String,
        /// Whether the sound name is a file or an event.
        #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
        kind: Option<SoundKind>,

        /// The sound's volume.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        volume: Option<f32>,
        /// The sound's pitch.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pitch: Option<f32>,
        /// The sound's attenuation distance.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        attenuation_distance: Option<i32>,
        /// The sound's weight.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        weight: Option<i32>,

        /// Whether the sound should be streamed from disk.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        stream: Option<bool>,
        /// Whether the sound should be preloaded.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        preload: Option<bool>,
    },
}

impl SoundDefinition {
    /// The default [`SoundKind`] for a [`SoundDefinition`].
    pub const DEFAULT_KIND: SoundKind = SoundKind::File;
    /// The default volume for a [`SoundDefinition`].
    pub const DEFAULT_VOLUME: f32 = 1.0;
    /// The default pitch for a [`SoundDefinition`].
    pub const DEFAULT_PITCH: f32 = 1.0;
    /// The default attenuation distance for a [`SoundDefinition`].
    pub const DEFAULT_ATTENUATION_DISTANCE: i32 = 16;
    /// The default weight for a [`SoundDefinition`].
    pub const DEFAULT_WEIGHT: i32 = 1;
    /// The default stream for a [`SoundDefinition`].
    pub const DEFAULT_STREAM: bool = false;
    /// The default preload for a [`SoundDefinition`].
    pub const DEFAULT_PRELOAD: bool = false;

    /// Gets the name of the sound.
    pub const fn get_name(&self) -> &String {
        match self {
            SoundDefinition::WithSettings { name, .. } | SoundDefinition::Simple(name) => name,
        }
    }

    /// Gets the kind of the sound.
    pub const fn get_kind(&self) -> SoundKind {
        match self {
            SoundDefinition::Simple(_) => SoundKind::File,
            SoundDefinition::WithSettings { kind, .. } => {
                if let Some(kind) = kind {
                    *kind
                } else {
                    Self::DEFAULT_KIND
                }
            }
        }
    }
}

/// Determines whether the sound name is a file or an event.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SoundKind {
    /// The sound name is a file.
    #[default]
    File,
    /// The sound name is an event.
    Event,
}
