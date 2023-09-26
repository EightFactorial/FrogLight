use super::{Block, BlocksMap};

pub trait BlocksMapFn {
    fn get_block(&self, block_id: &u32) -> &Block;
    fn get_fallback(&self) -> &Block;
}

impl BlocksMapFn for BlocksMap {
    fn get_block(&self, block_id: &u32) -> &Block { self.get(block_id).unwrap_or(&self[&u32::MAX]) }
    fn get_fallback(&self) -> &Block { &self[&u32::MAX] }
}