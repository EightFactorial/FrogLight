use bevy::prelude::*;
use froglight_assets::assets::ResourcePack;
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

use super::{LanguageManager, ParticleManager, SoundManager};
use crate::assets::{AssetLoading, ResourcePackSettings};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<AssetManager>()
        .register_type::<AssetManager>()
        .init_resource::<AssetManagerState>()
        .register_type::<AssetManagerState>();

    app.add_systems(
        OnEnter(AssetLoading::Loading),
        AssetManager::reset_asset_manager.run_if(resource_exists::<AssetManager>),
    );
    app.add_systems(
        Update,
        AssetManager::populate_asset_manager
            .run_if(not(AssetManager::is_finished))
            .run_if(resource_exists::<AssetManager>)
            .ambiguous_with(LanguageManager::populate_language_manager)
            .ambiguous_with(ParticleManager::populate_particle_manager)
            .ambiguous_with(SoundManager::populate_sound_manager)
            .in_set(AssetLoading::Processing),
    );
}

/// A [`Resource`] for managing [`sound`](AudioSource) and [`texture`](Image)
/// assets.
#[derive(Debug, Default, Clone, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct AssetManager {
    /// Sounds
    pub sounds: HashMap<ResourceKey, Handle<AudioSource>>,
    /// Textures
    pub textures: HashMap<ResourceKey, Handle<Image>>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Resource, Reflect)]
#[reflect(Default, Resource)]
#[allow(unreachable_pub)]
pub struct AssetManagerState {
    finished: bool,
    current: usize,
}

impl AssetManager {
    /// Returns `true` if the [`AssetManager`] has finished loading all assets.
    #[must_use]
    pub fn is_finished(state: Res<AssetManagerState>) -> bool { state.finished }

    /// Resets the [`AssetManager`] to its initial state.
    fn reset_asset_manager(
        mut manager: ResMut<AssetManager>,
        mut state: ResMut<AssetManagerState>,
    ) {
        manager.sounds.clear();
        manager.textures.clear();
        state.finished = false;
        state.current = 0;
    }

    /// Populates the [`AssetManager`] with assets from currently loaded
    /// [`ResourcePack`]s.
    ///
    /// Does not rely on any other asset managers.
    pub(crate) fn populate_asset_manager(
        settings: Res<ResourcePackSettings>,
        mut assets: ResMut<Assets<ResourcePack>>,
        mut manager: ResMut<AssetManager>,
        mut state: ResMut<AssetManagerState>,
    ) {
        // Get the current `ResourcePack` from the list
        if let Some(pack_item) = settings.resourcepacks.get(state.current) {
            // If the `ResourcePack` has a handle
            if let Some(pack_handle) = pack_item.handle.as_ref() {
                // Access the `ResourcePack` data
                if let Some(resourcepack) = assets.get_mut(pack_handle) {
                    // Take the sounds and textures from the `ResourcePack`,
                    // if they don't already exist.
                    for (resourcekey, sound_handle) in std::mem::take(&mut resourcepack.sounds) {
                        manager.sounds.entry(resourcekey).or_insert(sound_handle);
                    }
                    for (resourcekey, texture_handle) in std::mem::take(&mut resourcepack.textures)
                    {
                        manager.textures.entry(resourcekey).or_insert(texture_handle);
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
            debug!(
                "Loaded \"{}\" sounds and \"{}\" textures",
                manager.sounds.len(),
                manager.textures.len()
            );

            state.finished = true;
        }
    }
}
