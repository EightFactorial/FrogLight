use bevy::{
    prelude::{Assets, Handle, Image, World},
    sprite::TextureAtlas,
};
use mc_rs_core::ResourceLocation;

use crate::{
    resourcepacks::ResourcePackAsset,
    resources::resourcepacks::{AtlasKind, ResourcePacks},
};

pub trait AssetFromWorld {
    /// Get a texture from the loaded resource packs
    ///
    /// If the texture is not found, this will return the fallback texture
    fn get_texture(&self, resource: &ResourceLocation) -> &Handle<Image>;

    /// Optionally get a texture from the loaded resource packs
    fn try_get_texture(&self, resource: &ResourceLocation) -> Option<&Handle<Image>>;

    /// Optionally get a texture atlas from the loaded resource packs
    fn try_get_atlas(&self, kind: AtlasKind) -> Option<&Handle<TextureAtlas>>;
}

impl AssetFromWorld for World {
    fn get_texture(&self, resource: &ResourceLocation) -> &Handle<Image> {
        let packs = self.resource::<ResourcePacks>();
        let assets = self.resource::<Assets<ResourcePackAsset>>();

        packs.get_texture(resource, assets)
    }

    fn try_get_texture(&self, resource: &ResourceLocation) -> Option<&Handle<Image>> {
        let packs = self.resource::<ResourcePacks>();
        let assets = self.resource::<Assets<ResourcePackAsset>>();

        packs.try_get_texture(resource, assets)
    }

    fn try_get_atlas(&self, kind: AtlasKind) -> Option<&Handle<TextureAtlas>> {
        let packs = self.resource::<ResourcePacks>();

        packs.try_get_atlas(&kind)
    }
}
