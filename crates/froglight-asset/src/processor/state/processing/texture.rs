use bevy_app::{App, Update};
use bevy_asset::Assets;
use bevy_ecs::{
    prelude::not,
    reflect::ReflectResource,
    schedule::IntoSystemConfigs,
    system::{Res, ResMut, Resource},
};
use bevy_log::error;
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_render::texture::{Image, ImageSampler};
use bevy_state::state::OnEnter;

use crate::{AssetCatalog, AssetProcess, ResourcePack, ResourcePackList};

#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    app.register_type::<TextureProcessor>();
    app.init_resource::<TextureProcessor>();

    // Reset the `TextureProcessor` state
    app.add_systems(OnEnter(AssetProcess::Processing), TextureProcessor::reset_state);

    // Catalog textures
    app.add_systems(
        Update,
        TextureProcessor::catalog_textures
            .run_if(not(TextureProcessor::is_finished))
            .ambiguous_with_all()
            .in_set(AssetProcess::Processing),
    );
}

/// A processor that catalogs textures in the [`AssetCatalog`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct TextureProcessor {
    resource_index: usize,
    texture_index: usize,
    finished: bool,
}

impl TextureProcessor {
    /// A [`Condition`](bevy_ecs::schedule::Condition) that checks if the
    /// [`TextureProcessor`] is finished.
    #[must_use]
    pub fn is_finished(res: Res<Self>) -> bool { res.finished }

    /// A [`System`](bevy_ecs::system::System) that adds textures to the
    /// [`AssetCatalog`] in batches.
    ///
    /// [`ResourcePack`]s are processed in the same order as they are in the
    /// [`ResourcePackList`].
    pub fn catalog_textures(
        resources: Res<ResourcePackList>,
        mut assets: ResMut<Assets<ResourcePack>>,
        mut images: ResMut<Assets<Image>>,
        mut catalog: ResMut<AssetCatalog>,
        mut state: ResMut<Self>,
    ) {
        let _ = Self::catalog_texture_batch(
            &resources,
            &mut assets,
            &mut images,
            &mut catalog,
            &mut state,
        );

        // Check if the processor is finished.
        if state.resource_index >= resources.len() {
            #[cfg(debug_assertions)]
            {
                bevy_log::info!("TextureProcessor: Finished");
                bevy_log::debug!(
                    "TextureProcessor: Cataloged {} Textures",
                    catalog.len_of::<Image>()
                );
            }
            // Set the processor to finished.
            *state = Self { finished: true, ..Self::default() };
        }
    }

    /// The number of textures to process per frame.
    const TEXTURES_PER_FRAME: usize = 50;

    /// Catalogs a batch of textures.
    ///
    /// Also sets used textures to use nearest sampling.
    fn catalog_texture_batch(
        resources: &ResourcePackList,
        assets: &mut Assets<ResourcePack>,
        images: &mut Assets<Image>,
        catalog: &mut AssetCatalog,
        state: &mut TextureProcessor,
    ) -> Result<(), ()> {
        // Get the current ResourcePack.
        let handle = resources.get(state.resource_index).ok_or(())?;
        let asset = assets.get_mut(handle).ok_or_else(|| {
            error!("TextureProcessor: ResourcePack Asset missing!");
            state.resource_index += 1;
        })?;

        // Iterate over the next `TEXTURES_PER_FRAME` textures.
        let mut typed_catalog = catalog.typed_mut::<Image>();
        for (texture_key, texture_handle) in
            asset.textures.iter_mut().skip(state.texture_index).take(Self::TEXTURES_PER_FRAME)
        {
            // Replace the existing strong handle with a weak handle.
            let texture_handle = std::mem::replace(texture_handle, texture_handle.clone_weak());

            // Add the taken strong handle to the catalog, if it doesn't already exist.
            typed_catalog.entry(texture_key.to_owned()).or_insert_with(|| {
                // Set the texture sampler to nearest.
                if let Some(texture) = images.get_mut(&texture_handle) {
                    texture.sampler = ImageSampler::nearest();
                }
                // Insert the strong handle into the catalog.
                texture_handle.untyped()
            });

            // Increment the texture index.
            state.texture_index += 1;
        }

        // If the texture index is at the end of the textures,
        // increment the resource index.
        if state.texture_index >= asset.textures.len() {
            state.resource_index += 1;
            state.texture_index = 0;
        }

        Ok(())
    }

    /// Resets the state of the [`TextureProcessor`].
    fn reset_state(mut res: ResMut<Self>) {
        #[cfg(debug_assertions)]
        bevy_log::info!("TextureProcessor: Resetting state");
        *res = Self::default();
    }
}
