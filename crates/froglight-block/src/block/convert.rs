use alloc::boxed::Box;

use froglight_common::version::Version;

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
    fn convert_from(block: Block<B, V2>) -> Block<B, V1> { Block::new(*block.state()) }

    #[inline]
    fn convert_into(block: Block<B, V1>) -> Block<B, V2> { Block::new(*block.state()) }
}

impl<B: BlockConvert<V1, V2>, V1: Version, V2: Version> From<&Block<B, V1>> for Block<B, V2> {
    #[inline]
    fn from(block: &Block<B, V1>) -> Self { B::convert_into(*block) }
}
impl<B: BlockConvert<V1, V2>, V1: Version, V2: Version> From<&&Block<B, V2>> for Block<B, V1> {
    #[inline]
    fn from(block: &&Block<B, V2>) -> Self { B::convert_from(**block) }
}

// -------------------------------------------------------------------------------------------------

impl<B: BlockTypeExt<V>, V: Version> Block<B, V> {
    /// Convert a [`Block`] from another [`Version`] into this [`Version`].
    #[inline]
    #[must_use]
    pub fn from_version<V2: Version>(block: Block<B, V2>) -> Block<B, V>
    where B: BlockConvert<V, V2> {
        B::convert_from(block)
    }

    /// Convert this [`Block`] into a [`Block`] from another [`Version`].
    #[inline]
    #[must_use]
    pub fn into_version<V2: Version>(self) -> Block<B, V2>
    where B: BlockConvert<V, V2> {
        B::convert_into(self)
    }
}

// -------------------------------------------------------------------------------------------------

/// A helper for converting blocks between multiple [`Version`]s.
///
/// # Note
/// It is much faster to use the [`BlockConvert`] trait directly,
/// but where many conversions are needed this can be more convenient.
#[expect(clippy::type_complexity)]
pub struct BlockConverter<B: BlockTypeExt<V1> + BlockTypeExt<V2>, V1: Version, V2: Version> {
    from_fn: Box<dyn Fn(Block<B, V2>) -> Block<B, V1>>,
    into_fn: Box<dyn Fn(Block<B, V1>) -> Block<B, V2>>,
}

impl<B: BlockConvert<V1, V2>, V1: Version, V2: Version> Default for BlockConverter<B, V1, V2> {
    #[inline]
    fn default() -> Self { Self::new() }
}

impl<B: BlockTypeExt<V1> + BlockTypeExt<V2>, V1: Version, V2: Version> BlockConverter<B, V1, V2> {
    /// Create a new [`BlockConverter`] for
    /// converting blocks between two [`Version`]s.
    #[inline]
    #[must_use]
    pub fn new() -> Self
    where B: BlockConvert<V1, V2> {
        Self { from_fn: Box::new(B::convert_from), into_fn: Box::new(B::convert_into) }
    }

    /// Extend the [`BlockConverter`] forward and backward another [`Version`].
    #[must_use]
    pub fn extend<V0: Version, V3: Version>(self) -> BlockConverter<B, V0, V3>
    where B: BlockConvert<V0, V1> + BlockConvert<V2, V3> {
        BlockConverter::<B, V0, V3> {
            from_fn: Box::new(move |block| B::convert_from((self.from_fn)(B::convert_from(block)))),
            into_fn: Box::new(move |block| B::convert_into((self.into_fn)(B::convert_into(block)))),
        }
    }

    /// Extend the [`BlockConverter`] forward an additional [`Version`].
    #[must_use]
    pub fn extend_front<V3: Version>(self) -> BlockConverter<B, V1, V3>
    where B: BlockConvert<V2, V3> {
        BlockConverter::<B, V1, V3> {
            from_fn: Box::new(move |block| (self.from_fn)(B::convert_from(block))),
            into_fn: Box::new(move |block| B::convert_into((self.into_fn)(block))),
        }
    }

    /// Extend the [`BlockConverter`] backward an additional [`Version`].
    #[must_use]
    pub fn extend_back<V0: Version>(self) -> BlockConverter<B, V0, V2>
    where B: BlockConvert<V0, V1> + BlockConvert<V1, V2> {
        BlockConverter::<B, V0, V2> {
            from_fn: Box::new(move |block| B::convert_from((self.from_fn)(block))),
            into_fn: Box::new(move |block| (self.into_fn)(B::convert_into(block))),
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
