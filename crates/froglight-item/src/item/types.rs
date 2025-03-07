use std::marker::PhantomData;

use froglight_common::{prelude::Identifier, version::Version};
use froglight_nbt::nbt::UnnamedNbt;

use super::{ItemRarity, ItemTypeExt};
use crate::storage::ItemWrapper;

/// An item with optional data.
pub struct Item<I: ItemTypeExt<V>, V: Version> {
    data: UnnamedNbt,
    _phantom: PhantomData<(I, V)>,
}

impl<I: ItemTypeExt<V>, V: Version> Item<I, V> {
    /// Get the identifier of the [`Item`].
    ///
    /// ```rust,ignore
    /// use froglight_item::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     assert_eq!(Item::<item::Air, V1_21_4>::const_identifier(), "minecraft:air");
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub const fn const_identifier() -> &'static str { I::IDENTIFIER }

    /// Get the rarity of the [`Item`].
    ///
    /// ```rust,ignore
    /// use froglight_item::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     assert_eq!(Item::<item::Air, V1_21_4>::default().rarity(), ItemRarity::Common);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn const_rarity(&self) -> ItemRarity { I::RARITY }

    /// Get the identifier of the [`Item`].
    ///
    /// Matches [`UntypedItem::identifier`] for consistency.
    ///
    /// If you need `const` access, see
    /// [`Item::const_identifier`] or [`ItemTypeExt::IDENTIFIER`].
    ///
    /// ```rust,ignore
    /// use froglight_item::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     assert_eq!(Item::<item::Air, V1_21_4>::default().identifier().as_str(), "minecraft:air");
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn identifier(&self) -> &'static Identifier { I::as_static().identifier() }

    /// Get the rarity of the [`Item`].
    ///
    /// Matches [`UntypedItem::rarity`] for consistency.
    ///
    /// If you need `const` access, see
    /// [`Item::const_rarity`] or [`ItemTypeExt::RARITY`].
    ///
    /// ```rust,ignore
    /// use froglight_item::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     assert_eq!(Item::<item::Air, V1_21_4>::default().rarity(), ItemRarity::Common);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn rarity(&self) -> ItemRarity { I::as_static().rarity() }
}

// -------------------------------------------------------------------------------------------------

/// An untyped item with optional data.
pub struct UntypedItem<V: Version> {
    data: UnnamedNbt,
    wrapper: ItemWrapper<V>,
}

impl<V: Version> UntypedItem<V> {
    /// Create a new [`UntypedItem`] from the given
    /// [`UnnamedNbt`] and [`ItemWrapper`].
    #[inline]
    #[must_use]
    pub(crate) const fn new(data: UnnamedNbt, wrapper: ItemWrapper<V>) -> Self {
        Self { data, wrapper }
    }

    /// Get the internal [`ItemWrapper`] of the [`UntypedItem`].
    #[inline]
    #[must_use]
    pub(crate) const fn wrapper(&self) -> &ItemWrapper<V> { &self.wrapper }

    /// Get the identifier of the [`UntypedItem`].
    ///
    /// ```rust,ignore
    /// use froglight_item::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let item = Item::<item::Air, V1_21_4>::default();
    ///     assert_eq!(item.into_untyped().identifier().as_str(), "minecraft:air");
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn identifier(&self) -> &'static Identifier { self.wrapper.identifier() }

    /// Get the rarity of the [`UntypedItem`].
    ///
    /// ```rust,ignore
    /// use froglight_item::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let item = Item::<item::Air, V1_21_4>::default();
    ///     assert_eq!(item.into_untyped().rarity(), ItemRarity::Common);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn rarity(&self) -> ItemRarity { self.wrapper.rarity() }
}

// ------------- Manual trait implementations to avoid trait bounds -----------

impl<I: ItemTypeExt<V>, V: Version> std::fmt::Debug for Item<I, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Item").field(&self.data).field(&Self::const_identifier()).finish()
    }
}
impl<I: ItemTypeExt<V>, V: Version> Clone for Item<I, V> {
    fn clone(&self) -> Self { Self { data: self.data.clone(), _phantom: PhantomData } }
}
impl<I: ItemTypeExt<V>, V: Version> Eq for Item<I, V> {}
impl<I: ItemTypeExt<V>, V: Version> PartialEq for Item<I, V> {
    fn eq(&self, other: &Self) -> bool { self.data == other.data }
}

impl<V: Version> std::fmt::Debug for UntypedItem<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UntypedItem").field(&self.data).field(&self.wrapper.identifier()).finish()
    }
}
impl<V: Version> Clone for UntypedItem<V> {
    fn clone(&self) -> Self { Self { data: self.data.clone(), wrapper: self.wrapper } }
}
impl<V: Version> Eq for UntypedItem<V> {}
impl<V: Version> PartialEq for UntypedItem<V> {
    fn eq(&self, other: &Self) -> bool { self.data == other.data }
}
