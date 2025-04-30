use downcast_rs::DowncastSync;
use froglight_common::{identifier::Identifier, version::Version};
use froglight_nbt::nbt::UnnamedNbt;

use super::ItemRarity;

/// A static item type.
pub trait StaticItem: 'static {
    /// Get a static reference to the item type.
    ///
    /// Useful when working internally with generic items.
    ///
    /// ```rust
    /// use froglight_item::{generated::item::Air, item::StaticItem};
    ///
    /// assert_eq!(Air::as_static(), &Air);
    /// ```
    fn as_static() -> &'static Self;
}

/// An item type.
pub trait ItemType<V: Version>: DowncastSync + MaybeReflect {
    /// The identifier of the item.
    ///
    /// ```rust
    /// use froglight_item::{
    ///     generated::item::Air,
    ///     item::{ItemType, ItemTypeExt, StaticItem},
    /// };
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Accessing the static identifier through the `ItemType` trait.
    ///     assert_eq!(<Air as ItemType<V1_21_4>>::identifier(Air::as_static()), "minecraft:air");
    ///
    ///     // Accessing the constant identifier through the `ItemTypeExt` trait.
    ///     assert_eq!(<Air as ItemTypeExt<V1_21_4>>::IDENTIFIER, "minecraft:air");
    /// }
    ///
    /// #[cfg(feature = "v1_21_5")]
    /// {
    ///     use froglight_common::version::V1_21_5;
    ///
    ///     // Accessing the static identifier through the `ItemType` trait.
    ///     assert_eq!(<Air as ItemType<V1_21_5>>::identifier(Air::as_static()), "minecraft:air");
    ///
    ///     // Accessing the constant identifier through the `ItemTypeExt` trait.
    ///     assert_eq!(<Air as ItemTypeExt<V1_21_5>>::IDENTIFIER, "minecraft:air");
    /// }
    /// ```
    fn identifier(&self) -> &'static Identifier;

    /// The default NBT of the item.
    ///
    /// ```rust
    /// use froglight_item::{
    ///     generated::item::Air,
    ///     item::{ItemType, ItemTypeExt, StaticItem},
    /// };
    /// use froglight_nbt::nbt::UnnamedNbt;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Accessing the default NBT through the `ItemType` trait.
    ///     let _nbt = <Air as ItemType<V1_21_4>>::default_nbt(Air::as_static());
    ///
    ///     // Accessing the default NBT through the `ItemTypeExt` trait.
    ///     let _nbt = <Air as ItemTypeExt<V1_21_4>>::default_nbt();
    /// }
    ///
    /// #[cfg(feature = "v1_21_5")]
    /// {
    ///     use froglight_common::version::V1_21_5;
    ///
    ///     // Accessing the default NBT through the `ItemType` trait.
    ///     let _nbt = <Air as ItemType<V1_21_5>>::default_nbt(Air::as_static());
    ///
    ///     // Accessing the default NBT through the `ItemTypeExt` trait.
    ///     let _nbt = <Air as ItemTypeExt<V1_21_5>>::default_nbt();
    /// }
    /// ```
    fn default_nbt(&self) -> UnnamedNbt;

    /// The rarity of the item.
    ///
    /// ```rust
    /// use froglight_item::{
    ///     generated::item::Air,
    ///     item::{ItemRarity, ItemType, ItemTypeExt, StaticItem},
    /// };
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Accessing the static rarity through the `ItemType` trait.
    ///     assert_eq!(<Air as ItemType<V1_21_4>>::rarity(Air::as_static()), ItemRarity::Common);
    ///
    ///     // Accessing the constant rarity through the `ItemTypeExt` trait.
    ///     assert_eq!(<Air as ItemTypeExt<V1_21_4>>::RARITY, ItemRarity::Common);
    /// }
    ///
    /// #[cfg(feature = "v1_21_5")]
    /// {
    ///     use froglight_common::version::V1_21_5;
    ///
    ///     // Accessing the static rarity through the `ItemType` trait.
    ///     assert_eq!(<Air as ItemType<V1_21_5>>::rarity(Air::as_static()), ItemRarity::Common);
    ///
    ///     // Accessing the constant rarity through the `ItemTypeExt` trait.
    ///     assert_eq!(<Air as ItemTypeExt<V1_21_5>>::RARITY, ItemRarity::Common);
    /// }
    /// ```
    fn rarity(&self) -> ItemRarity;
}

/// An extension of the [`ItemType`] trait.
pub trait ItemTypeExt<V: Version>: ItemType<V> + StaticItem {
    /// The identifier of the item.
    const IDENTIFIER: &'static str;
    /// The rarity of the item.
    const RARITY: ItemRarity;

    /// Get the default NBT of the item.
    fn default_nbt() -> UnnamedNbt;
}

use sealed::MaybeReflect;
mod sealed {
    #[cfg(feature = "reflect")]
    use bevy_reflect::Reflect;

    #[cfg(feature = "reflect")]
    pub trait MaybeReflect: Reflect {}
    #[cfg(feature = "reflect")]
    impl<T: Reflect> MaybeReflect for T {}

    #[cfg(not(feature = "reflect"))]
    pub trait MaybeReflect {}
    #[cfg(not(feature = "reflect"))]
    impl<T> MaybeReflect for T {}
}
