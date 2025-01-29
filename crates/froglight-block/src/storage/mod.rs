//! TODO

use std::{ops::Range, sync::Arc};

use derive_more::derive::Deref;
use froglight_common::Version;
use parking_lot::RwLock;
use rangemap::RangeMap;

use crate::block::{
    BlockAttributes, BlockType, BlockTypeExt, GlobalBlockState, RelativeBlockState, UntypedBlock,
};

#[derive(Default, Clone, Deref)]
pub struct AppBlockStorage<V: Version>(Arc<RwLock<BlockStorage<V>>>);

pub struct BlockStorage<V: Version>(RangeMap<u32, EqWrapper<V>>);

impl<V: Version> Default for BlockStorage<V> {
    fn default() -> Self { todo!() }
}

impl<V: Version> BlockStorage<V> {
    /// Create a new empty [`BlockStorage`].
    #[must_use]
    pub fn new_empty() -> Self { Self(RangeMap::new()) }

    /// Get an [`UntypedBlock`] from a [`GlobalBlockState`].
    #[must_use]
    #[expect(clippy::missing_panics_doc)]
    pub fn get_untyped(&self, block: GlobalBlockState) -> Option<UntypedBlock<V>> {
        let (range, wrapper) = self.0.get_key_value(&block.0)?;
        let relative = u16::try_from(range.start - block.0).expect("Way too many blockstates!");
        Some(UntypedBlock(RelativeBlockState(relative), wrapper.0))
    }

    /// Register a block type in the storage.
    #[expect(clippy::cast_possible_truncation)]
    pub fn register<B: BlockTypeExt<V>>(&mut self) {
        let range = self.0.last_range_value().map_or_else(
            || Range { start: 0, end: B::Attributes::COUNT as u32 },
            |(r, _)| Range { start: r.end, end: r.end + B::Attributes::COUNT as u32 },
        );
        self.0.insert(range, EqWrapper(B::as_static()));
    }
}

#[derive(Clone, Copy)]
struct EqWrapper<V: Version>(&'static dyn BlockType<V>);
impl<V: Version> PartialEq for EqWrapper<V> {
    fn eq(&self, other: &Self) -> bool { std::ptr::eq(self, other) }
}
impl<V: Version> Eq for EqWrapper<V> {}
