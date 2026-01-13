use core::{
    any::TypeId,
    cmp::Ordering,
    fmt::{self, Debug, Display},
};

use froglight_common::prelude::Identifier;

use crate::{
    item::{ComponentData, ComponentType, GlobalId, ItemMetadata},
    version::ItemVersion,
};

/// An item in the game.
#[derive(Clone)]
pub struct Item {
    data: ComponentData,
    reference: &'static ItemMetadata,
}

impl Item {
    /// Create a new [`Item`] of the given type.
    #[inline]
    #[must_use]
    pub fn new<I: ItemType<V>, V: ItemVersion>() -> Self { Self::new_from(I::METADATA) }

    /// Create a new [`Item`] from the given metadata.
    ///
    /// Uses the item's default data.
    #[inline]
    #[must_use]
    pub fn new_from(metadata: &'static ItemMetadata) -> Self {
        Self::new_from_data(metadata.default_data().clone(), metadata)
    }

    /// Create a new [`Item`] from the given item data and metadata.
    #[inline]
    #[must_use]
    pub fn new_from_data(data: ComponentData, metadata: &'static ItemMetadata) -> Self {
        Item { data, reference: metadata }
    }

    /// Get a component from this item.
    ///
    /// Returns `None` if the component is not present or fails to parse.
    ///
    /// ## Note
    ///
    /// If you are interested in parsing errors, use
    /// [`item.item_data().get::<C>()`](ComponentData::get) instead.
    #[inline]
    #[must_use]
    pub fn get_component<C: ComponentType<V>, V: ItemVersion>(&self) -> Option<C> {
        self.item_data().get::<C, V>().ok().flatten()
    }

    /// Set a component on this item.
    #[inline]
    pub fn set_component<C: ComponentType<V>, V: ItemVersion>(&mut self, component: &C) {
        self.item_data_mut().set::<C, V>(component);
    }

    /// Get a reference to the [`ItemData`] of this item.
    #[inline]
    #[must_use]
    pub const fn item_data(&self) -> &ComponentData { &self.data }

    /// Get a mutable reference to the [`ItemData`] of this item.
    #[inline]
    #[must_use]
    pub const fn item_data_mut(&mut self) -> &mut ComponentData { &mut self.data }

    /// Get the string identifier of this item.
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { self.reference.identifier() }

    /// Get the [`ItemMetadata`] of this item.
    #[inline]
    #[must_use]
    pub const fn metadata(&self) -> &'static ItemMetadata { self.reference }

    /// Get the [`GlobalId`] of this item.
    #[inline]
    #[must_use]
    pub fn global_id(&self) -> GlobalId { self.reference.global_id() }

    /// Returns `true` if this item is of type `B`.
    #[inline]
    #[must_use]
    pub fn is_item<I: 'static>(&self) -> bool { self.reference.is_item::<I>() }

    /// Returns `true` if this item is of version `V`.
    #[inline]
    #[must_use]
    pub fn is_version<V: 'static>(&self) -> bool { self.reference.is_version::<V>() }

    /// Get the [`TypeId`] of the item type.
    #[inline]
    #[must_use]
    pub const fn item_ty(&self) -> TypeId { self.reference.item_ty() }

    /// Get the [`TypeId`] of the version type.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.reference.version_ty() }
}

impl Eq for Item {}
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool { self.reference.global_id() == other.reference.global_id() }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.version_ty() == other.version_ty() {
            Some(self.reference.global_id().cmp(&other.reference.global_id()))
        } else {
            None
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { Display::fmt(self.identifier(), f) }
}

impl Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Item")
            .field(self.reference.identifier())
            .field(&self.global_id().into_inner())
            .field(self)
            .finish_non_exhaustive()
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait implemented by all item types.
pub trait ItemType<V: ItemVersion>: 'static {
    /// The [`ItemMetadata`] for this item type.
    const METADATA: &'static ItemMetadata;
}
