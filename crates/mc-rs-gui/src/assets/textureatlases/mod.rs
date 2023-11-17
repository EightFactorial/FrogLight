use bevy::{prelude::*, utils::HashMap};
use mc_rs_core::ResourceLocation;
use strum::{Display, EnumIter};

pub mod atlases;
use atlases::*;

mod traits;
pub use traits::AtlasFromWorld;

pub(super) fn setup(app: &mut App) { app.init_resource::<TextureAtlases>(); }

/// A collection of texture atlases
///
/// Created after loading all of the [`ResourcePack`](crate::assets::resourcepacks::ResourcePacks)s
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct TextureAtlases {
    pub atlases: HashMap<TextureAtlasType, Handle<TextureAtlas>>,
}

impl TextureAtlases {
    pub fn get(&self, atlas: impl Into<TextureAtlasType>) -> Option<&Handle<TextureAtlas>> {
        self.atlases.get(&atlas.into())
    }

    pub fn get_and_index(
        &self,
        atlas: impl Into<TextureAtlasType>,
        index: usize,
    ) -> Option<(&Handle<TextureAtlas>, UiTextureAtlasImage)> {
        self.get(atlas).map(|handle| {
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

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum TextureAtlasType {
    Icons,
}

impl From<TextureAtlasType> for (u32, u32) {
    fn from(value: TextureAtlasType) -> Self {
        match value {
            TextureAtlasType::Icons => IconAtlas::size(),
        }
    }
}

impl From<TextureAtlasType> for ResourceLocation {
    fn from(value: TextureAtlasType) -> Self {
        match value {
            TextureAtlasType::Icons => IconAtlas::path(),
        }
    }
}

impl From<TextureAtlasType> for Vec<Rect> {
    fn from(value: TextureAtlasType) -> Self {
        match value {
            TextureAtlasType::Icons => IconAtlas::coords(),
        }
    }
}
