use downcast_rs::DowncastSync;
use froglight_common::{identifier::Identifier, version::Version};

use super::ItemRarity;

/// A static item type.
pub trait StaticItem: 'static {
    /// Get a static reference to the item type.
    ///
    /// Useful when working internally with generic items.
    ///
    /// ```rust,ignore
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
    /// ```rust,ignore
    /// #[cfg(feature = "v1_21_4")]
    /// use froglight_common::version::V1_21_4;
    /// use froglight_item::{
    ///     generated::item::Air,
    ///     item::{ItemType, ItemTypeExt, StaticItem},
    /// };
    ///
    /// // Accessing the static identifier through the `ItemType` trait.
    /// assert_eq!(Air::as_static().identifier().as_str(), "minecraft:air");
    ///
    /// // Accessing the constant identifier through the `ItemTypeExt` trait.
    /// #[cfg(feature = "v1_21_4")]
    /// assert_eq!(<Air as ItemTypeExt<V1_21_4>>::IDENTIFIER, "minecraft:air");
    /// ```
    fn identifier(&self) -> &'static Identifier;

    /// The rarity of the item.
    ///
    /// ```rust,ignore
    /// #[cfg(feature = "v1_21_4")]
    /// use froglight_common::version::V1_21_4;
    /// use froglight_item::{
    ///    generated::item::Air,
    ///   item::{ItemRarity, ItemType, ItemTypeExt, StaticItem},
    /// };
    ///
    /// // Accessing the static rarity through the `ItemType` trait.
    /// assert_eq!(Air::as_static().rarity(), ItemRarity::Common);
    ///
    /// // Accessing the constant rarity through the `ItemTypeExt` trait.
    /// #[cfg(feature = "v1_21_4")]
    /// assert_eq!(<Air as ItemTypeExt<V1_21_4>>::RARITY, ItemRarity::Common);
    fn rarity(&self) -> ItemRarity;
}

/// An extension of the [`ItemType`] trait.
pub trait ItemTypeExt<V: Version>: ItemType<V> + StaticItem {
    /// The identifier of the item.
    const IDENTIFIER: &'static str;
    /// The rarity of the item.
    const RARITY: ItemRarity;
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
