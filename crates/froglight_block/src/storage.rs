//! TODO

use core::{
    fmt::Debug,
    ops::{Deref, Range},
};

use froglight_common::version::Version;
use rangemap::RangeMap;

use crate::{
    block::{Block, BlockType, GlobalBlockState},
    info::BlockInfo,
};

/// A [`Version`] with an associated [`BlockMap`].
pub trait Blocks: Version {
    /// Get the [`StaticBlockMap`] for this [`Version`].
    fn blocks() -> &'static StaticBlockMap;
    /// Initialize this version's blocks into the provided [`BlockMap`].
    fn init_blocks(map: &mut BlockMap);
}

// -------------------------------------------------------------------------------------------------

/// A modifiable, thread-safe reference to a [`BlockMap`].
#[repr(transparent)]
pub struct StaticBlockMap(
    #[cfg(feature = "async")] async_lock::RwLock<BlockMap>,
    #[cfg(not(feature = "async"))] parking_lot::RwLock<BlockMap>,
);

impl StaticBlockMap {
    /// Create a new [`StaticBlockMap`].
    #[must_use]
    #[cfg(feature = "async")]
    pub const fn new(map: BlockMap) -> Self { StaticBlockMap(async_lock::RwLock::new(map)) }

    /// Read the [`BlockMap`], blocking the current thread if necessary.
    #[must_use]
    #[cfg(feature = "async")]
    pub fn read_blocking(&self) -> async_lock::RwLockReadGuard<'_, BlockMap> {
        self.0.read_blocking()
    }

    /// Create a new [`StaticBlockMap`].
    #[must_use]
    #[cfg(not(feature = "async"))]
    pub const fn new(map: BlockMap) -> Self { StaticBlockMap(parking_lot::RwLock::new(map)) }

    /// Read the [`BlockMap`], blocking the current thread if necessary.
    #[cfg(not(feature = "async"))]
    pub fn read_blocking(&self) -> parking_lot::RwLockReadGuard<'_, BlockMap> { self.0.read() }
}

impl Debug for StaticBlockMap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticBlockMap").finish_non_exhaustive()
    }
}

impl Deref for StaticBlockMap {
    #[cfg(feature = "async")]
    type Target = async_lock::RwLock<BlockMap>;
    #[cfg(not(feature = "async"))]
    type Target = parking_lot::RwLock<BlockMap>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

// -------------------------------------------------------------------------------------------------

/// A map of [`GlobalBlockState`]s to their corresponding [`BlockInfo`].
///
/// Used for assigning ids to block states and retrieving their information.
pub struct BlockMap(RangeMap<u32, &'static BlockInfo>);

impl BlockMap {
    /// Create a new empty [`BlockMap`].
    #[must_use]
    pub const fn new_empty() -> Self { BlockMap(RangeMap::new()) }

    /// Initialize the [`BlockMap`] with blocks from the given version.
    #[inline]
    pub fn init<V: Blocks>(&mut self) { V::init_blocks(self); }

    /// Get a [`Block`] for a given [`GlobalBlockState`].
    ///
    /// Returns `None` if the block state is not registered in the [`BlockMap`].
    #[must_use]
    pub fn get_block(&self, block: GlobalBlockState) -> Option<Block> {
        let block_info = self.get_info(block)?;
        let base_id = block_info.base_id();

        // SAFETY: The block state is guaranteed to be valid for the block info.
        Some(unsafe { Block::new_unchecked(block_info, u16::try_from(*block - base_id).ok()?) })
    }

    /// Get the [`BlockInfo`] for a given [`GlobalBlockState`].
    ///
    /// Returns `None` if the block state is not registered in the [`BlockMap`].
    #[must_use]
    #[allow(clippy::needless_else, reason = "Only needless when `tracing` is disabled.")]
    pub fn get_info(&self, block: GlobalBlockState) -> Option<&'static BlockInfo> {
        if let Some(info) = self.0.get(&block) {
            Some(info)
        } else {
            #[cfg(feature = "tracing")]
            tracing::error!("BlockMap: No BlockInfo found for block `{}`!", *block);
            None
        }
    }

    /// Get the range of [`GlobalBlockState`]s for a given [`GlobalBlockState`].
    ///
    /// Returns `None` if the block state is not registered in the [`BlockMap`].
    #[must_use]
    #[allow(clippy::needless_else, reason = "Only needless when `tracing` is disabled.")]
    pub fn get_range(&self, block: GlobalBlockState) -> Option<Range<GlobalBlockState>> {
        if let Some((r, _)) = self.0.get_key_value(&block) {
            Some(Range {
                start: GlobalBlockState::from(r.start),
                end: GlobalBlockState::from(r.end),
            })
        } else {
            #[cfg(feature = "tracing")]
            tracing::error!("BlockMap: No BlockInfo found for block `{}`!", *block);
            None
        }
    }

    /// Get the number of blocks registered in this [`BlockMap`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the [`BlockMap`] is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Register a [`BlockType`] in the [`BlockMap`].
    ///
    /// Assigns a range of [`GlobalBlockState`]s to the [`BlockType`],
    /// starting from the last id to however many states the block has.
    #[inline]
    pub fn register<B: BlockType<V>, V: Version>(&mut self) { self.register_untyped(B::info()); }

    /// Register a [`BlockType`] in the [`BlockMap`].
    ///
    /// Assigns a range of [`GlobalBlockState`]s to the [`BlockType`],
    /// starting from the last id to however many states the block has.
    #[expect(clippy::missing_panics_doc, reason = "A block will never have 4,294,967,295 states")]
    pub fn register_untyped(&mut self, info: &'static BlockInfo) {
        let count = u32::try_from(info.states()).expect("BlockType has too many states!");
        let range = self.0.last_range_value().map_or_else(
            || Range { start: 0, end: count + 1 },
            |(r, _)| Range { start: r.end, end: r.end + count + 1 },
        );

        info.set_registered_id(range.start);
        self.0.insert(range, info);
    }

    /// Get a reference to the inner [`RangeMap`] of the [`BlockMap`].
    ///
    /// Requires calling [`BlockMap::as_inner`] explicitly.
    #[inline]
    #[must_use]
    pub fn as_inner(map: &Self) -> &RangeMap<u32, &'static BlockInfo> { &map.0 }

    /// Get a mutable reference to the inner [`RangeMap`] of the [`BlockMap`].
    ///
    /// Requires calling [`BlockMap::as_inner_mut`] explicitly.
    #[inline]
    #[must_use]
    pub fn as_inner_mut(map: &mut Self) -> &mut RangeMap<u32, &'static BlockInfo> { &mut map.0 }
}
