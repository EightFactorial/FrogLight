use downcast_rs::Downcast;
use froglight_protocol::traits::Version;

use super::ResolvableAttributes;

/// A block with a specific state.
pub trait BlockState<V: Version>: Downcast + Send + Sync {
    /// The key of the block.
    fn resource_key(&self) -> &'static str;

    /// What material the block is made of.
    fn material(&self) -> &'static str;
    /// Whether the block is breakable.
    fn diggable(&self) -> bool;
    /// The hardness of the block.
    fn hardness(&self) -> f32;
    /// The resistance of the block.
    fn resistance(&self) -> f32;

    /// Whether the block is transparent.
    fn transparent(&self) -> bool;
    /// The light level emitted by the block.
    fn emit_light(&self) -> u8;

    /// The key of the block's bounding box.
    fn bounding_box(&self) -> &'static str;
}

/// An extension of the [`BlockState`] trait.
pub trait BlockStateExt<V: Version>: BlockState<V> + Sized {
    /// The attributes that define this block state.
    ///
    /// Must be either a type or tuple of types
    /// that implement [`BlockAttribute`].
    #[expect(private_bounds)]
    type Attributes: ResolvableAttributes;
    /// The number of states this block can be in.
    const STATE_COUNT: usize = Self::Attributes::STATE_COUNT;

    /// The default block state.
    const DEFAULT: Self;

    /// Create a block state from a relative index.
    #[must_use]
    fn from_relative(relative: u16) -> Option<Self>;

    /// Convert a tuple of attributes into a block state.
    #[must_use]
    fn from_attributes(attributes: Self::Attributes) -> Self {
        if let Some(block) = Self::from_relative(u16::try_from(attributes.to_index()).unwrap()) {
            block
        } else {
            unreachable!("The current largest number of states is roughly 1300")
        }
    }

    /// Convert a block state into a relative index.
    #[must_use]
    fn to_relative(&self) -> u16;

    /// Convert a block state into a tuple of attributes.
    #[must_use]
    fn to_attributes(&self) -> Self::Attributes {
        Self::Attributes::from_index(usize::from(self.to_relative())).unwrap()
    }
}
