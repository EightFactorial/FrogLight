use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use mc_rs_core::sounds::{SoundEvent, SoundEventKind};
use mc_rs_resourcepack::{assets::resourcepacks::ResourcePacks, pack::ResourcePackAsset};
use serde::{Deserialize, Serialize, Serializer};

use super::Settings;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AudioSettings {
    #[serde(
        default = "AudioSettings::default_volume",
        serialize_with = "AudioSettings::clamp_serializer"
    )]
    pub global: f32,
    #[serde(
        default = "AudioSettings::default_volume",
        serialize_with = "AudioSettings::clamp_serializer"
    )]
    pub music: f32,
    #[serde(
        default = "AudioSettings::default_volume",
        serialize_with = "AudioSettings::clamp_serializer"
    )]
    pub jukebox: f32,
    #[serde(
        default = "AudioSettings::default_volume",
        serialize_with = "AudioSettings::clamp_serializer"
    )]
    pub weather: f32,
    #[serde(
        default = "AudioSettings::default_volume",
        serialize_with = "AudioSettings::clamp_serializer"
    )]
    pub block: f32,
    #[serde(
        default = "AudioSettings::default_volume",
        serialize_with = "AudioSettings::clamp_serializer"
    )]
    pub hostile: f32,
    #[serde(
        default = "AudioSettings::default_volume",
        serialize_with = "AudioSettings::clamp_serializer"
    )]
    pub neutral: f32,
    #[serde(
        default = "AudioSettings::default_volume",
        serialize_with = "AudioSettings::clamp_serializer"
    )]
    pub player: f32,
    #[serde(
        default = "AudioSettings::default_volume",
        serialize_with = "AudioSettings::clamp_serializer"
    )]
    pub ambient: f32,
    #[serde(
        default = "AudioSettings::default_volume",
        serialize_with = "AudioSettings::clamp_serializer"
    )]
    pub voice: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            global: AudioSettings::default_volume(),
            music: AudioSettings::default_volume(),
            jukebox: AudioSettings::default_volume(),
            weather: AudioSettings::default_volume(),
            block: AudioSettings::default_volume(),
            hostile: AudioSettings::default_volume(),
            neutral: AudioSettings::default_volume(),
            player: AudioSettings::default_volume(),
            ambient: AudioSettings::default_volume(),
            voice: AudioSettings::default_volume(),
        }
    }
}

impl AudioSettings {
    /// The default volume.
    fn default_volume() -> f32 { 1.0 }

    /// Clamp the volume to the range [0.0, 1.0].
    fn clamp_serializer<S: Serializer>(x: &f32, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_f32(x.clamp(0.0, 1.0))
    }

    /// Update the global volume.
    pub(super) fn update_volume(settings: Res<Settings>, mut volume: ResMut<GlobalVolume>) {
        if settings.audio.global != *volume.volume {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Updating GlobalVolume: {}", settings.audio.global);

            *volume = GlobalVolume::new(settings.audio.global);
        }
    }

    /// Play sound events.
    pub(super) fn sound_events(
        settings: Res<Settings>,
        packs: Res<ResourcePacks>,
        assets: Res<Assets<ResourcePackAsset>>,
        mut events: EventReader<SoundEvent>,
        mut commands: Commands,
    ) {
        events.read().for_each(|event| {
            // Get the volume for the event kind.
            let volume = match event.kind {
                SoundEventKind::Global => 1.0,
                SoundEventKind::Music => settings.audio.music,
                SoundEventKind::Jukebox => settings.audio.jukebox,
                SoundEventKind::Weather => settings.audio.weather,
                SoundEventKind::Block => settings.audio.block,
                SoundEventKind::Hostile => settings.audio.hostile,
                SoundEventKind::Neutral => settings.audio.neutral,
                SoundEventKind::Player => settings.audio.player,
                SoundEventKind::Ambient => settings.audio.ambient,
                SoundEventKind::Voice => settings.audio.voice,
            };

            if let Some(sound) = packs.get_sound(&event.name, &assets) {
                // Create an entity with the sound and volume.
                let mut entity = commands.spawn(AudioBundle {
                    source: sound.clone(),
                    settings: PlaybackSettings {
                        volume: Volume::new_relative(volume),
                        mode: PlaybackMode::Despawn,
                        ..Default::default()
                    },
                });

                // Set the position if it exists.
                if let Some(position) = event.position {
                    entity.insert(TransformBundle::from_transform(
                        Transform::from_translation(position),
                    ));
                }
            }
        });
    }
}
