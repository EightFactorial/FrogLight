use std::{hash::Hash, sync::Arc};

use bevy::prelude::*;
use nohash_hasher::IntMap;
use parking_lot::RwLock;

use self::{
    meshing::{BlockMesh, BlockMeshData},
    model::BlockModel,
    textures::BlockTextures,
};

use super::block::{Block, BlocksMap, BlocksMapFn};

pub mod meshing;
pub mod model;
pub mod textures;

mod statesmap;
pub use statesmap::StatesMapFn;

#[derive(Debug, Clone, Deref, DerefMut, Resource)]
pub struct BlockStates(pub Arc<RwLock<StatesMap>>);
pub(super) type StatesMap = IntMap<u32, BlockState>;

#[derive(Debug, Clone)]
pub struct BlockState {
    pub block_id: u32,
    pub state_id: u32,
    pub textures: BlockTextures,
    pub model: BlockModel,
}

impl Eq for BlockState {}
impl PartialEq for BlockState {
    fn eq(&self, other: &Self) -> bool { self.state_id == other.state_id }
}
impl Hash for BlockState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.state_id.hash(state); }
}

impl BlockState {
    pub fn get_block<'a>(&self, blocks: &'a BlocksMap) -> &'a Block {
        blocks.get_block(&self.block_id)
    }

    pub fn get_meshing(&self, blocks: &BlocksMap) -> BlockMeshData {
        let meshing = match &self.model {
            BlockModel::Standard => BlockMesh::Always,
            BlockModel::Simple { collision } => {
                let [min_x, min_y, min_z] = collision.min().to_array().map(|i| i as u32);
                let [max_x, max_y, max_z] = collision.max().to_array().map(|i| i as u32);

                match (min_x, min_y, min_z, max_x, max_y, max_z) {
                    (0, 0, 0, 0, 0, 0) => BlockMesh::Never,
                    (0, 0, 0, 16, 16, 16) => BlockMesh::Always,
                    _ => BlockMesh::Custom([
                        min_y == 0,
                        max_y == 16,
                        min_z == 0,
                        max_z == 16,
                        min_x == 0,
                        max_x == 16,
                    ]),
                }
            }
            BlockModel::None | BlockModel::Custom { .. } => BlockMesh::Never,
        };

        BlockMeshData {
            voxel: self.get_block(blocks).into(),
            meshing,
        }
    }
}

impl BlockStates {
    pub(super) fn create(asset_server: Res<AssetServer>, mut commands: Commands) {
        let mut states = StatesMap::default();

        states.insert(
            u32::MAX,
            BlockState {
                block_id: u32::MAX,
                state_id: u32::MAX,
                textures: BlockTextures::default(),
                model: BlockModel::Standard,
            },
        );

        commands.insert_resource(BlockStates(Arc::new(RwLock::new(states))));
    }
}
