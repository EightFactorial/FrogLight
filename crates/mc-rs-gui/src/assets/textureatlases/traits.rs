use bevy::prelude::*;

use super::{TextureAtlasType, TextureAtlases};

pub trait AtlasFromWorld {
    /// Gets the texture atlas from the world if it exists.
    fn get_atlas(&self, atlas: impl Into<TextureAtlasType>) -> Option<&Handle<TextureAtlas>>;

    /// Gets the texture atlas and descriptor from the world if it exists.
    fn get_atlas_and_index(
        &self,
        atlas: impl Into<TextureAtlasType>,
        index: usize,
    ) -> Option<(&Handle<TextureAtlas>, UiTextureAtlasImage)>;
}

impl AtlasFromWorld for World {
    fn get_atlas(&self, atlas: impl Into<TextureAtlasType>) -> Option<&Handle<TextureAtlas>> {
        self.resource::<TextureAtlases>().get(&atlas.into())
    }

    fn get_atlas_and_index(
        &self,
        atlas: impl Into<TextureAtlasType>,
        index: usize,
    ) -> Option<(&Handle<TextureAtlas>, UiTextureAtlasImage)> {
        self.get_atlas(atlas).map(|handle| {
            (
                handle,
                UiTextureAtlasImage {
                    index,
                    flip_x: false,
                    flip_y: false,
                },
            )
        })
    }
}
