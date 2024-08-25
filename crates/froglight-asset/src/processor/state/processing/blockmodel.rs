use bevy_app::{App, Update};
use bevy_asset::{Assets, Handle};
use bevy_ecs::{
    prelude::not,
    reflect::ReflectResource,
    schedule::IntoSystemConfigs,
    system::{Commands, Res, ResMut, Resource},
};
use bevy_log::{debug, error};
use bevy_math::prelude::Cuboid;
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_render::{
    mesh::{Indices, Mesh, PrimitiveTopology, VertexAttributeValues},
    render_asset::RenderAssetUsages,
    texture::{Image, ImageSampler},
};
use bevy_sprite::{TextureAtlasBuilder, TextureAtlasLayout};
use bevy_state::state::OnEnter;
use bevy_transform::components::Transform;
use bevy_utils::{HashMap, HashSet};
use froglight_common::{Direction, ResourceKey};
use glam::{FloatExt, Vec3};

use super::{BlockStateProcessor, TextureProcessor};
use crate::{
    assets::{
        processed::{
            model::{BlockModel, BlockModelCache, ModelTransformIndex},
            BlockAtlas,
        },
        raw::{
            blockstate::StateModelDefinitions,
            model::{DefinitionElement, DefinitionTransform, ElementFace, ResourceOrVariable},
            BlockModelDefinition, BlockStateDefinition,
        },
    },
    AssetCatalog, AssetProcess, ResourcePack, ResourcePackList,
};

#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    app.register_type::<BlockModelProcessor>();
    app.init_resource::<BlockModelProcessor>();

    // Reset the `BlockModelProcessor` state
    app.add_systems(OnEnter(AssetProcess::Processing), BlockModelProcessor::reset_blockmodel_state);
    // Clear the `AssetCatalog` blockmodels
    app.add_systems(OnEnter(AssetProcess::Processing), BlockModelProcessor::clear_catalog_models);

    // Catalog BlockModelDefinitions
    app.add_systems(
        Update,
        BlockModelProcessor::catalog_blockmodel_definitions
            .run_if(not(BlockModelProcessor::is_model_finished))
            .ambiguous_with_all()
            .in_set(AssetProcess::Processing),
    );

    // Create BlockAtlas
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

    // Create BlockModels
    app.add_systems(
        Update,
        BlockModelProcessor::create_blockmodels
            .after(BlockStateProcessor::catalog_blockstates)
            .after(BlockModelProcessor::create_block_atlas)
            .run_if(BlockStateProcessor::is_finished)
            .run_if(BlockModelProcessor::is_model_finished)
            .run_if(BlockModelProcessor::is_atlas_finished)
            .run_if(not(BlockModelProcessor::is_finished))
            .ambiguous_with_all()
            .in_set(AssetProcess::Processing),
    );
}

/// A processor that creates [`BlockModel`] for [`BlockStateDefinition`]s.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct BlockModelProcessor {
    resource_index: usize,
    model_index: usize,
    model_finished: bool,

    atlas_finished: bool,

    state_index: usize,
    finished: bool,
}

impl BlockModelProcessor {
    /// A [`Condition`](bevy_ecs::schedule::Condition) that checks if the
    /// [`BlockModelProcessor`] is finished.
    #[must_use]
    pub fn is_finished(res: Res<Self>) -> bool { res.finished }

    /// A [`Condition`](bevy_ecs::schedule::Condition) that checks if the
    /// [`BlockModelProcessor`] is finished cataloging models.
    #[must_use]
    fn is_model_finished(res: Res<Self>) -> bool { res.model_finished }

    /// A [`Condition`](bevy_ecs::schedule::Condition) that checks if the
    /// [`BlockModelProcessor`] is has created the [`BlockAtlas`].
    #[must_use]
    fn is_atlas_finished(res: Res<Self>) -> bool { res.atlas_finished }

    /// A [`System`](bevy_ecs::system::System) that adds block model definitions
    /// to the [`AssetCatalog`] in batches.
    ///
    /// [`ResourcePack`]s are processed in the same order as they are in the
    /// [`ResourcePackList`].
    fn catalog_blockmodel_definitions(
        resources: Res<ResourcePackList>,
        mut assets: ResMut<Assets<ResourcePack>>,
        mut catalog: ResMut<AssetCatalog>,
        mut state: ResMut<Self>,
    ) {
        let _ = Self::catalog_model_definitions_batch(
            &resources,
            &mut assets,
            &mut catalog,
            &mut state,
        );

        // Check if we've finished cataloging all model definitions
        if state.resource_index >= resources.len() {
            #[cfg(debug_assertions)]
            bevy_log::info!("BlockModelProcessor: Finished Cataloging");
            debug!(
                "BlockModelProcessor: Cataloged {} BlockModel Definitions",
                catalog.len_of::<BlockModelDefinition>()
            );
            state.model_finished = true;
        }
    }

    /// Creates a [`BlockAtlas`] from all textures referenced in
    /// [`BlockModelDefinition`]s.
    fn create_block_atlas(
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

    /// The number of models to catalog per frame.
    const CATALOGED_MODELS_PER_FRAME: usize = 50;

    fn catalog_model_definitions_batch(
        resources: &ResourcePackList,
        assets: &mut Assets<ResourcePack>,
        catalog: &mut AssetCatalog,
        state: &mut Self,
    ) -> Result<(), ()> {
        // Get the current ResourcePack.
        let handle = resources.get(state.resource_index).ok_or(())?;
        let asset = assets.get_mut(handle).ok_or_else(|| {
            error!("BlockModelProcessor: ResourcePack Asset missing!");
            state.resource_index += 1;
        })?;

        // Iterate over the next `CATALOGED_MODELS_PER_FRAME` sounds.
        let mut typed_catalog = catalog.typed_mut::<BlockModelDefinition>();
        for (model_key, model_handle) in asset
            .block_models
            .iter_mut()
            .skip(state.model_index)
            .take(Self::CATALOGED_MODELS_PER_FRAME)
        {
            // Replace the existing strong handle with a weak handle.
            let model_handle = std::mem::replace(model_handle, model_handle.clone_weak());

            // Add the taken strong handle to the catalog, if it doesn't already exist.
            typed_catalog.entry(model_key.to_owned()).or_insert(model_handle.untyped());

            // Increment the sound index.
            state.model_index += 1;
        }

        // If the model def index is at the end of the model list,
        // increment the resource index.
        if state.model_index >= asset.block_models.len() {
            state.resource_index += 1;
            state.model_index = 0;
        }

        Ok(())
    }

    /// The number of models to create per frame.
    const CREATED_MODELS_PER_FRAME: usize = 20;

    /// A [`System`](bevy_ecs::system::System) that builds [`BlockModel`]s from
    /// [`BlockStateDefinition`]s.
    #[allow(clippy::missing_panics_doc)]
    #[allow(clippy::too_many_arguments)]
    pub fn create_blockmodels(
        states: Res<Assets<BlockStateDefinition>>,
        definitions: Res<Assets<BlockModelDefinition>>,
        cache: ResMut<BlockModelCache>,
        atlas: Res<BlockAtlas>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut models: ResMut<Assets<BlockModel>>,
        mut catalog: ResMut<AssetCatalog>,
        mut state: ResMut<Self>,
    ) {
        catalog.typed_mut_scope::<BlockModel>(|catalog, mut catalog_models| {
            for (_state_key, state_handle) in catalog
                .typed_ref::<BlockStateDefinition>()
                .unwrap()
                .iter_untyped()
                .skip(state.state_index)
                .take(Self::CREATED_MODELS_PER_FRAME)
            {
                state.state_index += 1;
                if let Some(state) = states.get(state_handle.id().typed_debug_checked()) {
                    match state {
                        BlockStateDefinition::Variants { variants } => {
                            for def in variants.values().flat_map(StateModelDefinitions::as_slice) {
                                let Some(definition_key) = ResourceKey::try_new(def.model.clone()).ok() else { continue; };
                                let Some(definition_handle) = catalog.get_untyped::<BlockModelDefinition>(&definition_key) else { continue; };
                                let Some(definition) = definitions.get(definition_handle.id().typed_debug_checked()) else { continue; };

                                if !catalog_models.contains(&definition_key) {
                                    if let Some(model) = Self::create_model(
                                        definition,
                                        &definition_key,
                                        &definitions,
                                        &cache,
                                        &atlas,
                                        catalog,
                                        &mut meshes,
                                     ) {
                                        let model_handle = models.add(model);
                                        catalog_models.insert(definition_key, model_handle);
                                    } else {
                                        #[cfg(debug_assertions)]
                                        bevy_log::warn!(
                                            "BlockModelProcessor: Failed to create BlockModel \"{definition_key}\""
                                        );
                                    }
                                }
                            }
                        }
                        BlockStateDefinition::MultiPart { multipart } => {
                            for def in multipart.iter().flat_map(|p| p.apply.as_slice()) {
                                let Some(definition_key) = ResourceKey::try_new(def.model.clone()).ok() else { continue; };
                                let Some(definition_handle) = catalog.get_untyped::<BlockModelDefinition>(&definition_key) else { continue; };
                                let Some(definition) = definitions.get(definition_handle.id().typed_debug_checked()) else { continue; };

                                if !catalog_models.contains(&definition_key) {
                                    if let Some(model) = Self::create_model(
                                        definition,
                                        &definition_key,
                                        &definitions,
                                        &cache,
                                        &atlas,
                                        catalog,
                                        &mut meshes,
                                    ) {
                                        let model_handle = models.add(model);
                                        catalog_models.insert(definition_key, model_handle);
                                    } else {
                                        #[cfg(debug_assertions)]
                                        bevy_log::warn!(
                                            "BlockModelProcessor: Failed to create BlockModel \"{definition_key}\""
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        // Check if we've finished processing all blockstates
        if state.state_index >= catalog.len_of::<BlockStateDefinition>() {
            #[cfg(debug_assertions)]
            bevy_log::info!("BlockModelProcessor: Finished");
            debug!("BlockModelProcessor: Created {} BlockModels", catalog.len_of::<BlockModel>());

            *state = Self {
                model_finished: true,
                atlas_finished: true,
                finished: true,
                ..Self::default()
            };
        }
    }

    /// Resets the state of the [`BlockModelProcessor`].
    fn reset_blockmodel_state(mut res: ResMut<Self>) {
        #[cfg(debug_assertions)]
        bevy_log::trace!("BlockModelProcessor: Resetting state");
        *res = Self::default();
    }

    /// Clears the [`AssetCatalog`] of all [`BlockModel`]s.
    fn clear_catalog_models(mut catalog: ResMut<AssetCatalog>) {
        #[cfg(debug_assertions)]
        bevy_log::info!("BlockModelProcessor: Clearing AssetCatalog BlockModels");
        catalog.clear_of::<BlockModelDefinition>();
        catalog.clear_of::<BlockModel>();
    }
}

impl BlockModelProcessor {
    // Suppose Y-up right hand, and camera look from +Z to -Z
    const CUBOID_FACES: [Direction; 6] = [
        // Front
        Direction::South,
        // Back
        Direction::North,
        // Right
        Direction::East,
        // Left
        Direction::West,
        // Top
        Direction::Up,
        // Bottom
        Direction::Down,
    ];

    fn create_model(
        definition: &BlockModelDefinition,
        definition_key: &ResourceKey,
        definitions: &Assets<BlockModelDefinition>,
        cache: &BlockModelCache,
        atlas: &BlockAtlas,
        catalog: &AssetCatalog,
        meshes: &mut Assets<Mesh>,
    ) -> Option<BlockModel> {
        // Get the elements for the model
        let elements = Self::recurse_elements(definition, catalog, definitions)?;

        #[cfg(debug_assertions)]
        if elements.is_empty() {
            bevy_log::warn!("BlockModelProcessor: BlockModel \"{definition_key}\" has no elements");
        }

        // Get the ambient occlusion value
        let _ambient_occlusion = Self::recurse_ambient_occlusion(definition, catalog, definitions)
            .unwrap_or(BlockModelDefinition::DEFAULT_AMBIENT_OCCLUSION);

        // Get the display transforms for each display type
        let mut transforms = [Transform::default(); 8];
        for display_type in ModelTransformIndex::iter() {
            if let Some(display_transform) =
                Self::recurse_display_type(display_type, definition, catalog, definitions)
            {
                transforms[usize::from(display_type)] = display_transform.into();
            }
        }

        // Initialize the cache array
        let mut cache = cache.write();
        let cache = cache.entry(definition_key.clone()).or_insert([
            Self::default_mesh(),
            Self::default_mesh(),
            Self::default_mesh(),
            Self::default_mesh(),
            Self::default_mesh(),
            Self::default_mesh(),
        ]);

        // Iterate over the elements and create meshes
        let mut block_mesh = Self::default_mesh();
        for element in elements {
            let (from, to): (Vec3, Vec3) = (element.from.into(), element.to.into());

            let mut element_mesh = Mesh::from(Cuboid::from_corners(from, to));
            element_mesh.translate_by(from.midpoint(to) - Vec3::splat(8.0));
            element_mesh.scale_by(Vec3::splat(1.0 / 16.0));

            // TODO: Rotate the element mesh based on the element rotation

            // Append per-face data to the directional meshes
            for direction in Self::CUBOID_FACES {
                let Some(element_face) = element.faces.get(&direction) else {
                    continue;
                };

                let direction_mesh = &mut cache[usize::from(direction)];
                let attribute_group = usize::from(direction);

                // TODO: Set the element mesh UVs based on the element texture
                // TODO: Apply the element face rotation to the UVs
                if let Some(texture_handle) = Self::get_texture(
                    element_face,
                    definition,
                    definition_key,
                    catalog,
                    definitions,
                ) {
                    if let Some(atlas_index) = atlas.layout().get_texture_index(texture_handle.id())
                    {
                        let atlas_rect = atlas.layout().textures[atlas_index].as_rect();
                        let atlas_size = atlas.layout().size.as_vec2();

                        let element_uvs = element_mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap();
                        let VertexAttributeValues::Float32x2(element_uvs) = element_uvs else {
                            unreachable!();
                        };

                        let _face_uvs = element_face.uv(element);

                        let uv_range = (attribute_group * 4)..(attribute_group * 4 + 4);
                        for [u, v] in &mut element_uvs[uv_range] {
                            *u = u.remap(
                                0.0,
                                1.0,
                                atlas_rect.min.x / atlas_size.x,
                                atlas_rect.max.x / atlas_size.x,
                            );
                            *v = v.remap(
                                0.0,
                                1.0,
                                atlas_rect.min.y / atlas_size.y,
                                atlas_rect.max.y / atlas_size.y,
                            );
                        }
                    } else {
                        // bevy_log::error!("BlockModelProcessor: {direction:?} texture for \"{definition_key}\" not in BlockAtlas");
                    }
                } else {
                    // bevy_log::error!("BlockModelProcessor: Catalog missing
                    // {direction:?} texture for \"{definition_key}\"");
                }

                // Append the element positions to the direction mesh
                Self::append_element_positions(attribute_group, direction_mesh, &element_mesh);
                // Append the element normals to the direction mesh
                Self::append_element_normals(attribute_group, direction_mesh, &element_mesh);
                // Append the element uvs to the direction mesh
                Self::append_element_uvs(attribute_group, direction_mesh, &element_mesh);
            }

            block_mesh.merge(&element_mesh);
        }

        Some(BlockModel { block_mesh: meshes.add(block_mesh), transforms })
    }

    fn default_mesh() -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
        mesh.insert_indices(Indices::U32(Vec::new()));
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(Vec::new()),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::Float32x3(Vec::new()));
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float32x2(Vec::new()));
        mesh
    }

    fn append_element_positions(
        attribute_group: usize,
        direction_mesh: &mut Mesh,
        element_mesh: &Mesh,
    ) {
        let position_range = (attribute_group * 4)..(attribute_group * 4 + 4);
        match (
            element_mesh.attribute(Mesh::ATTRIBUTE_POSITION),
            direction_mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION),
        ) {
            (
                Some(VertexAttributeValues::Float32x3(elem_positions)),
                Some(VertexAttributeValues::Float32x3(dir_positions)),
            ) => {
                dir_positions.extend_from_slice(&elem_positions[position_range]);
            }
            (Some(VertexAttributeValues::Float32x3(elem_positions)), None) => {
                direction_mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    elem_positions[position_range].to_vec(),
                );
            }
            _ => unreachable!("Element will always have Float32x3 positions"),
        }
    }

    fn append_element_normals(
        attribute_group: usize,
        direction_mesh: &mut Mesh,
        element_mesh: &Mesh,
    ) {
        let normal_range = (attribute_group * 4)..(attribute_group * 4 + 4);
        match (
            element_mesh.attribute(Mesh::ATTRIBUTE_NORMAL),
            direction_mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL),
        ) {
            (
                Some(VertexAttributeValues::Float32x3(elem_normals)),
                Some(VertexAttributeValues::Float32x3(dir_normals)),
            ) => {
                dir_normals.extend_from_slice(&elem_normals[normal_range]);
            }
            (Some(VertexAttributeValues::Float32x3(elem_normals)), None) => {
                direction_mesh
                    .insert_attribute(Mesh::ATTRIBUTE_NORMAL, elem_normals[normal_range].to_vec());
            }
            _ => unreachable!("Element will always have Float32x3 normals"),
        }
    }

    fn append_element_uvs(attribute_group: usize, direction_mesh: &mut Mesh, element_mesh: &Mesh) {
        let uv_range = (attribute_group * 4)..(attribute_group * 4 + 4);
        match (
            element_mesh.attribute(Mesh::ATTRIBUTE_UV_0),
            direction_mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0),
        ) {
            (
                Some(VertexAttributeValues::Float32x2(elem_uvs)),
                Some(VertexAttributeValues::Float32x2(dir_uvs)),
            ) => {
                dir_uvs.extend_from_slice(&elem_uvs[uv_range]);
            }
            (Some(VertexAttributeValues::Float32x2(elem_uvs)), None) => {
                direction_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, elem_uvs[uv_range].to_vec());
            }
            _ => unreachable!("Element will always have Float32x2 uvs"),
        }
    }

    fn get_texture(
        element: &ElementFace,
        definition: &BlockModelDefinition,
        definition_key: &ResourceKey,
        catalog: &AssetCatalog,
        definitions: &Assets<BlockModelDefinition>,
    ) -> Option<Handle<Image>> {
        match &element.texture {
            ResourceOrVariable::Resource(resource) => {
                catalog.get::<Image>(&ResourceKey::try_new(resource).ok()?)
            }
            ResourceOrVariable::Variable(variable) => Self::recurse_for_resource(
                variable.clone(),
                definition,
                definition_key,
                definitions,
                catalog,
                &mut HashMap::new(),
            ),
        }
    }

    /// Get's the [`AssetId`] for a resource.
    fn recurse_for_resource<'a>(
        mut variable: String,
        definition: &'a BlockModelDefinition,
        definition_key: &ResourceKey,
        definitions: &'a Assets<BlockModelDefinition>,
        catalog: &'a AssetCatalog,
        variables: &mut HashMap<String, ResourceOrVariable>,
    ) -> Option<Handle<Image>> {
        // Append the current definition's variables to the variable map
        if let Some(textures) = definition.textures.as_ref() {
            variables.extend(textures.clone());
        }

        // Resolve the variable to a resource, or as far as possible
        if let Some(mut res_variable) = variables.get(&variable) {
            while let ResourceOrVariable::Variable(var) = res_variable {
                if let Some(value) = variables.get(var) {
                    if res_variable == value {
                        break;
                    }
                    res_variable = value;
                } else {
                    variable = var.to_string();
                    break;
                }
            }
            if let ResourceOrVariable::Resource(key) = res_variable {
                let key = ResourceKey::try_new(key).ok()?;
                if let Some(asset_id) = catalog.get::<Image>(&key) {
                    return Some(asset_id);
                }
                variable = key.to_string();
            }
        }

        if let Some(parent) = definition.parent.as_ref() {
            // Look for the variable in the parent definition
            let parent_key = ResourceKey::try_new(parent).ok()?;
            let parent_handle = catalog.get_untyped::<BlockModelDefinition>(&parent_key)?;
            let parent_def = definitions.get(parent_handle.id().typed_debug_checked())?;
            Self::recurse_for_resource(
                variable,
                parent_def,
                &parent_key,
                definitions,
                catalog,
                variables,
            )
        } else {
            #[cfg(debug_assertions)]
            bevy_log::warn!(
                "BlockModelProcessor: Failed to resolve variable \"{variable}\" in BlockModel \"{definition_key}\"",
            );
            None
        }
    }

    fn recurse_ambient_occlusion(
        definition: &BlockModelDefinition,
        catalog: &AssetCatalog,
        definitions: &Assets<BlockModelDefinition>,
    ) -> Option<bool> {
        if let Some(ambient_occlusion) = definition.ambient_occlusion {
            return Some(ambient_occlusion);
        }

        if let Some(parent) = definition.parent.as_ref() {
            let parent_key = ResourceKey::try_new(parent.to_owned()).ok()?;
            let parent_handle = catalog.get_untyped::<BlockModelDefinition>(&parent_key)?;
            let parent = definitions.get(parent_handle.id().typed_debug_checked())?;
            Self::recurse_ambient_occlusion(parent, catalog, definitions)
        } else {
            None
        }
    }

    fn recurse_display_type<'a>(
        display: ModelTransformIndex,
        definition: &'a BlockModelDefinition,
        catalog: &AssetCatalog,
        definitions: &'a Assets<BlockModelDefinition>,
    ) -> Option<&'a DefinitionTransform> {
        if let Some(displays) = definition.display.as_ref() {
            if let Some(transform) = displays.get(&display) {
                return Some(transform);
            }
        }

        if let Some(parent) = definition.parent.as_ref() {
            let parent_key = ResourceKey::try_new(parent.to_owned()).ok()?;
            let parent_handle = catalog.get_untyped::<BlockModelDefinition>(&parent_key)?;
            let parent = definitions.get(parent_handle.id().typed_debug_checked())?;
            Self::recurse_display_type(display, parent, catalog, definitions)
        } else {
            None
        }
    }

    fn recurse_elements<'a>(
        definition: &'a BlockModelDefinition,
        catalog: &AssetCatalog,
        definitions: &'a Assets<BlockModelDefinition>,
    ) -> Option<&'a [DefinitionElement]> {
        if let Some(elements) = definition.elements.as_ref() {
            return Some(elements);
        }

        if let Some(parent) = definition.parent.as_ref() {
            let parent_key = ResourceKey::try_new(parent.to_owned()).ok()?;
            let parent_handle = catalog.get_untyped::<BlockModelDefinition>(&parent_key)?;
            let parent = definitions.get(parent_handle.id().typed_debug_checked())?;
            Self::recurse_elements(parent, catalog, definitions)
        } else {
            None
        }
    }
}
