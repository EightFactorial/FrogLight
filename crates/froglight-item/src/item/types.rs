use core::marker::PhantomData;

#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use downcast_rs::Downcast;
use froglight_common::{prelude::Identifier, version::Version};
use froglight_nbt::nbt::UnnamedNbt;

use super::{ItemRarity, ItemType, ItemTypeExt};
use crate::{resolve::ItemResolver, storage::ItemWrapper};

/// An item with optional data.
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(no_field_bounds, from_reflect = false, PartialEq))]
pub struct Item<I: ItemTypeExt<V>, V: Version> {
    data: UnnamedNbt,
    #[cfg_attr(feature = "reflect", reflect(ignore))]
    _phantom: PhantomData<(I, V)>,
}

impl<I: ItemTypeExt<V>, V: Version> Item<I, V> {
    /// Create a new [`Item`] from the given [`UnnamedNbt`].
    #[inline]
    #[must_use]
    pub(crate) const fn new(data: UnnamedNbt) -> Self { Self { data, _phantom: PhantomData } }

    /// Get the default data of the [`UntypedItem`].
    ///
    /// ```rust
    /// use froglight_item::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let _data = Item::<item::Air, V1_21_4>::const_default_nbt();
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn const_default_nbt() -> UnnamedNbt { <I as ItemTypeExt<V>>::default_nbt() }

    /// Get the identifier of the [`Item`].
    ///
    /// ```rust
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
    /// ```rust
    /// use froglight_item::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     assert_eq!(Item::<item::Air, V1_21_4>::const_rarity(), ItemRarity::Common);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub const fn const_rarity() -> ItemRarity { I::RARITY }

    /// Get the default data of the [`Item`].
    ///
    /// Matches [`UntypedItem::default_nbt`] for consistency.
    ///
    /// If you need access without a `self` reference, see
    /// [`Item::const_default_nbt`] or [`ItemTypeExt::default_nbt`].
    ///
    /// ```rust
    /// use froglight_item::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let item = Item::<item::Air, V1_21_4>::default();
    ///     let _data = item.default_nbt();
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn default_nbt(&self) -> UnnamedNbt { <I as ItemTypeExt<V>>::default_nbt() }

    /// Get the internal [`UnnamedNbt`] of the [`Item`].
    ///
    /// ```rust
    /// use froglight_item::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let item = Item::<item::Air, V1_21_4>::default();
    ///     assert_eq!(item.raw_data().is_empty(), true);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub const fn raw_data(&self) -> &UnnamedNbt { &self.data }

    /// Get the internal [`UnnamedNbt`] of the [`Item`] mutably.
    ///
    /// ```rust
    /// use froglight_item::prelude::*;
    /// use froglight_nbt::nbt::UnnamedNbt;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let mut item = Item::<item::Air, V1_21_4>::default();
    ///     *item.raw_data_mut() = UnnamedNbt::new_empty();
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub const fn raw_data_mut(&mut self) -> &mut UnnamedNbt { &mut self.data }

    /// Convert the [`Item`] into an [`UntypedItem`].
    ///
    /// ```rust
    /// use froglight_item::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let mut items = Vec::with_capacity(4);
    ///     items.push(Item::<item::Air, V1_21_4>::default().into_untyped());
    ///     items.push(Item::<item::Apple, V1_21_4>::default().into_untyped());
    ///     items.push(Item::<item::BakedPotato, V1_21_4>::default().into_untyped());
    ///     items.push(Item::<item::OminousBottle, V1_21_4>::default().into_untyped());
    ///
    ///     assert_eq!(items.len(), 4);
    ///     assert_eq!(items[0].identifier(), "minecraft:air");
    ///     assert_eq!(items[1].identifier(), "minecraft:apple");
    ///     assert_eq!(items[2].identifier(), "minecraft:baked_potato");
    ///     assert_eq!(items[3].identifier(), "minecraft:ominous_bottle");
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn into_untyped(self) -> UntypedItem<V> { self.into() }

    /// Get the identifier of the [`Item`].
    ///
    /// Matches [`UntypedItem::identifier`] for consistency.
    ///
    /// If you need `const` access, see
    /// [`Item::const_identifier`] or [`ItemTypeExt::IDENTIFIER`].
    ///
    /// ```rust
    /// use froglight_item::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     assert_eq!(Item::<item::Air, V1_21_4>::default().identifier(), "minecraft:air");
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
    /// ```rust
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

impl<I: ItemTypeExt<V>, V: Version> Default for Item<I, V> {
    fn default() -> Self { Self::new(<I as ItemTypeExt<V>>::default_nbt()) }
}

impl<I: ItemTypeExt<V>, V: Version> TryFrom<UntypedItem<V>> for Item<I, V> {
    type Error = UntypedItem<V>;

    #[inline]
    fn try_from(value: UntypedItem<V>) -> Result<Self, Self::Error> { value.downcast::<I>() }
}

// -------------------------------------------------------------------------------------------------

/// An untyped item with optional data.
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(no_field_bounds, from_reflect = false, PartialEq))]
pub struct UntypedItem<V: Version> {
    data: UnnamedNbt,
    #[cfg_attr(feature = "reflect", reflect(ignore))]
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

    /// Get the default data of the [`UntypedItem`].
    ///
    /// ```rust
    /// use froglight_item::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let item = Item::<item::Air, V1_21_4>::default();
    ///     let _data = item.into_untyped().default_nbt();
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn default_nbt(&self) -> UnnamedNbt { self.wrapper.default_nbt() }

    /// Get the internal [`UnnamedNbt`] of the [`UntypedItem`].
    ///
    /// ```rust
    /// use froglight_item::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let item = Item::<item::Air, V1_21_4>::default().into_untyped();
    ///     assert_eq!(item.raw_data().is_empty(), true);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub const fn raw_data(&self) -> &UnnamedNbt { &self.data }

    /// Get the internal [`UnnamedNbt`] of the [`UntypedItem`] mutably.
    ///
    /// ```rust
    /// use froglight_item::prelude::*;
    /// use froglight_nbt::nbt::UnnamedNbt;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let mut item = Item::<item::Air, V1_21_4>::default().into_untyped();
    ///     *item.raw_data_mut() = UnnamedNbt::new_empty();
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub const fn raw_data_mut(&mut self) -> &mut UnnamedNbt { &mut self.data }

    /// Consume the [`UntypedItem`] and return it's internal [`UnnamedNbt`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> UnnamedNbt { self.data }

    /// Resolve the [`UntypedItem`] into a typed [`Item`].
    ///
    /// Returns the original [`UntypedItem`] if the type could not be resolved.
    #[inline]
    #[expect(clippy::missing_errors_doc)]
    pub fn resolve<R: ItemResolver<V>>(self) -> Result<R::ItemEnum, Self> { R::resolve(self) }

    /// Returns `true` if the [`Item`] is of a [`ItemType`].
    ///
    /// ```rust
    /// use froglight_common::vanilla::Vanilla;
    /// use froglight_item::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let item = Item::<item::Air, V1_21_4>::default();
    ///     assert!(item.into_untyped().is::<item::Air>());
    /// }
    /// ```
    #[must_use]
    pub fn is<I: ItemTypeExt<V>>(&self) -> bool {
        <dyn ItemType<V> as Downcast>::as_any(*self.wrapper).type_id()
            == <dyn ItemType<V> as Downcast>::as_any(I::as_static()).type_id()
    }

    /// Downcast the [`UntypedItem`] into an [`Item`].
    ///
    /// Returns the original [`UntypedItem`] if the type could not be
    /// downcasted.
    ///
    /// ```rust
    /// use froglight_common::vanilla::Vanilla;
    /// use froglight_item::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     let item = Item::<item::Air, V1_21_4>::default();
    ///     assert_eq!(item.clone().into_untyped().downcast::<item::Air>(), Ok(item));
    /// }
    /// ```
    #[expect(clippy::missing_errors_doc)]
    pub fn downcast<I: ItemTypeExt<V>>(self) -> Result<Item<I, V>, Self> {
        if self.is::<I>() { Ok(Item::new(self.data)) } else { Err(self) }
    }

    /// Get the identifier of the [`UntypedItem`].
    ///
    /// ```rust
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
    /// ```rust
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

impl<I: ItemTypeExt<V>, V: Version> From<Item<I, V>> for UntypedItem<V> {
    #[inline]
    fn from(item: Item<I, V>) -> Self {
        UntypedItem::new(item.data, ItemWrapper::new(I::as_static()))
    }
}

// ------------- Manual trait implementations to avoid trait bounds -----------

impl<I: ItemTypeExt<V>, V: Version> core::fmt::Debug for Item<I, V> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

impl<V: Version> core::fmt::Debug for UntypedItem<V> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UntypedItem").field(&self.data).field(&self.wrapper.identifier()).finish()
    }
}
impl<V: Version> Clone for UntypedItem<V> {
    fn clone(&self) -> Self { Self { data: self.data.clone(), wrapper: self.wrapper } }
}
impl<V: Version> PartialEq for UntypedItem<V> {
    fn eq(&self, other: &Self) -> bool { self.data == other.data }
}
