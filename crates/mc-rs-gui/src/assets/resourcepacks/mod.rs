use bevy::{asset::embedded_asset, prelude::*};
use compact_str::CompactString;
use mc_rs_core::ResourceLocation;
use mc_rs_resourcepack::ResourcePackAsset;

mod traits;
pub use traits::TextureFromWorld;

pub(super) fn setup(app: &mut App) {
    embedded_asset!(app, "fallback.png");
    app.add_systems(PreStartup, ResourcePacks::initialize);
}

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
    /// Adds the [ResourcePacks] resource to the world at startup.
    fn initialize(assets: Res<AssetServer>, mut commands: Commands) {
        let fallback: Handle<Image> =
            assets.load("embedded://mc_rs_gui/assets/resourcepacks/fallback.png");

        commands.insert_resource(ResourcePacks {
            fallback,
            list: Vec::new(),
        });
    }

    pub fn get_texture<'a>(
        &'a self,
        texture: &ResourceLocation,
        assets: &'a Assets<ResourcePackAsset>,
    ) -> Option<&Handle<Image>> {
        for pack in self.list.iter() {
            if let Some(pack) = assets.get(&pack.handle) {
                if let Some(texture) = pack.textures.get(texture) {
                    return Some(texture);
                }
            }
        }

        None
    }

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
