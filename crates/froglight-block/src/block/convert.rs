use froglight_common::Version;

use super::{Block, BlockTypeExt};
use crate::storage::BlockAttributes;

/// A trait for converting blocks between versions.
pub trait BlockConvert<V1: Version, V2: Version>:
    BlockTypeExt<V1> + BlockTypeExt<V2> + Sized
{
    /// Convert a [`Block`] from another [`Version`] into this [`Version`].
    #[must_use]
    fn convert_from(block: Block<Self, V2>) -> Block<Self, V1>;
    /// Convert a [`Block`] from this [`Version`] into another [`Version`].
    #[must_use]
    fn convert_into(block: Block<Self, V1>) -> Block<Self, V2>;
}

// Implement [`BlockConvert`] for each [`Block`] across all [`Version`]s
// where they have identical [`BlockAttributes`].
//
// This also covers the `BlockConvert<V, V>` case.
impl<
        A: BlockAttributes,
        B: BlockTypeExt<V1, Attributes = A> + BlockTypeExt<V2, Attributes = A>,
        V1: Version,
        V2: Version,
    > BlockConvert<V1, V2> for B
{
    #[inline]
    #[must_use]
    fn convert_from(block: Block<B, V2>) -> Block<B, V1> { Block::new(*block.state()) }
    #[inline]
    #[must_use]
    fn convert_into(block: Block<B, V1>) -> Block<B, V2> { Block::new(*block.state()) }
}

impl<B: BlockConvert<V1, V2>, V1: Version, V2: Version> From<&Block<B, V1>> for Block<B, V2> {
    #[inline]
    #[must_use]
    fn from(block: &Block<B, V1>) -> Self { B::convert_into(*block) }
}
impl<B: BlockConvert<V1, V2>, V1: Version, V2: Version> From<&&Block<B, V2>> for Block<B, V1> {
    #[inline]
    #[must_use]
    fn from(block: &&Block<B, V2>) -> Self { B::convert_from(**block) }
}

// -------------------------------------------------------------------------------------------------

/// A helper for converting blocks between multiple [`Version`]s.
///
/// Allows for chaining [`BlockConvert`] implementations.
pub struct BlockConverter<B: BlockTypeExt<V1> + BlockTypeExt<V2>, V1: Version, V2: Version> {
    from_fn: fn(Block<B, V2>) -> Block<B, V1>,
    into_fn: fn(Block<B, V1>) -> Block<B, V2>,
}

impl<B: BlockConvert<V1, V2>, V1: Version, V2: Version> Default for BlockConverter<B, V1, V2> {
    #[inline]
    fn default() -> Self { Self::new() }
}

impl<B: BlockConvert<V1, V2>, V1: Version, V2: Version> BlockConverter<B, V1, V2> {
    /// Create a new [`BlockChannel`] for
    /// converting blocks between two [`Version`]s.
    #[inline]
    #[must_use]
    pub const fn new() -> Self { Self { from_fn: B::convert_from, into_fn: B::convert_into } }

    /// Extend the [`BlockChannel`] forward and backward another [`Version`].
    #[must_use]
    pub const fn extend<V0: Version, V3: Version>(self) -> BlockConverter<B, V0, V3>
    where
        B: BlockConvert<V0, V1> + BlockConvert<V1, V2> + BlockConvert<V2, V3>,
    {
        BlockConverter::<B, V0, V3> {
            from_fn: |block| B::convert_from(B::convert_from(B::convert_from(block))),
            into_fn: |block| B::convert_into(B::convert_into(B::convert_into(block))),
        }
    }

    /// Extend the [`BlockChannel`] forward an additional [`Version`].
    #[must_use]
    pub const fn extend_front<V3: Version>(self) -> BlockConverter<B, V1, V3>
    where
        B: BlockConvert<V2, V3>,
    {
        BlockConverter::<B, V1, V3> {
            from_fn: |block| B::convert_from(B::convert_from(block)),
            into_fn: |block| B::convert_into(B::convert_into(block)),
        }
    }

    /// Extend the [`BlockChannel`] backward an additional [`Version`].
    #[must_use]
    pub const fn extend_back<V0: Version>(self) -> BlockConverter<B, V0, V2>
    where
        B: BlockConvert<V0, V1>,
    {
        BlockConverter::<B, V0, V2> {
            from_fn: |block| B::convert_from(B::convert_from(block)),
            into_fn: |block| B::convert_into(B::convert_into(block)),
        }
    }

    /// Convert a [`Block`] between [`Version`]s.
    #[inline]
    #[must_use]
    pub fn from(&self, block: Block<B, V2>) -> Block<B, V1> { (self.from_fn)(block) }

    /// Convert a [`Block`] between [`Version`]s.
    #[inline]
    #[must_use]
    pub fn into(&self, block: Block<B, V1>) -> Block<B, V2> { (self.into_fn)(block) }
}
