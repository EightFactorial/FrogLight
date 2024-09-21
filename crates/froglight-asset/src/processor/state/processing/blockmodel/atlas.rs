use bevy_app::{App, Update};
use bevy_asset::Assets;
use bevy_ecs::{
    prelude::not,
    schedule::IntoSystemConfigs,
    system::{Commands, Res, ResMut},
};
use bevy_log::{debug, error};
use bevy_render::texture::{Image, ImageSampler};
use bevy_sprite::{TextureAtlasBuilder, TextureAtlasLayout};
use bevy_utils::HashSet;
use froglight_common::ResourceKey;

use super::BlockModelProcessor;
use crate::{
    assets::{
        processed::{BlockAtlas, FallbackTexture},
        raw::{model::ResourceOrVariable, BlockModelDefinition},
    },
    processor::state::TextureProcessor,
    AssetCatalog, AssetProcess,
};

#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    app.add_systems(
        Update,
        BlockModelProcessor::create_block_atlas
            .after(TextureProcessor::catalog_textures)
            .after(BlockModelProcessor::catalog_blockmodel_definitions)
            .run_if(TextureProcessor::is_finished)
            .run_if(BlockModelProcessor::is_model_finished)
            .run_if(not(BlockModelProcessor::is_atlas_finished))
            .ambiguous_with_all()
            .in_set(AssetProcess::Processing),
    );
}

impl BlockModelProcessor {
    /// A [`Condition`](bevy_ecs::schedule::Condition) that checks if the
    /// [`BlockModelProcessor`] is has created the [`BlockAtlas`].
    #[must_use]
    pub(super) fn is_atlas_finished(res: Res<Self>) -> bool { res.atlas_finished }

    /// Creates a [`BlockAtlas`] from all textures referenced in
    /// [`BlockModelDefinition`]s.
    pub(super) fn create_block_atlas(
        definitions: Res<Assets<BlockModelDefinition>>,
        catalog: Res<AssetCatalog>,
        fallback: Res<FallbackTexture>,
        mut images: ResMut<Assets<Image>>,
        mut atlases: ResMut<Assets<TextureAtlasLayout>>,
        mut state: ResMut<Self>,
        mut commands: Commands,
    ) {
        let mut builder = TextureAtlasBuilder::default();
        builder.initial_size((512, 512).into()).max_size((4096, 4096).into());

        #[cfg(debug_assertions)]
        let mut definition_count = 0;
        let mut inserted_textures = HashSet::new();

        // Insert the `FallbackTexture`
        {
            let fallback_handle = fallback.as_handle();
            let fallback_image =
                images.get(fallback_handle).expect("AssetServer failed to load FallbackTexture");
            builder.add_texture(Some(fallback_handle.id()), fallback_image);
            inserted_textures.insert(fallback_handle.id().untyped());
        }

        // Iterate over all `BlockModelDefinition`s
        if let Some(defs) = catalog.typed_ref::<BlockModelDefinition>() {
            for def in defs.iter_untyped().filter_map(
                |(k, h)| {
                    definitions.get(h.id().typed_debug_checked()).or_else(|| {
                        error!("BlockModelProcessor: Failed to get BlockModelDefinition from AssetServer, \"{k}\"");
                        None
                    })
                },
            ) {
                // Get the resources (textures) from the current definition and all parents
                for resource in
                    Self::get_resources(def, &catalog, &definitions).into_iter().filter_map(|r| {
                        ResourceKey::try_new(r)
                            .inspect_err(|err| {
                                error!(
                                    "BlockModelProcessor: Failed to create ResourceKey from \"{r}\", {err}"
                                );
                            })
                            .ok()
                    })
                {
                    // Get the texture handle from the catalog
                    if let Some(image_handle) = catalog.get_untyped::<Image>(&resource) {
                        // If the texture has not been inserted before
                        if inserted_textures.insert(image_handle.id()) {
                            // Add the texture to the `TextureAtlasBuilder`
                            let image_id = image_handle.id().typed_debug_checked();
                            if let Some(image) = images.get(image_id) {
                                builder.add_texture(Some(image_id), image);
                            } else {
                                error!(
                                    "BlockModelProcessor: Failed to get Image from AssetServer, \"{resource}\""
                                );
                            }
                        }
                    } else {
                        error!(
                            "BlockModelProcessor: Failed to get Image from AssetCatalog, \"{resource}\""
                        );
                    }
                }
    
                #[cfg(debug_assertions)]
                {
                    definition_count += 1;
                }
            }
        }


        #[cfg(debug_assertions)]
        debug!(
            "BlockModelProcessor: Creating BlockAtlas from {definition_count} model definitions and {} textures",
            inserted_textures.len()
        );

        // Build the `BlockAtlas`
        match builder.build() {
            Ok((atlas, mut image)) => {
                image.sampler = ImageSampler::nearest();
                let image_handle = images.add(image);

                let atlas_handle = atlases.add(atlas.clone());
                commands.insert_resource(BlockAtlas::new(atlas, atlas_handle, image_handle));
            }
            Err(err) => {
                error!("BlockModelProcessor: Failed to build BlockAtlas, {err}");
            }
        }

        debug!("BlockModelProcessor: Created BlockAtlas");
        state.atlas_finished = true;
    }

    fn get_resources<'a>(
        def: &'a BlockModelDefinition,
        catalog: &AssetCatalog,
        definitions: &'a Assets<BlockModelDefinition>,
    ) -> Vec<&'a str> {
        // If a parent exists, collect resources from the parent
        let mut resources = if let Some(parent) = def
            .parent
            .as_ref()
            .and_then(|p| ResourceKey::try_new(p).ok())
            .and_then(|k| catalog.get_untyped::<BlockModelDefinition>(&k))
            .and_then(|h| definitions.get(h.id().typed_debug_checked()))
        {
            Self::get_resources(parent, catalog, definitions)
        } else {
            Vec::new()
        };

        // Get the resources from the current definition
        if let Some(textures) = &def.textures {
            for value in textures.values() {
                if let ResourceOrVariable::Resource(key) = value {
                    resources.push(key);
                }
            }
        }

        resources
    }
}
