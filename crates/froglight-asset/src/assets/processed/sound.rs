//! [`SoundMap`] and related types.

use bevy_app::App;
use bevy_asset::Handle;
use bevy_audio::AudioSource;
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{reflect::ReflectResource, system::Resource};
use bevy_log::warn;
use bevy_prng::SeedableEntropySource;
use bevy_rand::prelude::GlobalEntropy;
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_utils::HashMap;
use froglight_common::ResourceKey;
use rand::Rng;

use crate::{
    assets::raw::sound::{DefinitionEntry, DefinitionFile, DefinitionType, SoundDefinition},
    AssetCatalog,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<SoundMap>();
    app.init_resource::<SoundMap>();
}

/// A map containing all the sounds in the game.
#[derive(Debug, Default, Clone, PartialEq, Deref, DerefMut, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct SoundMap(HashMap<ResourceKey, SoundSet>);

impl SoundMap {
    /// Retrieves an audio handle and settings for a sound key.
    #[must_use]
    #[inline]
    pub fn get_audio<R: SeedableEntropySource>(
        &self,
        key: &str,
        entropy: &mut GlobalEntropy<R>,
    ) -> Option<(&Handle<AudioSource>, SoundSettings)> {
        self.get_audio_recursive(key, 0, entropy)
    }

    /// The maximum depth to recurse when retrieving audio.
    const DEPTH_LIMIT: usize = 8;

    /// Retrieves an audio handle and settings for a sound key, recursively.
    ///
    /// Only recurses up to [`SoundMap::DEPTH_LIMIT`] to prevent infinite loops.
    #[must_use]
    fn get_audio_recursive<R: SeedableEntropySource>(
        &self,
        key: &str,
        depth: usize,
        entropy: &mut GlobalEntropy<R>,
    ) -> Option<(&Handle<AudioSource>, SoundSettings)> {
        let entry = self.0.get(key).and_then(|set| set.random_entry(entropy))?;
        match &entry.reference {
            // Return the handle and settings
            SoundEntryRef::Handle(handle) => Some((handle, entry.settings)),
            // Recurse using the reference, and combine the settings
            SoundEntryRef::Entry(reference) => {
                if depth < Self::DEPTH_LIMIT {
                    self.get_audio_recursive(reference.as_str(), depth + 1, entropy)
                        .map(|(h, s)| (h, s.join(&entry.settings)))
                } else {
                    warn!("SoundMap: Depth limit reached for sound key '{key}'");
                    None
                }
            }
        }
    }
}

/// A set of sounds that can be played.
///
/// One will randomly be selected by weight.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
#[reflect(Default)]
pub struct SoundSet {
    /// A subtitle to display when playing a sound from the set.
    pub subtitle: Option<String>,

    /// The list of sounds in the set.
    pub entries: Vec<SoundEntry>,

    /// The total weight of all the sounds in the set.
    pub total_weight: u32,
}

impl SoundSet {
    /// Retrieves a random entry from the [`SoundSet`].
    ///
    /// Returns `None` if the [`SoundSet`] is empty.
    #[must_use]
    pub fn random_entry<'a, R: SeedableEntropySource>(
        self: &'a SoundSet,
        entropy: &mut GlobalEntropy<R>,
    ) -> Option<&'a SoundEntry> {
        // Get a random entry from the sound set.
        let mut random = entropy.gen_range(0..=self.total_weight);
        self.entries.iter().find(|&e| {
            random = random.saturating_sub(e.weight);
            random == 0
        })
    }

    /// Creates a [`SoundSet`] from a [`SoundDefinition`].
    ///
    /// Skips any [`DefinitionEntry`]s that fail to convert to [`SoundEntry`]s.
    #[must_use]
    pub fn from_definition(def: &SoundDefinition, catalog: &AssetCatalog) -> Self {
        // Convert `DefinitionEntry`s to `SoundEntry`s.
        let mut entries = Vec::with_capacity(def.sounds.len());
        for sound_def in &def.sounds {
            if let Some(entry) = SoundEntry::from_definition(sound_def, catalog) {
                entries.push(entry);
            } else {
                warn!("SoundSet: Failed to create SoundEntry from SoundDefinition");
                #[cfg(debug_assertions)]
                bevy_log::debug!("SoundDefinition: {sound_def:?}");
            }
        }

        // Calculate the total weight of all the sounds and return the `SoundSet`.
        let total_weight = entries.iter().map(|e| e.weight).sum();
        Self { subtitle: def.subtitle.clone(), entries, total_weight }
    }
}

/// An entry in a [`SoundSet`].
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct SoundEntry {
    /// The reference to the sound to play.
    pub reference: SoundEntryRef,
    /// The settings to use when playing the sound.
    pub settings: SoundSettings,

    /// The weight of the sound when randomly selecting a sound to play.
    pub weight: u32,
}

impl SoundEntry {
    /// The default weight of a [`SoundEntry`].
    pub const DEFAULT_WEIGHT: u32 = 1;

    /// Creates a [`SoundEntry`] from a [`DefinitionEntry`].
    #[allow(clippy::cast_sign_loss)]
    #[must_use]
    pub fn from_definition(def: &DefinitionEntry, catalog: &AssetCatalog) -> Option<Self> {
        let sound_key = ResourceKey::try_new(def.name()).ok()?;
        match def {
            DefinitionEntry::SoundPath(..) => Some(Self {
                reference: SoundEntryRef::Handle(catalog.get(&sound_key)?),
                settings: SoundSettings::default(),
                weight: Self::DEFAULT_WEIGHT,
            }),
            DefinitionEntry::SoundFile(file) => Some(Self {
                reference: match def.sound_type() {
                    DefinitionType::File => SoundEntryRef::Handle(catalog.get(&sound_key)?),
                    DefinitionType::Event => SoundEntryRef::Entry(sound_key),
                },
                settings: SoundSettings::from(file),
                weight: file.weight() as u32,
            }),
        }
    }
}

/// A reference to either a sound [`Handle`]
/// or another [`SoundSet`] in the [`SoundMap`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum SoundEntryRef {
    /// An [`AudioSource`] [`Handle`].
    Handle(Handle<AudioSource>),
    /// The key to another [`SoundSet`]
    Entry(ResourceKey),
}

/// Settings used when playing a sound.
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[reflect(Default)]
pub struct SoundSettings {
    /// The volume of the sound.
    pub volume: Option<f32>,

    /// The pitch of the sound.
    pub pitch: Option<f32>,

    /// The distance at which the sound starts to decrease in volume.
    pub attenuation_distance: Option<u32>,
}

impl SoundSettings {
    /// The default volume of the sound.
    pub const DEFAULT_VOLUME: f32 = 1.0f32;
    /// The default pitch of the sound.
    pub const DEFAULT_PITCH: f32 = 1.0f32;
    /// The default distance at which the sound starts to decrease in volume.
    pub const DEFAULT_ATTENUATION: u32 = 16u32;

    /// The volume of the sound.
    #[must_use]
    pub fn volume(&self) -> f32 { self.volume.unwrap_or(Self::DEFAULT_VOLUME) }

    /// The pitch of the sound.
    #[must_use]
    pub fn pitch(&self) -> f32 { self.pitch.unwrap_or(Self::DEFAULT_PITCH) }

    /// The distance at which the sound starts to decrease in volume.
    #[must_use]
    pub fn attenuation_distance(&self) -> u32 {
        self.attenuation_distance.unwrap_or(Self::DEFAULT_ATTENUATION)
    }

    /// Joins two [`SoundSettings`] together, preferring the current settings.
    #[must_use]
    pub fn join(&self, other: &Self) -> Self {
        Self {
            volume: self.volume.or(other.volume),
            pitch: self.pitch.or(other.pitch),
            attenuation_distance: self.attenuation_distance.or(other.attenuation_distance),
        }
    }
}

impl From<DefinitionFile> for SoundSettings {
    fn from(value: DefinitionFile) -> Self { Self::from(&value) }
}
#[allow(clippy::cast_sign_loss)]
impl From<&DefinitionFile> for SoundSettings {
    fn from(value: &DefinitionFile) -> Self {
        Self {
            volume: value.volume,
            pitch: value.pitch,
            attenuation_distance: value.attenuation_distance.map(|d| d as u32),
        }
    }
}
