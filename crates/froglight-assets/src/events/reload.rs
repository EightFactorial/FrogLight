use bevy_app::{App, Update};
use bevy_asset::Assets;
use bevy_ecs::{
    event::Event,
    schedule::{common_conditions::on_event, IntoSystemConfigs, NextState},
    system::ResMut,
};
use bevy_log::info;
use froglight_core::systemsets::AssetUpdateSet;

use crate::{assets::resourcepack::ResourcePack, AssetLoadingState, AssetManager};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Add the `ReloadAssets` event
    app.add_event::<ReloadAssets>();

    // Add the `reload_assets_event` system
    app.add_systems(
        Update,
        ReloadAssets::reload_assets_event
            .ambiguous_with_all()
            .run_if(on_event::<ReloadAssets>())
            .in_set(AssetUpdateSet),
    );
}

/// An event that reloads all assets.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct ReloadAssets;

impl ReloadAssets {
    pub(crate) fn reload_assets_event(
        manager: ResMut<AssetManager>,
        mut assets: ResMut<Assets<ResourcePack>>,
        mut state: ResMut<NextState<AssetLoadingState>>,
    ) {
        info!("Reloading assets...");

        // Clear the `AssetManager`
        {
            // Clear all resourcepacks
            manager.resourcepacks.write().clear();

            // Clear all loaded blocks
            manager.blocks.write().clear();
            // Clear all loaded textures
            manager.textures.write().clear();
            // Clear all loaded sounds
            manager.audio.write().clear();
            manager.sounds.write().clear();
        }

        // Clear all loaded `ResourcePacks`
        {
            for id in assets.ids().collect::<Vec<_>>() {
                assets.remove(id);
            }
        }

        // Enter the LoadingResources state
        state.set(AssetLoadingState::LoadingResources);
    }
}
