use bevy::{prelude::*, render::render_resource::TextureFormat, utils::HashMap};
use mc_rs_core::ResourceLocation;
use strum::{EnumIter, IntoEnumIterator};

use crate::{resourcepacks::ResourcePackAsset, traits::interface::AtlasData};

use self::gui_icons::GuiIcons;

use super::{events::ResourcePacksFinishEvent, ResourcePacks};

pub mod gui_icons;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum AtlasKind {
    GuiIcons,
}

impl AtlasKind {
    pub fn path(&self) -> ResourceLocation {
        match self {
            Self::GuiIcons => GuiIcons::path(),
        }
    }

    fn coords(&self) -> Vec<Rect> {
        match self {
            Self::GuiIcons => GuiIcons::coords(),
        }
    }

    /// Update the texture atlases.
    pub(super) fn update_atlases(
        mut packs: ResMut<ResourcePacks>,
        assets: Res<Assets<ResourcePackAsset>>,

        mut images: ResMut<Assets<Image>>,
        mut atlases: ResMut<Assets<TextureAtlas>>,

        mut finish_event: EventWriter<ResourcePacksFinishEvent>,
    ) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Updating atlases");

        let new_atlases = Self::create(&packs, &assets, &mut images, &mut atlases);
        packs.atlases = new_atlases;

        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Sending ResourcePacksFinishEvent");
        finish_event.send(ResourcePacksFinishEvent);
    }

    fn create(
        packs: &ResourcePacks,
        assets: &Assets<ResourcePackAsset>,

        images: &mut Assets<Image>,
        atlases: &mut Assets<TextureAtlas>,
    ) -> HashMap<AtlasKind, Handle<TextureAtlas>> {
        let mut pack_atlases = HashMap::with_capacity(AtlasKind::iter().count());

        for kind in AtlasKind::iter() {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Creating atlas for texture: {kind:?}");

            // Get the texture handle
            let Some(handle) = packs.try_get_texture(&kind.path(), assets) else {
                #[cfg(any(debug_assertions, feature = "debug"))]
                warn!("No resource pack has the texture: {}", kind.path());
                continue;
            };

            // Get the texture image
            let Some(image) = images.get(handle) else {
                #[cfg(any(debug_assertions, feature = "debug"))]
                error!("No image for texture: {}", kind.path());
                continue;
            };

            let width = image.width();
            let height = image.height();

            #[cfg(any(debug_assertions, feature = "debug"))]
            if width != height {
                warn!(
                    "Texture is not square: {} ({}x{})",
                    kind.path(),
                    width,
                    height
                );
            }

            // Create the texture atlas
            let mut builder = TextureAtlasBuilder::default().format(TextureFormat::Rgba8Unorm);
            builder.add_texture(handle.id(), image);

            let Ok(mut atlas) = builder.finish(images) else {
                #[cfg(any(debug_assertions, feature = "debug"))]
                error!("Failed to create atlas for texture: {}", kind.path());
                continue;
            };

            // Add texture coordinates, scaled to image size
            let mult = Vec2::new(width as f32 / 256., height as f32 / 256.);
            for mut rect in kind.coords() {
                rect.min *= mult;
                rect.max *= mult;

                atlas.add_texture(rect);
            }

            // Add the atlas to the list
            let handle = atlases.add(atlas);
            pack_atlases.insert(kind, handle);
        }

        pack_atlases
    }
}
