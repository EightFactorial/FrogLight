use core::{
    any::TypeId,
    fmt::{self, Debug},
};

use froglight_common::prelude::Identifier;

use crate::{
    item::{ComponentData, ItemType},
    state::GlobalItemId,
    version::ItemVersion,
};

/// Metadata about an item type.
pub struct ItemMetadata {
    /// The string identifier of the item.
    identifier: Identifier<'static>,
    /// The [`GlobalItemId`] assigned to this item.
    global_id: GlobalItemId,

    /// The default [`ComponentData`] for this item.
    default_data: ComponentData,

    /// The [`TypeId`] of the item type.
    item_ty: TypeId,
    /// The [`TypeId`] of the version type.
    version_ty: TypeId,
}

impl ItemMetadata {
    /// Create a new [`ItemMetadata`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the `global_id` value is correct for the
    /// [`ItemStorage`](crate::storage::ItemStorage) it will be used in.
    #[must_use]
    pub const unsafe fn new<I: ItemType<V>, V: ItemVersion>(
        identifier: Identifier<'static>,
        global_id: GlobalItemId,
        default_data: ComponentData,
    ) -> Self {
        Self {
            identifier,
            global_id,
            default_data,
            item_ty: TypeId::of::<I>(),
            version_ty: TypeId::of::<V>(),
        }
    }

    /// Get the string identifier of this item.
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { &self.identifier }

    /// Get the [`GlobalItemId`] of this item.
    #[inline]
    #[must_use]
    pub const fn global_id(&self) -> GlobalItemId { self.global_id }

    /// Get the default [`ComponentData`] for this item.
    #[inline]
    #[must_use]
    pub const fn default_data(&self) -> &ComponentData { &self.default_data }

    /// Returns `true` if this item is of type `B`.
    #[inline]
    #[must_use]
    pub fn is_item<I: 'static>(&self) -> bool { self.item_ty == TypeId::of::<I>() }

    /// Returns `true` if this item is of version `V`.
    #[inline]
    #[must_use]
    pub fn is_version<V: 'static>(&self) -> bool { self.version_ty == TypeId::of::<V>() }

    /// Get the [`TypeId`] of the item type.
    #[inline]
    #[must_use]
    pub const fn item_ty(&self) -> TypeId { self.item_ty }

    /// Get the [`TypeId`] of the version type.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.version_ty }
}

impl Debug for ItemMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ItemMetadata").field(self.identifier()).finish_non_exhaustive()
    }
}
