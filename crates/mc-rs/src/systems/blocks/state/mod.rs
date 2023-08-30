use std::{hash::Hash, sync::Arc};

use bevy::prelude::*;
use nohash_hasher::IntMap;
use parking_lot::RwLock;

use self::model::BlockModel;

use super::block::{Block, BlocksMap, BlocksMapFn};

pub mod model;

mod statesmap;
pub use statesmap::StatesMapFn;

#[derive(Debug, Clone, Deref, DerefMut, Resource)]
pub struct BlockStates(pub Arc<RwLock<StatesMap>>);
pub(super) type StatesMap = IntMap<u32, BlockState>;

#[derive(Debug, Clone)]
pub struct BlockState {
    pub block_id: u32,
    pub state_id: u32,
    pub model: BlockModel,
}

impl Eq for BlockState {}
impl PartialEq for BlockState {
    fn eq(&self, other: &Self) -> bool { self.state_id == other.state_id }
}
impl Hash for BlockState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.state_id.hash(state); }
}

impl BlockStates {
    pub(super) fn create(asset_server: Res<AssetServer>, mut commands: Commands) {
        let mut states = StatesMap::default();

        commands.insert_resource(BlockStates(Arc::new(RwLock::new(states))));
    }
}

impl BlockState {
    pub fn get_block<'a>(&self, blocks: &'a BlocksMap) -> &'a Block {
        blocks.get_block(&self.block_id)
    }
}
