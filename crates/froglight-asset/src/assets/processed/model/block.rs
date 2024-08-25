use std::sync::Arc;

use bevy_app::App;
use bevy_asset::{Asset, AssetApp, Handle, ReflectAsset, ReflectHandle};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::system::Resource;
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use bevy_render::mesh::Mesh;
use bevy_transform::components::Transform;
use bevy_utils::HashMap;
use froglight_common::ResourceKey;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_asset::<BlockModel>();

    app.register_type::<BlockModel>()
        .register_type::<Handle<BlockModel>>()
        .register_type_data::<Handle<BlockModel>, ReflectHandle>();

    app.init_resource::<BlockModelCache>();
    app.register_type::<ModelTransformIndex>();
}

/// A block model.
#[derive(Debug, Clone, Asset, Reflect)]
#[reflect(Asset)]
pub struct BlockModel {
    /// The mesh for the block.
    pub block_mesh: Handle<Mesh>,

    /// The transforms for the model.
    pub transforms: [Transform; 8],
}

impl BlockModel {
    /// Returns the transform for the given index.
    #[must_use]
    pub fn transform(&self, index: ModelTransformIndex) -> &Transform {
        &self.transforms[usize::from(index)]
    }
}

/// A cache of block face meshes.
///
/// Used for building terrain meshes.
///
/// Indexed by block [`ResourceKey`]s and face [`Direction`]s.
#[derive(Debug, Default, Clone, Deref, DerefMut, Resource)]
pub struct BlockModelCache(Arc<RwLock<HashMap<ResourceKey, [Mesh; 6]>>>);

/// An index into the `transforms` array of a [`BlockModel`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect, Serialize, Deserialize,
)]
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
    fn from(index: ModelTransformIndex) -> usize {
        match index {
            ModelTransformIndex::ThirdPersonRighthand => 0,
            ModelTransformIndex::ThirdPersonLefthand => 1,
            ModelTransformIndex::FirstPersonRighthand => 2,
            ModelTransformIndex::FirstPersonLefthand => 3,
            ModelTransformIndex::Gui => 4,
            ModelTransformIndex::Head => 5,
            ModelTransformIndex::Ground => 6,
            ModelTransformIndex::Fixed => 7,
        }
    }
}
impl TryFrom<usize> for ModelTransformIndex {
    type Error = ();
    fn try_from(index: usize) -> Result<Self, Self::Error> {
        match index {
            0 => Ok(ModelTransformIndex::ThirdPersonRighthand),
            1 => Ok(ModelTransformIndex::ThirdPersonLefthand),
            2 => Ok(ModelTransformIndex::FirstPersonRighthand),
            3 => Ok(ModelTransformIndex::FirstPersonLefthand),
            4 => Ok(ModelTransformIndex::Gui),
            5 => Ok(ModelTransformIndex::Head),
            6 => Ok(ModelTransformIndex::Ground),
            7 => Ok(ModelTransformIndex::Fixed),
            _ => Err(()),
        }
    }
}
