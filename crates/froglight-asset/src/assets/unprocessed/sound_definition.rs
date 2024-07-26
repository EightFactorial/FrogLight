#![allow(clippy::used_underscore_binding)]

use bevy_asset::{Asset, ReflectAsset};
use bevy_derive::{Deref, DerefMut};
use bevy_reflect::{prelude::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use bevy_utils::HashMap;
use serde::{Deserialize, Serialize};

/// A map of sound definitions.
///
/// Only contains sounds used in one namespace.
///
/// Read from the `assets/{namespace}/sounds.json` file.
#[derive(
    Debug, Default, Clone, PartialEq, Reflect, Asset, Serialize, Deserialize, Deref, DerefMut,
)]
#[reflect(Default, Asset, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SoundDefinitionMap {
    /// A map of sound names to sound event definitions.
    events: HashMap<String, SoundEventDefinition>,
}

/// A sound event definition.
#[derive(Debug, Default, Clone, PartialEq, Reflect, Serialize, Deserialize)]
pub struct SoundEventDefinition {
    /// Whether the event should replace existing sounds with the same name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    /// The sound event's subtitle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    /// The sound event's sounds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sounds: Option<Vec<SoundDefinition>>,
}

/// A sound definition.
///
/// May be just the name of an audio file or another sound, can have settings.
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
    /// The default weight for a [`SoundDefinition`].
    pub const DEFAULT_WEIGHT: i32 = 1;

    /// Gets the name of the sound.
    #[must_use]
    pub const fn get_name(&self) -> &String {
        match self {
            SoundDefinition::WithSettings { name, .. } | SoundDefinition::Simple(name) => name,
        }
    }

    /// Gets the kind of the sound.
    #[must_use]
    pub const fn get_kind(&self) -> SoundKind {
        if let SoundDefinition::WithSettings { kind: Some(kind), .. } = self {
            *kind
        } else {
            Self::DEFAULT_KIND
        }
    }

    /// Gets the weight of the sound.
    #[must_use]
    pub const fn get_weight(&self) -> i32 {
        if let SoundDefinition::WithSettings { weight: Some(weight), .. } = self {
            *weight
        } else {
            Self::DEFAULT_WEIGHT
        }
    }
}

/// Whether the sound name points to a file or an event.
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
