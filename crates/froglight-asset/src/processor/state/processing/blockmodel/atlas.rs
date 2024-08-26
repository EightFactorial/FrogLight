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
        processed::BlockAtlas,
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
        mut images: ResMut<Assets<Image>>,
        mut atlases: ResMut<Assets<TextureAtlasLayout>>,
        mut state: ResMut<Self>,
        mut commands: Commands,
    ) {
        let mut builder = TextureAtlasBuilder::default();
        builder.initial_size((512, 512).into()).max_size((4096, 4096).into());

        let mut inserted_textures = HashSet::new();

        // Iterate over all `BlockModelDefinition`s
        for def in catalog
            .typed_ref::<BlockModelDefinition>()
            .unwrap()
            .iter_untyped()
            .filter_map(|(_, h)| definitions.get(h.id().typed_debug_checked()))
        {
            // If the `BlockModelDefinition` has textures
            if let Some(textures) = &def.textures {
                // Get all texture references
                for texture in textures.values().filter_map(|v| {
                    if let ResourceOrVariable::Resource(key) = v {
                        ResourceKey::try_new(key).ok()
                    } else {
                        None
                    }
                }) {
                    if let Some(image_handle) = catalog.get::<Image>(&texture) {
                        if inserted_textures.insert(image_handle.id()) {
                            if let Some(image) = images.get(&image_handle) {
                                builder.add_texture(Some(image_handle.id()), image);
                            }
                        }
                    } else {
                        error!("BlockModelProcessor: Catalog missing texture \"{texture}\"");
                    }
                }
            }
        }

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
}
