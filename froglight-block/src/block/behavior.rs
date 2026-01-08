use crate::{
    block::{BlockType, StateId},
    version::BlockVersion,
};

type BoolFn = fn(StateId) -> bool;
type U8Fn = fn(StateId) -> u8;

/// A collection of functions that define a block's behavior.
#[derive(Clone, Copy)]
#[expect(missing_docs, reason = "The functions have documentation")]
pub struct BlockBehavior {
    pub is_air: BoolFn,
    pub is_solid: BoolFn,
    pub is_liquid: BoolFn,
    pub has_collision: BoolFn,
    pub is_transparent: BoolFn,
    pub has_occlusion: BoolFn,
    pub light_emission: U8Fn,
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
        )
    }

    /// Create a new [`BlockBehavior`] from manually provided functions.
    #[must_use]
    pub const fn new_manual(
        is_air: BoolFn,
        is_solid: BoolFn,
        is_liquid: BoolFn,
        has_collision: BoolFn,
        is_transparent: BoolFn,
        has_occlusion: BoolFn,
        light_emission: U8Fn,
    ) -> Self {
        Self {
            is_air,
            is_solid,
            is_liquid,
            has_collision,
            is_transparent,
            has_occlusion,
            light_emission,
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

    /// Returns `true` if the block is transparent.
    #[inline]
    #[must_use]
    pub fn is_transparent(&self, state: StateId) -> bool { (self.is_transparent)(state) }

    /// Returns `true` if the block has occlusion.
    #[inline]
    #[must_use]
    pub fn has_occlusion(&self, state: StateId) -> bool { (self.has_occlusion)(state) }

    /// Returns the light emission level of the block.
    #[inline]
    #[must_use]
    pub fn light_emission(&self, state: StateId) -> u8 { (self.light_emission)(state) }
}
