use downcast_rs::DowncastSync;
use froglight_common::{Identifier, Version};

use crate::storage::BlockAttributes;

/// A block type.
pub trait BlockType<V: Version>: DowncastSync + MaybeReflect {
    /// The identifier of the block.
    fn identifier(&self) -> &'static Identifier;
}

/// An extension of the [`BlockType`] trait.
pub trait BlockTypeExt<V: Version>: StaticBlockType + BlockType<V> {
    /// The attributes of the block.
    type Attributes: BlockAttributes + MaybeReflect;
}

/// A static block type.
pub trait StaticBlockType: 'static {
    /// Get the static block type.
    fn as_static() -> &'static Self;
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
