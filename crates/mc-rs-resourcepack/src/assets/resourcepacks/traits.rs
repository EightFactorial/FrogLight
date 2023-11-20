use bevy::prelude::*;
use mc_rs_core::ResourceLocation;

use crate::pack::ResourcePackAsset;

use super::ResourcePacks;

/// A trait for getting textures from the world.
pub trait TextureFromWorld {
    /// Gets the texture from the world if it exists.
    fn get_texture(&self, texture: impl Into<ResourceLocation>) -> Option<&Handle<Image>>;
    /// Gets the texture from the world, or the fallback texture if it doesn't exist.
    fn get_texture_or_fallback(&self, texture: impl Into<ResourceLocation>) -> &Handle<Image>;
}

impl TextureFromWorld for World {
    fn get_texture(&self, texture: impl Into<ResourceLocation>) -> Option<&Handle<Image>> {
        let packs = self.resource::<ResourcePacks>();
        let assets = self.resource::<Assets<ResourcePackAsset>>();
        packs.get_texture(&texture.into(), assets)
    }

    fn get_texture_or_fallback(&self, texture: impl Into<ResourceLocation>) -> &Handle<Image> {
        let packs = self.resource::<ResourcePacks>();
        let assets = self.resource::<Assets<ResourcePackAsset>>();
        packs.get_texture_or_fallback(&texture.into(), assets)
    }
}
