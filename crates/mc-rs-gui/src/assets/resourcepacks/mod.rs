use bevy::{
    asset::{embedded_asset, RecursiveDependencyLoadState},
    prelude::*,
};
use compact_str::CompactString;
use mc_rs_core::ResourceLocation;
use mc_rs_resourcepack::ResourcePackAsset;

mod traits;
pub use traits::TextureFromWorld;

pub(super) fn setup(app: &mut App) {
    embedded_asset!(app, "fallback.png");
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
    /// Add the [ResourcePacks] resource to the world at startup.
    fn initialize(assets: Res<AssetServer>, mut commands: Commands) {
        let fallback: Handle<Image> =
            assets.load("embedded://mc_rs_gui/assets/resourcepacks/fallback.png");

        commands.insert_resource(ResourcePacks {
            fallback,
            list: Vec::new(),
        });
    }

    /// Returns true if all of the [`ResourcePackAsset`]s are loaded.
    pub fn loaded(packs: Res<ResourcePacks>, assets: Res<AssetServer>) -> bool {
        packs.list.iter().all(|pack| {
            let state = assets.get_recursive_dependency_load_state(&pack.handle);

            matches!(state, None | Some(RecursiveDependencyLoadState::Loaded))
        })
    }

    /// Get a texture from the list of resource packs.
    ///
    /// Loops through the list in reverse order,
    /// so the last pack in the list has the highest priority.
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
    pub fn get_texture_or_fallback<'a>(
        &'a self,
        texture: &ResourceLocation,
        assets: &'a Assets<ResourcePackAsset>,
    ) -> &Handle<Image> {
        self.get_texture(texture, assets).unwrap_or_else(|| {
            #[cfg(any(debug_assertions, feature = "debug"))]
            warn!("Texture {:?} not found, using fallback", texture);

            &self.fallback
        })
    }
}
