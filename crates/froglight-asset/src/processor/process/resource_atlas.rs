use std::any::TypeId;

use bevy_app::{App, Update};
use bevy_asset::{Assets, UntypedAssetId};
use bevy_ecs::{
    prelude::not,
    reflect::ReflectResource,
    schedule::IntoSystemConfigs,
    system::{Res, ResMut, Resource},
};
use bevy_log::{error, warn};
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_render::texture::Image;
use bevy_sprite::{TextureAtlasBuilder, TextureAtlasLayout};
use bevy_state::state::OnEnter;

use super::texture::TextureState;
use crate::{
    assets::{
        processed::{resource_atlas::ResourceAtlasStorage, ResourceAtlas},
        unprocessed::{atlas_definition::AtlasDefinitionEntry, ResourceAtlasDefinition},
    },
    AssetCatalog, AssetLoadState, ResourcePack, ResourcePackList,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<ResourceAtlasState>();

    // Reset the `ResourceAtlasState` when entering `AssetLoadState::Processing`
    app.add_systems(OnEnter(AssetLoadState::Processing), ResourceAtlasState::reset);

    // Catalog textures from the `ResourcePackList`
    app.add_systems(
        Update,
        ResourceAtlasState::create_resource_atlases
            .ambiguous_with_all()
            .run_if(not(ResourceAtlasState::is_finished))
            .run_if(TextureState::is_finished)
            .after(TextureState::catalog_textures)
            .in_set(AssetLoadState::Processing),
    );
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Resource)]
#[reflect(Default, Resource)]
pub(super) struct ResourceAtlasState {
    resource_index: usize,
    atlas_index: usize,
    finished: bool,
}

impl ResourceAtlasState {
    /// Returns `true` if the [`ResourceAtlasState`] has finished.
    pub(super) const fn finished(&self) -> bool { self.finished }

    /// Returns `true` if the [`ResourceAtlasState`] has finished.
    pub(super) fn is_finished(res: Res<Self>) -> bool { res.finished() }

    /// Catalogs textures from the [`ResourcePackList`].
    #[allow(clippy::too_many_arguments)]
    pub(super) fn create_resource_atlases(
        list: Res<ResourcePackList>,
        pack_assets: Res<Assets<ResourcePack>>,
        def_assets: Res<Assets<ResourceAtlasDefinition>>,

        mut state: ResMut<Self>,
        mut catalog: ResMut<AssetCatalog>,
        mut storage: ResMut<ResourceAtlasStorage>,

        mut atlas_assets: ResMut<Assets<ResourceAtlas>>,
        mut image_assets: ResMut<Assets<Image>>,
        mut layout_assets: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let handle = list.get(state.resource_index).expect("ResourceIndex out of bounds");
        let resource = pack_assets.get(handle).expect("ResourcePack not found");

        // Get the next `ResourceAtlasDefinition`
        if resource.atlas_definitions.is_empty() {
            // There are no `ResourceAtlasDefinitions` in the `ResourcePack`
        } else if let Some((key, handle)) = resource.atlas_definitions.iter().nth(state.atlas_index)
        {
            // If the `ResourceAtlas` does not exist, create it
            if catalog.get::<ResourceAtlas>(key).is_none() {
                // Create a `TextureAtlasBuilder`
                let mut builder = TextureAtlasBuilder::default();
                builder.max_size((4096, 4096).into());

                // Build the `ResourceAtlas`
                let atlas_def = def_assets.get(handle).expect("ResourceAtlasDefinition not found");
                let untyped_assets = catalog.inner.get(&TypeId::of::<Image>()).unwrap();
                for (index, entry) in atlas_def.sources.iter().enumerate() {
                    match entry {
                        // Add all images that start with `dir.source`
                        AtlasDefinitionEntry::Directory(dir) => {
                            for asset_id in untyped_assets.iter().filter_map(|(key, untyped_id)| {
                                if key.starts_with(dir.source.as_str()) {
                                    Some(untyped_id.typed_debug_checked())
                                } else {
                                    None
                                }
                            }) {
                                let image = image_assets.get(asset_id).expect("Image not found");
                                builder.add_texture(Some(asset_id), image);
                            }
                        }
                        // Add a single image
                        AtlasDefinitionEntry::Single(single) => {
                            let Some(asset_id) = untyped_assets
                                .get(&single.resource)
                                .copied()
                                .map(UntypedAssetId::typed_debug_checked)
                            else {
                                error!(
                                    "ResourceAtlas: Image not in catalog: \"{}\"",
                                    &single.resource
                                );
                                continue;
                            };

                            let Some(image) = image_assets.get(asset_id) else {
                                warn!(
                                    "ResourceAtlas: Image asset not found: \"{}\"",
                                    &single.resource
                                );
                                continue;
                            };

                            builder.add_texture(Some(asset_id), image);
                        }
                        // TODO: Support other `AtlasDefinitionEntry` types
                        _ => {
                            warn!("ResourceAtlas: Unsupported AtlasDefinitionEntry #{index} in \"{key}\"");
                        }
                    }
                }

                // Build the `ResourceAtlas`
                match builder.build() {
                    Err(err) => error!("ResourceAtlas: Failed to build, {err}"),
                    Ok((atlas_layout, atlas_image)) => {
                        let atlas_image = image_assets.add(atlas_image);
                        let atlas_layout = layout_assets.add(atlas_layout);
                        let atlas = atlas_assets.add(ResourceAtlas { atlas_image, atlas_layout });

                        // Add the `ResourceAtlas` to the `AssetCatalog`
                        catalog.insert(key.clone(), atlas.id());
                        // Add the `ResourceAtlas` to the `ResourceAtlasStorage`
                        storage.push(atlas);
                    }
                }
            }

            state.atlas_index += 1;
        } else {
            error!(
                "ResourceAtlas: Failed to get ResourceAtlasDefinition #{} in ResourcePack #{}",
                state.atlas_index, state.resource_index
            );
        }

        match (
            state.resource_index >= list.len().checked_sub(1).unwrap_or_default(),
            state.atlas_index
                >= resource.atlas_definitions.len().checked_sub(1).unwrap_or_default(),
        ) {
            (true, true) => {
                #[cfg(debug_assertions)]
                {
                    bevy_log::info!("AssetCatalog: Finished Cataloging ResourceAtlases");
                    bevy_log::debug!(
                        "AssetCatalog: {} ResourceAtlases",
                        catalog.len_of::<ResourceAtlas>()
                    );
                }

                state.finished = true;
            }
            (false, true) => {
                state.resource_index += 1;
                state.atlas_index = 0;
            }
            _ => {}
        }
    }

    /// Resets the [`ResourceAtlasState`].
    fn reset(mut res: ResMut<Self>) {
        res.resource_index = 0;
        res.atlas_index = 0;
        res.finished = false;
    }
}
