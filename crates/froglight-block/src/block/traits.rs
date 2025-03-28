use downcast_rs::DowncastSync;
use froglight_common::{identifier::Identifier, version::Version};

use crate::storage::BlockAttributes;

/// A static block type.
pub trait StaticBlock: 'static {
    /// Get a static reference to the block type.
    ///
    /// Useful when working internally with generic blocks.
    ///
    /// ```rust
    /// use froglight_block::{block::StaticBlock, generated::block::Air};
    ///
    /// assert_eq!(Air::as_static(), &Air);
    /// ```
    fn as_static() -> &'static Self;
}

/// A block type.
pub trait BlockType<V: Version>: DowncastSync + MaybeReflect {
    /// Get the value of an attribute as a string.
    ///
    /// ```rust
    /// use froglight_block::{
    ///     block::{BlockType, BlockTypeExt, StaticBlock},
    ///     generated::block::GrassBlock,
    /// };
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Accessing the attribute through the `BlockType` trait.
    ///     assert_eq!(
    ///         <GrassBlock as BlockType<V1_21_4>>::get_attr_str(GrassBlock::as_static(), 0, "snowy"),
    ///         Some("true")
    ///     );
    ///     assert_eq!(
    ///         <GrassBlock as BlockType<V1_21_4>>::get_attr_str(GrassBlock::as_static(), 1, "snowy"),
    ///         Some("false")
    ///     );
    /// }
    ///
    /// #[cfg(feature = "v1_21_5")]
    /// {
    ///     use froglight_common::version::V1_21_5;
    ///
    ///     // Accessing the attribute through the `BlockType` trait.
    ///     assert_eq!(
    ///         <GrassBlock as BlockType<V1_21_5>>::get_attr_str(GrassBlock::as_static(), 0, "snowy"),
    ///         Some("true")
    ///     );
    ///     assert_eq!(
    ///         <GrassBlock as BlockType<V1_21_5>>::get_attr_str(GrassBlock::as_static(), 1, "snowy"),
    ///         Some("false")
    ///     );
    /// }
    /// ```
    fn get_attr_str(&self, state: u16, attr: &str) -> Option<&'static str>;

    /// The identifier of the block.
    ///
    /// ```rust
    /// use froglight_block::{
    ///     block::{BlockType, BlockTypeExt, StaticBlock},
    ///     generated::block::Air,
    /// };
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Accessing the static identifier through the `BlockType` trait.
    ///     assert_eq!(<Air as BlockType<V1_21_4>>::identifier(Air::as_static()), "minecraft:air");
    ///
    ///     // Accessing the constant identifier through the `BlockTypeExt` trait.
    ///     assert_eq!(<Air as BlockTypeExt<V1_21_4>>::IDENTIFIER, "minecraft:air");
    /// }
    ///
    /// #[cfg(feature = "v1_21_5")]
    /// {
    ///     use froglight_common::version::V1_21_5;
    ///
    ///     // Accessing the static identifier through the `BlockType` trait.
    ///     assert_eq!(<Air as BlockType<V1_21_5>>::identifier(Air::as_static()), "minecraft:air");
    ///
    ///     // Accessing the constant identifier through the `BlockTypeExt` trait.
    ///     assert_eq!(<Air as BlockTypeExt<V1_21_5>>::IDENTIFIER, "minecraft:air");
    /// }
    /// ```
    fn identifier(&self) -> &'static Identifier;

    /// Returns `true` if the block is air.
    ///
    /// ```rust
    /// use froglight_block::{
    ///     block::{BlockType, BlockTypeExt, StaticBlock},
    ///     generated::block::Air,
    /// };
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Accessing the static identifier through the `BlockType` trait.
    ///     assert_eq!(<Air as BlockType<V1_21_4>>::is_air(Air::as_static()), true);
    ///
    ///     // Accessing the constant identifier through the `BlockTypeExt` trait.
    ///     assert_eq!(<Air as BlockTypeExt<V1_21_4>>::IS_AIR, true);
    /// }
    ///
    /// #[cfg(feature = "v1_21_5")]
    /// {
    ///     use froglight_common::version::V1_21_5;
    ///
    ///     // Accessing the static identifier through the `BlockType` trait.
    ///     assert_eq!(<Air as BlockType<V1_21_5>>::is_air(Air::as_static()), true);
    ///
    ///     // Accessing the constant identifier through the `BlockTypeExt` trait.
    ///     assert_eq!(<Air as BlockTypeExt<V1_21_5>>::IS_AIR, true);
    /// }
    /// ```
    fn is_air(&self) -> bool;
}

/// An extension of the [`BlockType`] trait.
pub trait BlockTypeExt<V: Version>: BlockType<V> + StaticBlock {
    /// The attributes of the block.
    type Attributes: BlockAttributes + MaybeReflect;
    /// The names of the block attributes.
    const ATTRIBUTES: &'static [&'static str];
    /// The default state of the block.
    const DEFAULT: u16;

    /// The identifier of the block.
    const IDENTIFIER: &'static str;
    /// Whether the block is air.
    const IS_AIR: bool;
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
