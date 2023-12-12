use bevy::{
    asset::{embedded_asset, RecursiveDependencyLoadState},
    prelude::*,
};
use compact_str::CompactString;
use mc_rs_core::ResourceLocation;

mod traits;
pub use traits::AssetFromWorld;

use crate::pack::{asset::model::Model, ResourcePackAsset};

pub(super) fn setup(app: &mut App) {
    embedded_asset!(app, "embedded/fallback.png");
    app.add_systems(PreStartup, ResourcePacks::initialize);
}

/// A collection of all of the loaded resource packs
///
/// Textures are loaded from the list in reverse order,
/// so the last pack in the list has the highest priority.
#[derive(Debug, Clone, PartialEq, Eq, Resource)]
pub struct ResourcePacks {
    pub fallback: Handle<Image>,
    pub list: Vec<ResourcePackContainer>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourcePackContainer {
    pub path: CompactString,
    pub handle: Handle<ResourcePackAsset>,
}

impl ResourcePacks {
    /// A [bevy] system that adds the [`ResourcePacks`] resource to the world at startup.
    fn initialize(assets: Res<AssetServer>, mut commands: Commands) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Initializing ResourcePacks");

        let fallback: Handle<Image> =
            assets.load("embedded://mc_rs_resourcepack/assets/resourcepacks/embedded/fallback.png");

        commands.insert_resource(ResourcePacks {
            fallback,
            list: Vec::new(),
        });
    }

    /// A [bevy] system that returns `true` if all of the [`ResourcePackAsset`]s are loaded.
    #[must_use]
    pub fn loaded(packs: Res<ResourcePacks>, assets: Res<AssetServer>) -> bool {
        packs.list.iter().all(|pack| {
            let state = assets.get_recursive_dependency_load_state(&pack.handle);

            !matches!(
                state,
                Some(RecursiveDependencyLoadState::NotLoaded)
                    | Some(RecursiveDependencyLoadState::Loading)
            )
        })
    }

    /// Get a texture from the list of resource packs.
    ///
    /// Loops through the list in reverse order,
    /// so the last pack in the list has the highest priority.
    #[must_use]
    pub fn get_texture<'a>(
        &'a self,
        texture: &ResourceLocation,
        assets: &'a Assets<ResourcePackAsset>,
    ) -> Option<&Handle<Image>> {
        for pack in self.list.iter().rev() {
            if let Some(pack) = assets.get(&pack.handle) {
                if let Some(texture) = pack.textures.get(texture) {
                    return Some(texture);
                }
            }
        }

        None
    }

    /// Get a texture from the list of resource packs, or the fallback if it doesn't exist.
    ///
    /// Loops through the list in reverse order,
    /// so the last pack in the list has the highest priority.
    #[must_use]
    pub fn get_texture_or_fallback<'a>(
        &'a self,
        texture: &ResourceLocation,
        assets: &'a Assets<ResourcePackAsset>,
    ) -> &Handle<Image> {
        self.get_texture(texture, assets).unwrap_or_else(|| {
            #[cfg(any(debug_assertions, feature = "debug"))]
            warn!("Texture {} not found, using fallback", texture.to_string());

            &self.fallback
        })
    }

    /// Get a model from the list of resource packs.
    ///
    /// Loops through the list in reverse order,
    /// so the last pack in the list has the highest priority.
    #[must_use]
    pub fn get_model<'a>(
        &'a self,
        model: &ResourceLocation,
        assets: &'a Assets<ResourcePackAsset>,
    ) -> Option<&Model> {
        for pack in self.list.iter().rev() {
            if let Some(pack) = assets.get(&pack.handle) {
                if let Some(model) = pack.models.get(model) {
                    return Some(model);
                }
            }
        }

        None
    }

    /// Get a texture for a model from the list of resource packs.
    ///
    /// This will check all resource packs for the model, and then
    /// check all resource packs for the texture.
    ///
    /// Loops through the list in reverse order,
    /// so the last pack in the list has the highest priority.
    #[must_use]
    pub fn get_model_texture<'a>(
        &'a self,
        model: &ResourceLocation,
        name: &str,
        assets: &'a Assets<ResourcePackAsset>,
    ) -> Option<&Handle<Image>> {
        for pack in self.list.iter().rev() {
            if let Some(pack) = assets.get(&pack.handle) {
                if let Some(model) = pack.models.get(model) {
                    if let Some(texture) = model.get_texture(name, &pack.models, None) {
                        return self.get_texture(&texture, assets);
                    }
                }
            }
        }

        None
    }

    /// Get a sound from the list of resource packs.
    ///
    /// Loops through the list in reverse order,
    /// so the last pack in the list has the highest priority.
    #[must_use]
    pub fn get_sound<'a>(
        &'a self,
        sound: &ResourceLocation,
        assets: &'a Assets<ResourcePackAsset>,
    ) -> Option<&Handle<AudioSource>> {
        for pack in self.list.iter().rev() {
            if let Some(pack) = assets.get(&pack.handle) {
                if let Some(sound) = pack.sounds.get(sound) {
                    return Some(sound);
                }
            }
        }

        None
    }
}
