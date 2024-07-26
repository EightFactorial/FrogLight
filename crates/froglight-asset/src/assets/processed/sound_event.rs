use bevy_app::App;
use bevy_asset::{Asset, AssetApp, Handle, ReflectAsset, ReflectHandle};
use bevy_audio::AudioSource;
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    reflect::ReflectResource,
    system::{ResMut, Resource},
};
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_state::state::OnExit;
use froglight_common::ResourceKey;

use crate::{
    assets::unprocessed::sound_definition::{SoundDefinition, SoundKind},
    AssetState,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<SoundEventStorage>();

    // Clear the `SoundEventStorage` when assets are unloaded
    app.add_systems(OnExit(AssetState::Loaded), SoundEventStorage::clear);

    // Register `SoundEvent`
    app.register_type::<SoundEvent>()
        .register_type::<Handle<SoundEvent>>()
        .register_type_data::<Handle<SoundEvent>, ReflectHandle>()
        .init_asset::<SoundEvent>();
}

/// A [`Vec`] used to store [`Handle::Strong`] references to [`SoundEvent`]s.
///
/// Without this, [`SoundEvent`]s would unload when they are no longer in use.
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect, Resource, Deref, DerefMut)]
#[reflect(Default, Resource)]
pub(crate) struct SoundEventStorage {
    inner: Vec<Handle<SoundEvent>>,
}
impl SoundEventStorage {
    /// Clear the [`SoundEventStorage`].
    fn clear(mut res: ResMut<Self>) { res.clear() }
}

/// A sound event.
#[derive(Debug, Default, Clone, PartialEq, Reflect, Asset)]
#[reflect(Default, Asset)]
pub struct SoundEvent {
    /// The sound event's subtitle.
    ///
    /// If none, no subtitle is displayed when the sound event is triggered.
    pub subtitle: Option<String>,

    /// The total weight of all sounds in the sound event.
    ///
    /// Used to select a random sound from the pool.
    pub total_weight: i32,

    /// The sound event's sounds.
    ///
    /// When triggered, one of these sounds is selected at random.
    pub sound_pool: Vec<SoundEntry>,
}

/// A sound entry.
///
/// An entry in a [`SoundEvent`]'s sound pool.
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct SoundEntry {
    /// A reference to another [`SoundEvent`] or an [`AudioSource`].
    pub sound_ref: SoundRef,
    /// The reference's name.
    pub sound_ref_name: ResourceKey,

    /// Settings to use when playing the sound.
    pub settings: SoundSettings,
    /// The sound's weight.
    pub weight: i32,
}

/// A reference to a sound.
#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
pub enum SoundRef {
    /// A reference to another [`SoundEvent`].
    Event(Handle<SoundEvent>),
    /// A reference to an [`AudioSource`].
    Audio(Handle<AudioSource>),
}

/// Sound settings.
///
/// When `None`:
/// - [`SoundRef::Event`] should use that event's values
/// - [`SoundRef::Audio`] should use default [`SoundSettings`] values
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[reflect(Default)]
pub struct SoundSettings {
    pub volume: Option<f32>,
    pub pitch: Option<f32>,
    pub attenuation_distance: Option<i32>,
}

impl SoundSettings {
    /// The default volume for a [`SoundEvent`].
    pub const DEFAULT_VOLUME: f32 = 1.0;
    /// The default pitch for a [`SoundEvent`].
    pub const DEFAULT_PITCH: f32 = 1.0;
    /// The default attenuation distance for a [`SoundEvent`].
    pub const DEFAULT_ATTENUATION_DISTANCE: i32 = 16;
    /// The default weight for a [`SoundEvent`].
    pub const DEFAULT_WEIGHT: i32 = 1;
}

impl From<SoundDefinition> for SoundSettings {
    fn from(value: SoundDefinition) -> Self { Self::from(&value) }
}
impl From<&SoundDefinition> for SoundSettings {
    fn from(value: &SoundDefinition) -> Self {
        match (value.get_kind(), value) {
            // Simple files use the default settings
            (SoundKind::File, SoundDefinition::Simple(..)) => SoundSettings {
                volume: Some(Self::DEFAULT_VOLUME),
                pitch: Some(Self::DEFAULT_PITCH),
                attenuation_distance: Some(Self::DEFAULT_ATTENUATION_DISTANCE),
            },
            // Files with settings use the settings provided or the default
            (
                SoundKind::File,
                SoundDefinition::WithSettings { volume, pitch, attenuation_distance, .. },
            ) => SoundSettings {
                volume: volume.or(Some(Self::DEFAULT_VOLUME)),
                pitch: pitch.or(Some(Self::DEFAULT_PITCH)),
                attenuation_distance: attenuation_distance
                    .or(Some(Self::DEFAULT_ATTENUATION_DISTANCE)),
            },
            // Simple events use the referenced event's settings
            (SoundKind::Event, SoundDefinition::Simple(..)) => {
                SoundSettings { volume: None, pitch: None, attenuation_distance: None }
            }
            // Events with settings use the settings provided
            (
                SoundKind::Event,
                SoundDefinition::WithSettings { volume, pitch, attenuation_distance, .. },
            ) => SoundSettings {
                volume: *volume,
                pitch: *pitch,
                attenuation_distance: *attenuation_distance,
            },
        }
    }
}
