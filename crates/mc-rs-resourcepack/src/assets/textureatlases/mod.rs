use bevy::{
    asset::RecursiveDependencyLoadState, prelude::*, render::render_resource::TextureFormat,
    utils::HashMap,
};
use mc_rs_core::ResourceLocation;
use strum::{Display, EnumIter, IntoEnumIterator};

pub mod atlases;
use atlases::*;

mod traits;
pub use traits::AtlasFromWorld;

use crate::pack::ResourcePackAsset;

use super::resourcepacks::ResourcePacks;

pub(super) fn setup(app: &mut App) { app.init_resource::<TextureAtlases>(); }

/// A collection of texture atlases
///
/// Created after loading all of the [`ResourcePack`](crate::assets::resourcepacks::ResourcePacks)s
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct TextureAtlases {
    pub atlases: HashMap<TextureAtlasType, Handle<TextureAtlas>>,
}

impl TextureAtlases {
    /// A Bevy system that returns true if all of the [`TextureAtlas`]s are loaded.
    pub fn loaded(atlases: Res<TextureAtlases>, assets: Res<AssetServer>) -> bool {
        atlases.atlases.values().all(|handle| {
            let state = assets.get_recursive_dependency_load_state(handle);

            matches!(state, None | Some(RecursiveDependencyLoadState::Loaded))
        })
    }

    /// A Bevy system that builds all of the [`TextureAtlas`]es from the [`ResourcePackAsset`]s.
    pub fn build(
        packs: Res<ResourcePacks>,
        pack_assets: Res<Assets<ResourcePackAsset>>,

        mut atlases: ResMut<TextureAtlases>,
        mut atlas_assets: ResMut<Assets<TextureAtlas>>,
        mut image_assets: ResMut<Assets<Image>>,
    ) {
        for kind in TextureAtlasType::iter() {
            let path: ResourceLocation = kind.into();
            let coords: Vec<Rect> = kind.into();

            #[cfg(any(debug_assertions, feature = "debug"))]
            trace!("Building TextureAtlasType::{kind} from {path}");

            // Get the image handle
            let Some(handle) = packs.get_texture(&path, &pack_assets) else {
                #[cfg(any(debug_assertions, feature = "debug"))]
                error!("Missing texture for TextureAtlasType::{kind}");
                continue;
            };

            // Get the image
            let Some(image) = image_assets.get(handle) else {
                #[cfg(any(debug_assertions, feature = "debug"))]
                error!("Missing image for TextureAtlasType::{kind}");
                continue;
            };

            // Get the image size and the coordinate size
            let (image_width, image_height) = image.size().into();
            let (coord_width, coord_height) = kind.into();

            // Build the atlas
            let mut builder = TextureAtlasBuilder::default().format(TextureFormat::Rgba8UnormSrgb);
            builder.add_texture(handle.id(), image);

            let mut atlas = match builder.finish(&mut image_assets) {
                Err(err) => {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    error!("Failed to build TextureAtlasType::{kind}, {err}");
                    continue;
                }
                Ok(atlas) => atlas,
            };

            // Add coordinates to the atlas
            for mut coord in coords {
                // Scale the coordinates to the image size
                coord.min.x *= image_width as f32 / coord_width as f32;
                coord.max.x *= image_width as f32 / coord_width as f32;
                coord.min.y *= image_height as f32 / coord_height as f32;
                coord.max.y *= image_height as f32 / coord_height as f32;

                atlas.add_texture(coord);
            }

            // Add the atlas to the list
            let handle = atlas_assets.add(atlas);
            atlases.insert(kind, handle);

            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Added TextureAtlasType::{kind}");
        }
    }

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
    Slider,
    Widget,
}

impl From<TextureAtlasType> for (u32, u32) {
    fn from(value: TextureAtlasType) -> Self {
        match value {
            TextureAtlasType::Icons => IconAtlas::size(),
            TextureAtlasType::Slider => SliderAtlas::size(),
            TextureAtlasType::Widget => WidgetAtlas::size(),
        }
    }
}

impl From<TextureAtlasType> for ResourceLocation {
    fn from(value: TextureAtlasType) -> Self {
        match value {
            TextureAtlasType::Icons => IconAtlas::path(),
            TextureAtlasType::Slider => SliderAtlas::path(),
            TextureAtlasType::Widget => WidgetAtlas::path(),
        }
    }
}

impl From<TextureAtlasType> for Vec<Rect> {
    fn from(value: TextureAtlasType) -> Self {
        match value {
            TextureAtlasType::Icons => IconAtlas::coords(),
            TextureAtlasType::Slider => SliderAtlas::coords(),
            TextureAtlasType::Widget => WidgetAtlas::coords(),
        }
    }
}
