use std::{collections::BTreeMap, sync::Arc};

use bevy_app::App;
use bevy_asset::{Asset, AssetApp, AssetId, Assets, Handle, ReflectAsset, ReflectHandle};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    reflect::ReflectResource,
    system::{Commands, ResMut, Resource},
};
use bevy_reflect::{prelude::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use bevy_render::mesh::{
    Indices, Mesh, MeshVertexAttribute, MeshVertexAttributeId, VertexAttributeValues,
};
use bevy_sprite::TextureAtlasLayout;
use bevy_state::state::OnExit;
use bevy_transform::components::Transform;
use bevy_utils::HashMap;
use froglight_common::{Direction, ResourceKey};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use super::ResourceAtlas;
use crate::{AssetCatalog, AssetState};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<BlockModelStorage>();

    // Clear the `BlockModelStorage` when assets are unloaded
    app.add_systems(OnExit(AssetState::Loaded), BlockModelStorage::clear);
    // Remove the `BlockDataStorage` when assets are unloaded
    app.add_systems(OnExit(AssetState::Loaded), BlockDataStorage::remove);

    // Register `BlockModel`
    app.register_type::<BlockModel>()
        .register_type::<Handle<BlockModel>>()
        .register_type_data::<Handle<BlockModel>, ReflectHandle>()
        .init_asset::<BlockModel>();
}

/// A block model.
///
/// Contains a [`Mesh`] and [`Transform`]s for different contexts.
#[derive(Debug, Default, Clone, PartialEq, Reflect, Asset)]
#[reflect(Default, Asset)]
pub struct BlockModel {
    /// The block model's mesh.
    pub mesh: Handle<Mesh>,

    /// Transforms used in different contexts.
    ///
    /// Indexed by [`ModelTransformIndex`].
    pub transforms: [Transform; 8],
}

impl BlockModel {
    /// Get the transform for a given [`ModelTransformIndex`].
    #[must_use]
    pub fn get_transform(&self, index: ModelTransformIndex) -> &Transform {
        &self.transforms[index as usize]
    }
}

/// An index into the `transforms` array of a [`BlockModel`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub enum ModelTransformIndex {
    /// When the player is in third person
    /// and the block is held in their right hand.
    #[serde(rename = "thirdperson_righthand")]
    ThirdPersonRighthand,
    /// When the player is in third person
    /// and the block is held in their left hand.
    #[serde(rename = "thirdperson_lefthand")]
    ThirdPersonLefthand,
    /// When the player is in first person
    /// and the block is held in their right hand.
    #[serde(rename = "firstperson_righthand")]
    FirstPersonRighthand,
    /// When the player is in first person
    /// and the block is held in their left hand.
    #[serde(rename = "firstperson_lefthand")]
    FirstPersonLefthand,
    /// When the block is inside an inventory.
    #[serde(rename = "gui")]
    Gui,
    /// When the block is on a player's head.
    #[serde(rename = "head")]
    Head,
    /// When the block is dropped on the ground.
    #[serde(rename = "ground")]
    Ground,
    /// When the block is held in an item frame.
    #[serde(rename = "fixed")]
    Fixed,
}

impl From<ModelTransformIndex> for usize {
    fn from(index: ModelTransformIndex) -> usize { index as usize }
}
impl From<usize> for ModelTransformIndex {
    fn from(index: usize) -> ModelTransformIndex {
        match index {
            0 => ModelTransformIndex::ThirdPersonRighthand,
            1 => ModelTransformIndex::ThirdPersonLefthand,
            2 => ModelTransformIndex::FirstPersonRighthand,
            3 => ModelTransformIndex::FirstPersonLefthand,
            4 => ModelTransformIndex::Gui,
            5 => ModelTransformIndex::Head,
            6 => ModelTransformIndex::Ground,
            7 => ModelTransformIndex::Fixed,
            _ => panic!("Invalid ModelTransformIndex"),
        }
    }
}

// --- Block Data ---

/// Get the block model data for a given [`BlockModel`].
#[derive(Debug)]
pub struct BlockModelData {
    /// The [`AssetId`] of the [`BlockModel`].
    pub asset_id: AssetId<BlockModel>,

    /// Whether ambient occlusion is enabled.
    pub ambient_occlusion: bool,

    /// The faces of the [`BlockModel`].
    ///
    /// Indexed by [`Direction`].
    pub faces: [ModelFaceData; 6],
}

impl BlockModelData {
    /// Get the face of the block model in a given [`Direction`].
    #[must_use]
    pub fn get_face(&self, direction: Direction) -> &ModelFaceData {
        &self.faces[usize::from(direction)]
    }

    /// Get the face of the block model in a given [`Direction`] mutably.
    #[must_use]
    pub fn get_face_mut(&mut self, direction: Direction) -> &mut ModelFaceData {
        &mut self.faces[usize::from(direction)]
    }
}

/// The data for a face of a [`BlockModel`].
#[derive(Debug)]
pub struct ModelFaceData {
    /// The attributes of the face.
    pub attributes: BTreeMap<MeshVertexAttributeId, (MeshVertexAttribute, VertexAttributeValues)>,
    /// The indices of the face.
    pub indices: Indices,
}

impl Default for ModelFaceData {
    fn default() -> Self {
        Self { attributes: BTreeMap::default(), indices: Indices::U32(Vec::new()) }
    }
}

impl ModelFaceData {
    /// Append [`MeshVertexAttribute`] data to the [`ModelFaceData`].
    pub fn append_to_face(
        &mut self,
        attribute: MeshVertexAttribute,
        values: VertexAttributeValues,
    ) {
        if let Some((_, face_attr)) = self.attributes.get_mut(&attribute.id) {
            Self::append_face_to_values(face_attr, values);
        } else {
            self.attributes.insert(attribute.id, (attribute, values));
        }
    }

    /// Append the [`ModelFaceData`] to a [`Mesh`].
    ///
    /// If the mesh does not have an attribute, it will be added.
    ///
    /// If the mesh does not have indices, they will be added.
    pub fn append_to_mesh(&self, mesh: &mut Mesh) {
        // Append the attributes
        for (attr_id, (attr, values)) in &self.attributes {
            if let Some(mesh_attr) = mesh.attribute_mut(*attr_id) {
                Self::append_face_to_values(mesh_attr, values.clone());
            } else {
                mesh.insert_attribute(attr.clone(), values.clone());
            }
        }

        // Append the indices
        match (mesh.indices_mut(), self.indices.clone()) {
            (Some(Indices::U32(mesh_ind)), Indices::U32(indices)) => {
                mesh_ind.extend(indices);
            }
            (Some(Indices::U16(mesh_ind)), Indices::U16(indices)) => {
                mesh_ind.extend(indices);
            }
            (None, indices) => {
                mesh.insert_indices(indices);
            }
            _ => {}
        }
    }

    /// Append the attribute data to the [`VertexAttributeValues`].
    fn append_face_to_values(mesh_attr: &mut VertexAttributeValues, values: VertexAttributeValues) {
        match (mesh_attr, values) {
            (VertexAttributeValues::Float32(a), VertexAttributeValues::Float32(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Sint32(a), VertexAttributeValues::Sint32(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Uint32(a), VertexAttributeValues::Uint32(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Float32x2(a), VertexAttributeValues::Float32x2(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Sint32x2(a), VertexAttributeValues::Sint32x2(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Uint32x2(a), VertexAttributeValues::Uint32x2(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Float32x3(a), VertexAttributeValues::Float32x3(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Sint32x3(a), VertexAttributeValues::Sint32x3(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Uint32x3(a), VertexAttributeValues::Uint32x3(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Float32x4(a), VertexAttributeValues::Float32x4(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Sint32x4(a), VertexAttributeValues::Sint32x4(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Uint32x4(a), VertexAttributeValues::Uint32x4(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Sint16x2(a), VertexAttributeValues::Sint16x2(b))
            | (VertexAttributeValues::Snorm16x2(a), VertexAttributeValues::Snorm16x2(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Uint16x2(a), VertexAttributeValues::Uint16x2(b))
            | (VertexAttributeValues::Unorm16x2(a), VertexAttributeValues::Unorm16x2(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Sint16x4(a), VertexAttributeValues::Sint16x4(b))
            | (VertexAttributeValues::Snorm16x4(a), VertexAttributeValues::Snorm16x4(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Uint16x4(a), VertexAttributeValues::Uint16x4(b))
            | (VertexAttributeValues::Unorm16x4(a), VertexAttributeValues::Unorm16x4(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Sint8x2(a), VertexAttributeValues::Sint8x2(b))
            | (VertexAttributeValues::Snorm8x2(a), VertexAttributeValues::Snorm8x2(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Uint8x2(a), VertexAttributeValues::Uint8x2(b))
            | (VertexAttributeValues::Unorm8x2(a), VertexAttributeValues::Unorm8x2(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Sint8x4(a), VertexAttributeValues::Sint8x4(b))
            | (VertexAttributeValues::Snorm8x4(a), VertexAttributeValues::Snorm8x4(b)) => {
                a.extend(b);
            }
            (VertexAttributeValues::Uint8x4(a), VertexAttributeValues::Uint8x4(b))
            | (VertexAttributeValues::Unorm8x4(a), VertexAttributeValues::Unorm8x4(b)) => {
                a.extend(b);
            }
            _ => panic!("Attribute Value Mismatch"),
        }
    }
}

// --- Block Data Storage ---

/// Storage for block data.
///
/// It is backed by an [`Arc`] so clones will share state.
/// Clones can be freely used in parallel.
#[derive(Debug, Clone, Resource, Deref)]
pub struct BlockDataStorage {
    inner: Arc<RwLock<BlockDataStorageInner>>,
}

impl BlockDataStorage {
    /// Create a new [`BlockDataStorage`] with the given [`TextureAtlasLayout`].
    ///
    /// This **must** be the layout of the
    /// [`ResourceAtlas`](super::ResourceAtlas) located at
    /// `minecraft:blocks`.
    #[must_use]
    pub fn new(atlas_layout: TextureAtlasLayout) -> Self {
        Self {
            inner: Arc::new(RwLock::new(BlockDataStorageInner {
                model_data: HashMap::default(),
                block_atlas: atlas_layout,
            })),
        }
    }

    /// Create a new [`BlockDataStorage`] using the [`AssetCatalog`].
    ///
    /// Requires the `minecraft:blocks` [`ResourceAtlas`](super::ResourceAtlas)
    /// be loaded inside the [`AssetCatalog`].
    #[must_use]
    pub fn from_catalog(
        catalog: &AssetCatalog,
        atlases: &Assets<ResourceAtlas>,
        layouts: &Assets<TextureAtlasLayout>,
    ) -> Option<Self> {
        let atlas_id = catalog.get::<ResourceAtlas>("minecraft:blocks")?;
        let atlas = atlases.get(atlas_id)?;
        let layout = layouts.get(&atlas.atlas_layout)?;
        Some(Self::new(layout.clone()))
    }

    /// Remove the [`BlockDataStorage`].
    fn remove(mut commands: Commands) { commands.remove_resource::<Self>(); }
}

/// The inner storage for [`BlockDataStorage`].
#[derive(Debug)]
pub struct BlockDataStorageInner {
    pub model_data: HashMap<ResourceKey, BlockModelData>,
    pub block_atlas: TextureAtlasLayout,
}

// --- Handle Storage ---

/// A [`Vec`] used to store [`Handle::Strong`] references to [`BlockModel`]s.
///
/// Without this, [`BlockModel`]s would unload when they are no longer in use.
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect, Resource, Deref, DerefMut)]
#[reflect(Default, Resource)]
pub(crate) struct BlockModelStorage {
    inner: Vec<Handle<BlockModel>>,
}
impl BlockModelStorage {
    /// Clear the [`BlockModelStorage`].
    fn clear(mut res: ResMut<Self>) { res.clear(); }
}
