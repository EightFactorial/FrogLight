use bevy::{asset::embedded_asset, prelude::*};
use froglight_assets::assets::ResourcePack;
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

use super::{LanguageManager, ParticleManager, SoundManager};
use crate::assets::{AssetLoading, ResourcePackSettings};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Embed the fallback texture asset
    embedded_asset!(app, "assets/fallback.png");

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
#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct AssetManager {
    /// Sounds
    pub sounds: HashMap<ResourceKey, Handle<AudioSource>>,
    /// Textures
    pub textures: HashMap<ResourceKey, Handle<Image>>,
}

impl FromWorld for AssetManager {
    fn from_world(world: &mut World) -> Self {
        let mut textures = HashMap::with_capacity(1);

        // Load and insert the fallback texture
        {
            let handle = if let Some(asset_server) = world.get_resource::<AssetServer>() {
                asset_server.load(
                    "embedded://froglight_client/assets/managers/asset_manager/assets/fallback.png",
                )
            } else {
                Handle::default()
            };
            textures.insert(Self::FALLBACK_TEXTURE.clone(), handle);
        }

        Self { sounds: HashMap::default(), textures }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Resource, Reflect)]
#[reflect(Default, Resource)]
#[allow(unreachable_pub)]
pub struct AssetManagerState {
    finished: bool,
    current: usize,
}

impl AssetManager {
    /// The [`ResourceKey`] for the fallback texture.
    pub const FALLBACK_TEXTURE: ResourceKey = ResourceKey::new_inline("froglight:fallback");

    /// Returns `true` if the [`AssetManager`] has finished loading all assets.
    #[must_use]
    pub fn is_finished(state: Res<AssetManagerState>) -> bool { state.finished }

    /// Resets the [`AssetManager`] to its initial state.
    fn reset_asset_manager(
        mut manager: ResMut<AssetManager>,
        mut state: ResMut<AssetManagerState>,
    ) {
        manager.textures.retain(|key, _| key == &Self::FALLBACK_TEXTURE);

        manager.sounds.clear();
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

    /// Returns the texture handle for the given [`ResourceKey`], or the
    /// [`AssetManager::FALLBACK_TEXTURE`].
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn get_texture_or_fallback(&self, key: &ResourceKey) -> &Handle<Image> {
        self.textures.get(key).unwrap_or_else(|| {
            #[cfg(debug_assertions)]
            warn!("Unable to find texture for: \"{key}\"");
            self.textures.get(&Self::FALLBACK_TEXTURE).expect("Fallback texture not found")
        })
    }
}
