use bevy::{asset::VisitAssetDependencies, prelude::*, utils::HashMap};
use mc_rs_core::ResourceLocation;

pub mod meta;
use meta::ResourcePackMetaContainer;

/// A Minecraft resourcepack. Typically a zip file.
///
/// If loaded from the `resourcepack://` [AssetSource](bevy::asset::io::AssetSource), the following
/// directories are used:
///
/// - Linux: ~/.config/MC-RS/resourcepacks
/// - Windows: %APPDATA%/MC-RS/resourcepacks
#[derive(Debug, Clone, TypePath)]
pub struct ResourcePackAsset {
    pub icon: Option<Handle<Image>>,
    pub mcmeta: ResourcePackMetaContainer,

    pub textures: HashMap<ResourceLocation, Handle<Image>>,
}

impl Asset for ResourcePackAsset {}
impl VisitAssetDependencies for ResourcePackAsset {
    fn visit_dependencies(&self, visit: &mut impl FnMut(bevy::asset::UntypedAssetId)) {
        if let Some(icon) = &self.icon {
            visit(icon.id().untyped());
        }

        for texture in self.textures.values() {
            visit(texture.id().untyped());
        }
    }
}
