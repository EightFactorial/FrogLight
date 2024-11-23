use froglight_protocol::traits::Version;

use crate::{BlockState, BlockStateExt};

/// A trait that provides a set of actions that can be performed on a block.
///
/// Used to resolve ambiguity when working with blocks.
///
/// # Example
/// ```rust
/// use froglight_block::{
///     attribute::SnowyBooleanAttribute, block::GrassBlock, BlockActions, BlockStateExt,
/// };
/// use froglight_protocol::versions::v1_21_0::V1_21_0;
///
/// let snowy = SnowyBooleanAttribute(false);
///
/// // Using `BlockActions` to convert between block states and attributes.
/// let action = GrassBlock::from_attr::<V1_21_0>(SnowyBooleanAttribute(false));
/// assert_eq!(action.to_attr::<V1_21_0>(), snowy);
///
/// // Using `BlockStateExt` directly.
/// let blockstate =
///     <GrassBlock as BlockStateExt<V1_21_0>>::from_attributes(SnowyBooleanAttribute(false));
/// assert_eq!(<GrassBlock as BlockStateExt<V1_21_0>>::to_attributes(&blockstate), snowy);
/// ```
pub trait BlockActions {
    /// Get the block's resource key.
    #[inline]
    #[must_use]
    fn key<V: Version>(&self) -> &'static str
    where
        Self: BlockState<V>,
    {
        self.resource_key()
    }

    /// Convert the block state into a set of attributes.
    #[inline]
    #[must_use]
    fn to_attr<V: Version>(&self) -> <Self as BlockStateExt<V>>::Attributes
    where
        Self: BlockStateExt<V>,
    {
        self.to_attributes()
    }

    /// Convert a set of attributes into a block state.
    #[inline]
    #[must_use]
    fn from_attr<V: Version>(attr: <Self as BlockStateExt<V>>::Attributes) -> Self
    where
        Self: BlockStateExt<V>,
    {
        Self::from_attributes(attr)
    }
}

impl<B> BlockActions for B {}
