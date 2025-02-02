use downcast_rs::DowncastSync;
use froglight_common::{Identifier, Version};

use crate::storage::BlockAttributes;

/// A static block type.
pub trait StaticBlock: 'static {
    /// Get the static block type.
    fn as_static() -> &'static Self;
}

/// A block type.
pub trait BlockType<V: Version>: DowncastSync + MaybeReflect {
    /// The identifier of the block.
    fn identifier(&self) -> &'static Identifier;
}

/// An extension of the [`BlockType`] trait.
pub trait BlockTypeExt<V: Version>: BlockType<V> + StaticBlock {
    /// The attributes of the block.
    type Attributes: BlockAttributes + MaybeReflect;
    /// The names of the block attributes.
    const NAMES: &'static [&'static str];
}

use sealed::MaybeReflect;
mod sealed {
    #[cfg(feature = "bevy")]
    use bevy_reflect::Reflect;

    #[cfg(feature = "bevy")]
    pub trait MaybeReflect: Reflect {}
    #[cfg(feature = "bevy")]
    impl<T: Reflect> MaybeReflect for T {}

    #[cfg(not(feature = "bevy"))]
    pub trait MaybeReflect {}
    #[cfg(not(feature = "bevy"))]
    impl<T> MaybeReflect for T {}
}
