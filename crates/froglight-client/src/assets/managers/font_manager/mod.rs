use bevy::prelude::*;
use froglight_assets::assets::{FontDefinition, ResourcePack};
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

use super::AssetManager;
use crate::assets::{AssetLoading, ResourcePackSettings};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<FontManager>()
        .register_type::<FontManager>()
        .init_resource::<FontManagerState>()
        .register_type::<FontManagerState>();

    app.add_systems(
        OnEnter(AssetLoading::Loading),
        FontManager::reset_font_manager.run_if(resource_exists::<FontManager>),
    );
    app.add_systems(
        Update,
        FontManager::populate_font_manager
            .run_if(not(FontManager::is_finished))
            .run_if(resource_exists::<FontManager>)
            .run_if(AssetManager::is_finished)
            .after(AssetManager::populate_asset_manager)
            .in_set(AssetLoading::Processing),
    );
}

/// A [`Resource`] for managing font assets.
#[derive(Default, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct FontManager {
    /// Loaded fonts.
    ///
    /// TODO: This should be a `HashMap<ResourceKey, Handle<Font>>`.
    pub fonts: HashMap<ResourceKey, FontDefinition>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Resource, Reflect)]
#[reflect(Default, Resource)]
#[allow(unreachable_pub)]
pub struct FontManagerState {
    finished: bool,
    current: usize,
}

impl FontManager {
    /// Returns `true` if the [`FontManager`] has finished loading all assets.
    #[must_use]
    pub fn is_finished(state: Res<FontManagerState>) -> bool { state.finished }

    /// Resets the [`FontManager`] to its initial state.
    fn reset_font_manager(mut manager: ResMut<FontManager>, mut state: ResMut<FontManagerState>) {
        manager.fonts.clear();
        state.finished = false;
        state.current = 0;
    }

    /// Populates the [`FontManager`] with fonts.
    ///
    /// Relies on the [`AssetServer`] to load font textures.
    pub(crate) fn populate_font_manager(
        settings: Res<ResourcePackSettings>,
        mut manager: ResMut<FontManager>,
        mut state: ResMut<FontManagerState>,
        mut assets: ResMut<Assets<ResourcePack>>,
    ) {
        // Get the current `ResourcePack` from the list
        if let Some(pack_item) = settings.resourcepacks.get(state.current) {
            // If the `ResourcePack` has a handle
            if let Some(pack_handle) = pack_item.handle.as_ref() {
                // Access the `ResourcePack` data
                if let Some(resourcepack) = assets.get_mut(pack_handle) {
                    // Take the fonts from the `ResourcePack`,
                    // if they don't already exist.
                    for (resourcekey, font) in std::mem::take(&mut resourcepack.fonts) {
                        manager.fonts.entry(resourcekey).or_insert(font);
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
            debug!("Loaded \"{}\" font definitions", manager.fonts.len());

            state.finished = true;
        }
    }
}
