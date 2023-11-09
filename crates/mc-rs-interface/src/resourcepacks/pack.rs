use bevy::{asset::VisitAssetDependencies, prelude::*, utils::HashMap};
use compact_str::CompactString;
use mc_rs_core::ResourceLocation;
use serde::Deserialize;

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

    pub textures: HashMap<ResourceLocation, UntypedHandle>,
}

impl Asset for ResourcePackAsset {}
impl VisitAssetDependencies for ResourcePackAsset {
    fn visit_dependencies(&self, visit: &mut impl FnMut(bevy::asset::UntypedAssetId)) {
        if let Some(icon) = &self.icon {
            visit(icon.id().untyped());
        }

        for texture in self.textures.values() {
            visit(texture.id());
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, Deserialize)]
pub struct ResourcePackMetaContainer {
    pub pack: Option<ResourcePackMeta>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ResourcePackMeta {
    #[serde(rename = "pack_format")]
    pub version: u32,
    pub description: CompactString,

    #[serde(flatten)]
    pub extra: HashMap<CompactString, serde_json::Value>,
}

impl From<ResourcePackMeta> for ResourcePackMetaContainer {
    fn from(pack: ResourcePackMeta) -> Self { Self { pack: Some(pack) } }
}

impl From<Option<ResourcePackMeta>> for ResourcePackMetaContainer {
    fn from(pack: Option<ResourcePackMeta>) -> Self { Self { pack } }
}
