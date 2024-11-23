use froglight_protocol::traits::Version;

use crate::BlockStorage;

mod v1_21_0;

/// A resolver for vanilla blocks.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VanillaResolver;

/// A trait for converting block IDs to blocks.
pub trait BlockResolver<V: Version> {
    /// The output type of the resolver.
    type Output;
    /// Resolve a block ID into a block.
    fn resolve(block_id: u32, storage: &BlockStorage<V>) -> Self::Output;
}
