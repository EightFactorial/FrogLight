//! TODO

use froglight_common::version::Version;

use crate::{block::UntypedBlock, storage::BlockStorage};

/// A trait for resolving block types from global block IDs.
pub trait BlockResolver<V: Version> {
    /// The possible block types that can be resolved.
    type BlockEnum: Sized;

    /// Register all known [`BlockType`](crate::prelude::BlockType)s with the
    /// given [`BlockStorage`].
    fn register(storage: &mut BlockStorage<V>);

    /// Resolve the block type for the given [`UntypedBlock`].
    fn resolve(block: UntypedBlock<V>) -> Option<Self::BlockEnum>;
}
