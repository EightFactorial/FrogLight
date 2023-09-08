use std::{hash::Hash, sync::Arc};

use bevy::prelude::*;
use nohash_hasher::IntMap;
use parking_lot::RwLock;

use self::{model::BlockModel, textures::BlockTextures};

use super::block::{Block, BlocksMap, BlocksMapFn};

mod list;
pub mod meshing;
pub mod model;
pub mod textures;

mod statesmap;
pub use statesmap::StatesMapFn;

#[derive(Debug, Clone, Deref, DerefMut)]
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
}

impl BlockStates {
    pub(super) fn create(asset_server: &AssetServer) -> BlockStates {
        let mut states = StatesMap::default();

        list::create_states(&mut states, asset_server);

        // Add the fallback block state
        states.insert(
            u32::MAX,
            BlockState {
                block_id: u32::MAX,
                state_id: u32::MAX,
                textures: BlockTextures::new(&["light_blue_wool.png"], asset_server),
                model: BlockModel::Standard,
            },
        );

        BlockStates(Arc::new(RwLock::new(states)))
    }
}
