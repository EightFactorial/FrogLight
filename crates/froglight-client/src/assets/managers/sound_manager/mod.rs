use bevy::prelude::*;
use froglight_assets::assets::{ResourcePack, SoundDefinition, SoundDefinitions, SoundObject};

mod event;
pub use event::SoundEvent;
use hashbrown::hash_map::Entry;

use super::{AssetManager, LanguageManager};
use crate::assets::{AssetLoading, ResourcePackSettings};

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
            .in_set(AssetLoading::Processing),
    );
}

/// A [`Resource`] for managing sound effects.
#[derive(Debug, Default, Clone, Resource, Reflect)]
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
    /// Get the number of sound definitions.
    #[must_use]
    pub fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the [`SoundManager`] is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Get a sound definition by key.
    #[must_use]
    pub fn get_definition(&self, key: &str) -> Option<&SoundDefinition> { self.0.get(key) }

    /// Get a mutable sound definition by key.
    #[must_use]
    pub fn get_definition_mut(&mut self, key: &str) -> Option<&mut SoundDefinition> {
        self.0.get_mut(key)
    }

    /// Get a random sound object by key.
    ///
    /// The sound is randomly selected from the list
    /// of sounds in the [`SoundDefinition`], if it exists.
    #[must_use]
    pub fn get_sound(&self, key: &str) -> Option<&SoundObject> {
        self.get_definition(key).and_then(SoundDefinition::get_sound)
    }

    /// Returns `true` if the [`SoundManager`] has finished loading all assets.
    fn is_finished(state: Res<SoundManagerState>) -> bool { state.finished }

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
