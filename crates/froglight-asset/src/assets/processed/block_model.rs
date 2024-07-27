use std::sync::Arc;

use bevy_app::App;
use bevy_asset::{Asset, AssetApp, AssetId, Assets, Handle, ReflectAsset, ReflectHandle};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    reflect::ReflectResource,
    system::{Commands, ResMut, Resource},
};
use bevy_reflect::{prelude::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use bevy_render::mesh::Mesh;
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

/// Get the block model data for a given [`BlockModel`].
#[derive(Debug)]
pub struct BlockModelData {
    /// The [`AssetId`] of the [`BlockModel`].
    pub asset_id: AssetId<BlockModel>,

    /// The faces of the [`BlockModel`].
    ///
    /// Indexed by [`Direction`].
    pub faces: [Mesh; 6],

    /// Whether ambient occlusion is enabled.
    pub ambient_occlusion: bool,
}

impl BlockModelData {
    /// Get the face of the block model in a given [`Direction`].
    #[must_use]
    pub fn get_face(&self, direction: Direction) -> &Mesh { &self.faces[usize::from(direction)] }

    /// Get the face of the block model in a given [`Direction`] mutably.
    #[must_use]
    pub fn get_face_mut(&mut self, direction: Direction) -> &mut Mesh {
        &mut self.faces[usize::from(direction)]
    }
}

// --- Handle Storage ---

/// A [`Vec`] used to store [`Handle::Strong`] references to [`BlockModel`]s.
///
/// Without this, [`BlockModel`]s would unload when they are no longer in use.
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect, Resource, Deref, DerefMut)]
#[reflect(Default, Resource)]
pub(crate) struct BlockModelStorage {
    models: Vec<Handle<BlockModel>>,
}
impl BlockModelStorage {
    /// Clear the [`BlockModelStorage`].
    fn clear(mut res: ResMut<Self>) { res.clear(); }
}
