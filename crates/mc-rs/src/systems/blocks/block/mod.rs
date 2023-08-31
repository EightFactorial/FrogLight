use std::{hash::Hash, ops::Range, sync::Arc};

use bevy::prelude::*;
use mc_rs_proto::types::ResourceLocation;
use nohash_hasher::IntMap;
use parking_lot::RwLock;

use self::properties::BlockProperties;

pub mod properties;

mod blocksmap;
pub use blocksmap::BlocksMapFn;

#[derive(Debug, Clone, Deref, DerefMut, Resource)]
pub struct Blocks(pub Arc<RwLock<BlocksMap>>);
pub(super) type BlocksMap = IntMap<u32, Block>;

#[derive(Debug, Clone)]
pub struct Block {
    pub block_id: u32,
    pub block_states: Range<u32>,
    pub name: String,
    pub key: ResourceLocation,
    pub properties: BlockProperties,
}

impl Eq for Block {}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool { self.block_id == other.block_id }
}
impl Hash for Block {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.block_id.hash(state); }
}

impl Blocks {
    pub(super) fn create(mut commands: Commands) {
        let mut blocks = BlocksMap::default();

        blocks.insert(
            0u32,
            Block {
                block_id: 0u32,
                block_states: 0..0,
                name: "Air".to_string(),
                key: ResourceLocation::new("minecraft:air"),
                properties: BlockProperties {
                    is_air: true,
                    ..Default::default()
                },
            },
        );

        blocks.insert(
            u32::MAX,
            Block {
                block_id: u32::MAX,
                block_states: u32::MAX..u32::MAX,
                name: "Error".to_string(),
                key: ResourceLocation::new("mc-rs:error"),
                properties: BlockProperties::default(),
            },
        );

        commands.insert_resource(Blocks(Arc::new(RwLock::new(blocks))));
    }
}
