use bevy_app::{App, Update};
use bevy_asset::Assets;
use bevy_ecs::{
    prelude::not,
    reflect::ReflectResource,
    schedule::IntoSystemConfigs,
    system::{Res, ResMut, Resource},
};
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_render::texture::{Image, ImageSampler};
use bevy_state::state::OnEnter;

use crate::{AssetCatalog, AssetLoadState, ResourcePack, ResourcePackList};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<TextureState>();

    // Reset the `TextureState` when entering `AssetLoadState::Processing`
    app.add_systems(OnEnter(AssetLoadState::Processing), TextureState::reset);

    // Catalog textures from the `ResourcePackList`
    app.add_systems(
        Update,
        TextureState::catalog_textures
            .ambiguous_with_all()
            .run_if(not(TextureState::is_finished))
            .in_set(AssetLoadState::Processing),
    );
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Resource)]
#[reflect(Default, Resource)]
pub(super) struct TextureState {
    resource_index: usize,
    texture_index: usize,
    finished: bool,
}

impl TextureState {
    /// The number of textures to add to the [`AssetCatalog`] per frame.
    const TEXTURES_PER_FRAME: usize = 8;

    /// Returns `true` if the [`TextureState`] has finished.
    pub(super) const fn finished(&self) -> bool { self.finished }

    /// Returns `true` if the [`TextureState`] has finished.
    pub(super) fn is_finished(res: Res<Self>) -> bool { res.finished() }

    /// Catalogs textures from the [`ResourcePackList`].
    pub(super) fn catalog_textures(
        list: Res<ResourcePackList>,
        assets: Res<Assets<ResourcePack>>,

        mut state: ResMut<Self>,
        mut images: ResMut<Assets<Image>>,
        mut catalog: ResMut<AssetCatalog>,
    ) {
        let handle = list.get(state.resource_index).expect("ResourceIndex out of bounds");
        let resource = assets.get(handle).expect("ResourcePack not found");

        for (key, handle) in
            resource.textures.iter().skip(state.texture_index).take(Self::TEXTURES_PER_FRAME)
        {
            // Set the image sampler to nearest
            let image = images.get_mut(handle).expect("Image not found");
            image.sampler = ImageSampler::nearest();

            // Add the texture to the catalog
            catalog.entry::<Image>(key.clone()).or_insert(handle.id().untyped());
            state.texture_index += 1;
        }

        match (
            state.resource_index >= list.len().checked_sub(1).unwrap_or_default(),
            state.texture_index >= resource.textures.len().checked_sub(1).unwrap_or_default(),
        ) {
            (true, true) => {
                #[cfg(debug_assertions)]
                {
                    bevy_log::info!("AssetCatalog: Finished Cataloging Textures");
                    bevy_log::debug!("AssetCatalog: {} Textures", catalog.len_of::<Image>());
                }

                state.finished = true;
            }
            (false, true) => {
                state.resource_index += 1;
                state.texture_index = 0;
            }
            _ => {}
        }
    }

    /// Resets the [`TextureState`].
    fn reset(mut res: ResMut<Self>) {
        res.resource_index = 0;
        res.texture_index = 0;
        res.finished = false;
    }
}
