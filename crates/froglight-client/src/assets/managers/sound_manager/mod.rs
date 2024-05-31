use bevy::{audio::Volume, prelude::*};
use froglight_assets::assets::{ResourcePack, SoundDefinitions};
use hashbrown::hash_map::Entry;

use super::{AssetManager, LanguageManager, ParticleManager};
use crate::{
    assets::{AssetLoading, ResourcePackSettings},
    systemsets::ClientPostUpdateSet,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<SoundManager>()
        .register_type::<SoundManager>()
        .init_resource::<SoundManagerState>()
        .register_type::<SoundManagerState>()
        .add_event::<SoundEvent>();

    app.add_systems(
        OnEnter(AssetLoading::Loading),
        SoundManager::reset_sound_manager.run_if(resource_exists::<SoundManager>),
    );
    app.add_systems(
        Update,
        SoundManager::populate_sound_manager
            .run_if(not(SoundManager::is_finished))
            .run_if(resource_exists::<SoundManager>)
            .ambiguous_with(AssetManager::populate_asset_manager)
            .ambiguous_with(LanguageManager::populate_language_manager)
            .ambiguous_with(ParticleManager::populate_particle_manager)
            .in_set(AssetLoading::Processing),
    );

    app.add_systems(
        PostUpdate,
        SoundEvent::handle_sound_events
            .run_if(on_event::<SoundEvent>())
            .in_set(ClientPostUpdateSet),
    );
}

/// A [`Resource`] for managing sound effects.
#[derive(Debug, Default, Clone, Resource, Deref, DerefMut, Reflect)]
#[reflect(Default, Resource)]
pub struct SoundManager(pub SoundDefinitions);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Resource, Reflect)]
#[reflect(Default, Resource)]
#[allow(unreachable_pub)]
pub struct SoundManagerState {
    pub finished: bool,
    pub current: usize,
}

impl SoundManager {
    /// Returns `true` if the [`SoundManager`] has finished loading all assets.
    #[must_use]
    pub fn is_finished(state: Res<SoundManagerState>) -> bool { state.finished }

    /// Resets the [`SoundManager`] to its initial state.
    fn reset_sound_manager(
        mut manager: ResMut<SoundManager>,
        mut state: ResMut<SoundManagerState>,
    ) {
        manager.0.clear();
        state.finished = false;
        state.current = 0;
    }

    /// Populates the [`SoundManager`] with sound definitions from currently
    /// loaded [`ResourcePack`]s.
    ///
    /// Does not rely on any other asset managers.
    pub(crate) fn populate_sound_manager(
        settings: Res<ResourcePackSettings>,
        mut manager: ResMut<SoundManager>,
        mut state: ResMut<SoundManagerState>,
        mut assets: ResMut<Assets<ResourcePack>>,
    ) {
        // Get the current `ResourcePack` from the list
        if let Some(pack_item) = settings.resourcepacks.get(state.current) {
            // If the `ResourcePack` has a handle
            if let Some(pack_handle) = pack_item.handle.as_ref() {
                // Access the `ResourcePack` data
                if let Some(resourcepack) = assets.get_mut(pack_handle) {
                    // Take the sounds definitions from the `ResourcePack`.
                    for (_, definitions) in std::mem::take(&mut resourcepack.sound_defs) {
                        for (sound_key, definition) in definitions.0 {
                            match manager.0.entry(sound_key) {
                                Entry::Vacant(entry) => {
                                    // Insert the sound definition into the SoundManager
                                    entry.insert(definition);
                                }
                                Entry::Occupied(mut entry) => {
                                    // Replace the existing sound definition if
                                    // the `replace` flag is set
                                    if definition.replace {
                                        entry.insert(definition);
                                    }
                                }
                            }
                        }
                    }
                } else if let Some(path) = &pack_item.path {
                    error!("Failed to access ResourcePack: \"{path}\"");
                } else {
                    error!("Failed to access ResourcePack: #{}", state.current);
                }
            }
        }

        // Increment the current `ResourcePack` index
        state.current += 1;

        // Set the finished flag if all `ResourcePack`s have been loaded
        if state.current >= settings.resourcepacks.len() {
            #[cfg(debug_assertions)]
            debug!("Loaded \"{}\" sound definitions", manager.len());

            state.finished = true;
        }
    }
}

/// A sound event.
///
/// Plays the given sound, if it exists in the [`SoundManager`].
#[derive(Debug, Clone, PartialEq, Event)]
pub struct SoundEvent {
    /// The name of the sound to play.
    pub sound: String,

    /// The position of the sound.
    ///
    /// If `None`, the sound will be played at the listener's position.
    pub position: Option<Vec3>,
}

impl SoundEvent {
    /// Creates a new [`SoundEvent`] with the given key.
    #[must_use]
    pub fn new(name: &str) -> Self { Self { sound: name.to_string(), position: None } }

    /// Creates a new [`SoundEvent`] with the given key and position.
    #[must_use]
    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = Some(position);
        self
    }

    /// Play sounds when [`SoundEvent`]s are received.
    pub(crate) fn handle_sound_events(
        assets: Res<AssetManager>,
        sounds: Res<SoundManager>,

        mut events: EventReader<Self>,
        mut commands: Commands,
    ) {
        for event in events.read() {
            #[cfg(debug_assertions)]
            debug!("SoundEvent: \"{}\"", event.sound);

            if let Some((key, settings)) = sounds.get_sound(&event.sound) {
                if let Some(sound) = assets.sounds.get(key) {
                    #[cfg(debug_assertions)]
                    debug!("Playing: \"{key}\"");

                    let mut audio =
                        AudioBundle { source: sound.clone(), settings: PlaybackSettings::DESPAWN };

                    // Apply sound settings, if available
                    if let Some(settings) = settings {
                        audio.settings.volume = Volume::new(settings.volume);
                        // audio.settings.pitch = settings.pitch;
                    }

                    // Play the sound
                    commands.spawn(audio);
                } else {
                    error!("Failed to find sound asset: \"{key}\"");
                }
            } else {
                warn!("Unexpected Sound: \"{}\"", event.sound);
            }
        }
    }
}
