use bevy_app::App;
use bevy_derive::{Deref, DerefMut};
use bevy_reflect::{std_traits::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use froglight_components::resourcekey::ResourceKey;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<SoundDefinitions>()
        .register_type::<SoundDefinition>()
        .register_type::<SoundObject>()
        .register_type::<SoundSettings>()
        .register_type::<SoundType>();
}

/// A collection of sound definitions.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Deref, DerefMut, Reflect)]
#[reflect(Default, Serialize, Deserialize)]

pub struct SoundDefinitions(pub HashMap<String, SoundDefinition>);

/// A sound definition.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Default, Serialize, Deserialize)]
pub struct SoundDefinition {
    /// If the sound should replace an existing sound.
    #[serde(
        default = "SoundDefinition::default_replace",
        skip_serializing_if = "SoundDefinition::is_default_replace"
    )]
    pub replace: bool,
    /// The language key for the sound.
    ///
    /// This is used to look up the subtitle in the language file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    /// A list of sounds objects.
    ///
    /// The probability of a sound object being selected
    /// is based on each object's `weight` field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sounds: Vec<SoundObject>,
}

impl SoundDefinition {
    const fn default_replace() -> bool { false }
    #[allow(clippy::trivially_copy_pass_by_ref)]
    const fn is_default_replace(b: &bool) -> bool { *b == Self::default_replace() }

    /// Get a random sound object, if any.
    ///
    /// The probability of a sound object being selected
    /// is based on the `weight` field.
    #[must_use]
    pub fn get_sound(&self) -> Option<&SoundObject> {
        if self.sounds.is_empty() {
            return None;
        }

        // Calculate the total weight of all sounds and pick a random weight.
        let total_weight = self.sounds.iter().map(SoundObject::weight).sum();
        let random_weight = fastrand::choice(0..total_weight).unwrap_or_default();

        // Iterate over all sounds and return when the random weight is reached.
        let mut current_weight = 0;
        for sound in &self.sounds {
            current_weight += sound.weight();
            if random_weight < current_weight {
                return Some(sound);
            }
        }

        None
    }
}

/// A sound object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SoundObject {
    /// A path to a sound file.
    Path(ResourceKey),
    /// A path to a sound file with settings.
    PathSettings(ResourceKey, SoundSettings),
    /// A sound with settings.
    Settings(SoundSettings),
}

impl SoundObject {
    /// Get the name of the sound object.
    #[must_use]
    pub fn name(&self) -> &ResourceKey {
        match self {
            SoundObject::Path(name) | SoundObject::PathSettings(name, _) => name,
            SoundObject::Settings(settings) => &settings.name,
        }
    }

    /// Get the volume of the sound object.
    ///
    /// If the volume is not set, the default volume `1.0` is returned.
    #[must_use]
    pub fn volume(&self) -> f32 {
        match self {
            SoundObject::Path(_) => SoundSettings::default_volume(),
            SoundObject::PathSettings(_, settings) | SoundObject::Settings(settings) => {
                settings.volume
            }
        }
    }

    /// Get the pitch of the sound object.
    ///
    /// If the pitch is not set, the default pitch `1.0` is returned.
    #[must_use]
    pub fn pitch(&self) -> f32 {
        match self {
            SoundObject::Path(_) => SoundSettings::default_pitch(),
            SoundObject::PathSettings(_, settings) | SoundObject::Settings(settings) => {
                settings.pitch
            }
        }
    }

    /// Get the weight of the sound object.
    ///
    /// If the weight is not set, the default weight `1` is returned.
    #[must_use]
    pub fn weight(&self) -> i32 {
        match self {
            SoundObject::Path(_) => SoundSettings::default_weight(),
            SoundObject::PathSettings(_, settings) | SoundObject::Settings(settings) => {
                settings.weight
            }
        }
    }

    /// Get if the sound object should be streamed.
    ///
    /// If streaming is not set, the default value `false` is returned.
    #[must_use]
    pub fn stream(&self) -> bool {
        match self {
            SoundObject::Path(_) => SoundSettings::default_stream(),
            SoundObject::PathSettings(_, settings) | SoundObject::Settings(settings) => {
                settings.stream
            }
        }
    }

    /// Get the attenuation distance of the sound object.
    ///
    /// If the attenuation distance is not set, the default value `16` is
    /// returned.
    #[must_use]
    pub fn attenuation_distance(&self) -> i32 {
        match self {
            SoundObject::Path(_) => SoundSettings::default_attenuation_distance(),
            SoundObject::PathSettings(_, settings) | SoundObject::Settings(settings) => {
                settings.attenuation_distance
            }
        }
    }

    /// Get if the sound object should be preloaded.
    ///
    /// If preloading is not set, the default value `false` is returned.
    #[must_use]
    pub fn preload(&self) -> bool {
        match self {
            SoundObject::Path(_) => SoundSettings::default_preload(),
            SoundObject::PathSettings(_, settings) | SoundObject::Settings(settings) => {
                settings.preload
            }
        }
    }

    /// Get the kind of the sound object.
    ///
    /// If the kind is not set, the default kind `SoundType::Sound` is returned.
    #[must_use]
    pub fn kind(&self) -> SoundType {
        match self {
            SoundObject::Path(_) => SoundSettings::default_kind(),
            SoundObject::PathSettings(_, settings) | SoundObject::Settings(settings) => {
                settings.kind
            }
        }
    }
}

/// Sound settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct SoundSettings {
    /// The name.
    pub name: ResourceKey,
    /// The volume.
    #[serde(
        default = "SoundSettings::default_volume",
        skip_serializing_if = "SoundSettings::is_default_volume"
    )]
    pub volume: f32,
    /// The pitch.
    #[serde(
        default = "SoundSettings::default_pitch",
        skip_serializing_if = "SoundSettings::is_default_pitch"
    )]
    pub pitch: f32,
    /// The weight.
    ///
    /// The chance of this sound playing over others.
    #[serde(
        default = "SoundSettings::default_weight",
        skip_serializing_if = "SoundSettings::is_default_weight"
    )]
    pub weight: i32,
    /// If the sound should be streamed.
    #[serde(
        default = "SoundSettings::default_stream",
        skip_serializing_if = "SoundSettings::is_default_stream"
    )]
    pub stream: bool,
    /// The reduction rate of volume over distance.
    #[serde(
        default = "SoundSettings::default_attenuation_distance",
        skip_serializing_if = "SoundSettings::is_default_attenuation_distance"
    )]
    pub attenuation_distance: i32,
    /// If the sound should be preloaded.
    #[serde(
        default = "SoundSettings::default_preload",
        skip_serializing_if = "SoundSettings::is_default_preload"
    )]
    pub preload: bool,
    /// The kind of sound.
    #[serde(
        default = "SoundSettings::default_kind",
        skip_serializing_if = "SoundSettings::is_default_kind"
    )]
    pub kind: SoundType,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
impl SoundSettings {
    const fn default_volume() -> f32 { 1.0 }
    fn is_default_volume(f: &f32) -> bool { (*f - Self::default_pitch()).abs() < f32::EPSILON }

    const fn default_pitch() -> f32 { 1.0 }
    fn is_default_pitch(f: &f32) -> bool { (*f - Self::default_pitch()).abs() < f32::EPSILON }

    const fn default_weight() -> i32 { 1 }
    const fn is_default_weight(i: &i32) -> bool { *i == Self::default_weight() }

    const fn default_stream() -> bool { false }
    const fn is_default_stream(b: &bool) -> bool { *b == Self::default_stream() }

    const fn default_attenuation_distance() -> i32 { 16 }
    const fn is_default_attenuation_distance(i: &i32) -> bool {
        *i == Self::default_attenuation_distance()
    }

    const fn default_preload() -> bool { false }
    const fn is_default_preload(b: &bool) -> bool { *b == Self::default_preload() }

    const fn default_kind() -> SoundType { SoundType::Sound }
    fn is_default_kind(k: &SoundType) -> bool { *k == Self::default_kind() }
}

/// The kind of sound.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
#[reflect(Default, Serialize, Deserialize)]
pub enum SoundType {
    /// A sound.
    ///
    /// Causes the `name` of a sound to
    /// be interpreted as a sound file.
    #[default]
    #[serde(rename = "sound")]
    Sound,
    /// A sound event.
    ///
    /// Causes the `name` of a sound to
    /// be interpreted as another sound event.
    #[serde(rename = "event")]
    Event,
}
