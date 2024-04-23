use bevy_derive::{Deref, DerefMut};
use bevy_log::warn;
use compact_str::CompactString;
use froglight_protocol::common::{ResourceKey, ResourceKeyError};
use hashbrown::HashMap;
use serde::Deserialize;

/// A map of sounds that can be played in the game.
#[derive(Debug, Default, Clone, PartialEq, Deref, DerefMut, Deserialize)]
#[serde(transparent)]
pub struct SoundMap(pub(crate) HashMap<CompactString, SoundItem>);

impl SoundMap {
    /// Returns a random [`ResourceKey`] for the given sound.
    pub fn get_random_key<'a>(&'a self, key: &str) -> Option<&'a ResourceKey> {
        if let Some(item) = self.0.get(key) {
            // Total together the weights of all sounds.
            let mut total_weight = 0u32;
            for sound in &item.sounds {
                total_weight += match sound {
                    SoundKind::FileOptions(options) => options.weight,
                    SoundKind::File(_) => 1,
                };
            }

            // Generate a random number between 0 and the total weight.
            let mut selected_weight = fastrand::choice(0..total_weight)?;

            // Find the sound that corresponds to the selected weight.
            for sound in &item.sounds {
                match sound {
                    SoundKind::FileOptions(options) => {
                        selected_weight = selected_weight.saturating_sub(options.weight);
                        if selected_weight == 0 {
                            return Some(&options.name);
                        }
                    }
                    SoundKind::File(key) => {
                        selected_weight = selected_weight.saturating_sub(1);
                        if selected_weight == 0 {
                            return Some(key);
                        }
                    }
                }
            }

            // If we reach this point, we failed to pick a sound.
            warn!("Failed to pick a Sound for: \"{key}\"");
        }

        None
    }

    /// Returns a subtitle for the given sound key.
    ///
    /// Will return the key itself if no subtitle is found.
    #[must_use]
    pub fn get_subtitle<'a>(&'a self, key: &'a str) -> &'a str {
        if let Some(SoundItem { subtitle: Some(subtitle), .. }) = self.0.get(key) {
            subtitle.as_str()
        } else {
            key
        }
    }
}

/// A sound that can be played in the game.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SoundItem {
    pub sounds: Vec<SoundKind>,
    pub subtitle: Option<CompactString>,
}

/// A type of sound
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum SoundKind {
    /// A sound
    File(SoundKey),
    /// A sound with options.
    FileOptions(SoundOptions),
}

/// A sound with options.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SoundOptions {
    pub name: SoundKey,
    #[serde(default = "SoundOptions::default_volume")]
    pub volume: f32,
    #[serde(default = "SoundOptions::default_pitch")]
    pub pitch: f32,
    #[serde(default = "SoundOptions::default_weight")]
    pub weight: u32,
}

impl SoundOptions {
    const fn default_volume() -> f32 { 1.0 }
    const fn default_pitch() -> f32 { 1.0 }
    const fn default_weight() -> u32 { 1 }
}

/// A key for a sound.
///
/// Is equivalent to
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Deserialize)]
#[serde(try_from = "CompactString")]
pub struct SoundKey(pub ResourceKey);

impl From<SoundKey> for ResourceKey {
    fn from(key: SoundKey) -> Self { key.0 }
}

impl From<ResourceKey> for SoundKey {
    fn from(key: ResourceKey) -> Self { Self(key) }
}

impl TryFrom<CompactString> for SoundKey {
    type Error = ResourceKeyError;
    fn try_from(value: CompactString) -> Result<Self, Self::Error> {
        if value.contains(':') {
            // If the value contains a colon, it should be a `ResourceKey`.
            Ok(Self(ResourceKey::try_new(value)?))
        } else {
            // Otherwise, convert the value into a `ResourceKey`.
            // (This will add a prefix of "minecraft:sounds/")
            Ok(Self(ResourceKey::try_new(format!("sounds/{value}"))?))
        }
    }
}
