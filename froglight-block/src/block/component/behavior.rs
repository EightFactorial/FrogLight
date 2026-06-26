#![expect(clippy::too_many_arguments, reason = "Only when manually constructing a BlockBehavior")]
#![expect(missing_docs, reason = "The functions themselves have documentation")]

use crate::{
    block::{BlockShape, BlockType},
    state::StateId,
    version::BlockVersion,
};

type StateFn<T> = fn(StateId) -> T;

/// A collection of functions that define a block's behavior.
#[derive(Clone, Copy)]
pub struct BlockBehavior {
    pub is_air: StateFn<bool>,
    pub is_solid: StateFn<bool>,
    pub is_liquid: StateFn<bool>,
    pub has_collision: StateFn<bool>,
    pub is_transparent: StateFn<bool>,
    pub has_occlusion: StateFn<bool>,
    pub light_emission: StateFn<u8>,
    pub shape_of: StateFn<&'static BlockShape<'static>>,
}

impl BlockBehavior {
    /// Create a new [`BlockBehavior`] for the given [`BlockType`].
    #[must_use]
    pub const fn new<B: BlockType<V>, V: BlockVersion>() -> Self {
        Self::new_manual(
            B::is_air,
            B::is_solid,
            B::is_liquid,
            B::has_collision,
            B::is_transparent,
            B::has_occlusion,
            B::light_emission,
            B::shape_of,
        )
    }

    /// Create a new [`BlockBehavior`] from manually provided functions.
    #[must_use]
    pub const fn new_manual(
        is_air: StateFn<bool>,
        is_solid: StateFn<bool>,
        is_liquid: StateFn<bool>,
        has_collision: StateFn<bool>,
        is_transparent: StateFn<bool>,
        has_occlusion: StateFn<bool>,
        light_emission: StateFn<u8>,
        shape_of: StateFn<&'static BlockShape<'static>>,
    ) -> Self {
        Self {
            is_air,
            is_solid,
            is_liquid,
            has_collision,
            is_transparent,
            has_occlusion,
            light_emission,
            shape_of,
        }
    }

    /// Returns `true` if the block is air.
    #[inline]
    #[must_use]
    pub fn is_air(&self, state: StateId) -> bool { (self.is_air)(state) }

    /// Returns `true` if the block is solid.
    #[inline]
    #[must_use]
    pub fn is_solid(&self, state: StateId) -> bool { (self.is_solid)(state) }

    /// Returns `true` if the block is liquid.
    #[inline]
    #[must_use]
    pub fn is_liquid(&self, state: StateId) -> bool { (self.is_liquid)(state) }

    /// Returns `true` if the block has collision.
    #[inline]
    #[must_use]
    pub fn has_collision(&self, state: StateId) -> bool { (self.has_collision)(state) }

    /// Returns `true` if the block has occlusion.
    #[inline]
    #[must_use]
    pub fn has_occlusion(&self, state: StateId) -> bool { (self.has_occlusion)(state) }

    /// Returns `true` if the block is transparent.
    #[inline]
    #[must_use]
    pub fn is_transparent(&self, state: StateId) -> bool { (self.is_transparent)(state) }

    /// Returns the light emission level of the block.
    #[inline]
    #[must_use]
    pub fn light_emission(&self, state: StateId) -> u8 { (self.light_emission)(state) }

    /// Returns the shape of the block.
    #[inline]
    #[must_use]
    pub fn shape_of(&self, state: StateId) -> &'static BlockShape<'static> {
        (self.shape_of)(state)
    }
}
