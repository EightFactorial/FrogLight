use bevy_app::App;
use bevy_asset::{Asset, AssetApp, Handle, ReflectAsset, ReflectHandle};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    reflect::ReflectResource,
    system::{ResMut, Resource},
};
use bevy_reflect::{prelude::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use bevy_render::mesh::Mesh;
use bevy_state::state::OnExit;
use bevy_transform::components::Transform;
use serde::{Deserialize, Serialize};

use crate::AssetState;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<BlockModelStorage>();

    // Clear the `BlockModelStorage` when assets are unloaded
    app.add_systems(OnExit(AssetState::Loaded), BlockModelStorage::clear);

    // Register `BlockModel`
    app.register_type::<BlockModel>()
        .register_type::<Handle<BlockModel>>()
        .register_type_data::<Handle<BlockModel>, ReflectHandle>()
        .init_asset::<BlockModel>();
}

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
    fn clear(mut res: ResMut<Self>) { res.clear() }
}

/// A block model.
#[derive(Debug, Default, Clone, PartialEq, Reflect, Asset)]
#[reflect(Default, Asset)]
pub struct BlockModel {
    /// The block model's mesh.
    pub mesh: Handle<Mesh>,

    /// Whether the block has ambient occlusion.
    pub ambient_occlusion: bool,

    /// Transforms used in different contexts.
    ///
    /// Indexed by [`ModelTransformIndex`].
    pub transforms: [Transform; 8],
}

/// An index into the `transforms` array of a [`BlockModel`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
#[repr(u8)]
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
