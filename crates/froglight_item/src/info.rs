//! TODO

#[cfg(feature = "nightly")]
use core::any::TypeId;
use core::{
    fmt::Debug,
    ops::{Deref, Index},
    sync::atomic::{AtomicU32, Ordering},
};

use froglight_common::{identifier::Identifier, version::Version};

use crate::item::ItemType;

/// Information about a [`ItemType`].
#[derive(Debug)]
pub struct ItemInfo {
    /// The item's identifier.
    identifier: Identifier,
    /// The item's default components.
    components: ItemComponentMap,

    /// The [`TypeId`] of the item, if available.
    #[cfg(feature = "nightly")]
    item_type: TypeId,
    /// The [`TypeId`] of the version, if available.
    #[cfg(feature = "nightly")]
    version_type: TypeId,

    /// The base registered id of the item.
    registered_id: AtomicU32,
}

impl ItemInfo {
    /// Create a new [`ItemInfo`] for a [`ItemType`].
    #[must_use]
    pub const fn new<I: ItemType<V>, V: Version>(
        identifier: &'static str,
        components: ItemComponentMap,
    ) -> Self {
        Self {
            identifier: Identifier::new_static(identifier),
            components,

            #[cfg(feature = "nightly")]
            item_type: TypeId::of::<I>(),
            #[cfg(feature = "nightly")]
            version_type: TypeId::of::<V>(),

            registered_id: AtomicU32::new(u32::MAX),
        }
    }

    /// Get the item's [`Identifier`].
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier { &self.identifier }

    /// Get the item's default [`ItemComponent`]s.
    #[inline]
    #[must_use]
    pub const fn default_components(&self) -> &ItemComponentMap { &self.components }

    // /// Get the item's [`ItemComponents`].
    // #[inline]
    // #[must_use]
    // pub const fn components(&self) -> &ItemComponentMap { &self.components }

    /// Returns `true` if this item is of the given type.
    #[inline]
    #[must_use]
    #[cfg(feature = "nightly")]
    pub fn is_item<I: ItemType<V>, V: Version>(&self) -> bool {
        TypeId::of::<I>() == self.item_type && TypeId::of::<V>() == self.version_type
    }

    /// Get the registered id of the item.
    ///
    /// # Panics
    ///
    /// Panics if the [`ItemInfo`] has not been registered yet.
    #[must_use]
    pub fn base_id(&self) -> u32 {
        let base = self.registered_id.load(Ordering::Relaxed);
        if base == u32::MAX {
            panic!("ItemInfo has not been registered yet!");
        } else {
            base
        }
    }

    /// Set the item's registered id.
    ///
    /// # Panics
    ///
    /// Panics if the [`ItemInfo`] already has a registered id.
    pub(super) fn set_registered_id(&self, item_id: u32) {
        if self.registered_id.load(Ordering::Relaxed) == u32::MAX {
            self.registered_id.store(item_id, Ordering::Relaxed);
        } else {
            panic!("ItemInfo already has a registered id!");
        }
    }
}

impl PartialEq for ItemInfo {
    #[cfg(feature = "nightly")]
    fn eq(&self, other: &Self) -> bool {
        self.item_type == other.item_type && self.version_type == other.version_type
    }

    #[cfg(not(feature = "nightly"))]
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.base_id() == other.base_id()
    }
}
impl Eq for ItemInfo {}

// -------------------------------------------------------------------------------------------------

/// A item component object.
///
/// Used to define item components in a static way.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemComponent {
    /// A map of strings to component objects
    Map(ItemComponentMap),
    /// A list of component objects
    List(&'static [ItemComponent]),
    /// A string value
    String(&'static str),
    /// An integer value
    Integer(i64),
    /// A float value
    Float(f64),
}

impl ItemComponent {
    /// If the values is a [`ItemComponent::Map`], returns the map.
    #[must_use]
    pub const fn as_map(&self) -> Option<&ItemComponentMap> {
        if let ItemComponent::Map(map) = self { Some(map) } else { None }
    }

    /// If the value is a [`ItemComponent::Map`],
    /// use the string key and get the corresponding value.
    #[must_use]
    pub fn get_key(&self, key: &str) -> Option<&ItemComponent> {
        self.as_map().and_then(|map| map.get(key))
    }

    /// If the value is a [`ItemComponent::List`], returns the list.
    #[must_use]
    pub const fn as_list(&self) -> Option<&'static [ItemComponent]> {
        if let ItemComponent::List(list) = self { Some(list) } else { None }
    }

    /// If the value is a [`ItemComponent::List`],
    /// use the index to get the corresponding value.
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<&ItemComponent> {
        self.as_list().and_then(|list| list.get(index))
    }

    /// If the value is a [`ItemComponent::String`], returns the string.
    #[must_use]
    pub const fn as_string(&self) -> Option<&'static str> {
        if let ItemComponent::String(value) = self { Some(value) } else { None }
    }

    /// If the value is a [`ItemComponent::Integer`], returns the integer.
    #[must_use]
    pub const fn as_integer(&self) -> Option<i64> {
        if let ItemComponent::Integer(value) = self { Some(*value) } else { None }
    }

    /// If the value is a [`ItemComponent::Float`], returns the float.
    #[must_use]
    pub const fn as_float(&self) -> Option<f64> {
        if let ItemComponent::Float(value) = self { Some(*value) } else { None }
    }
}

impl<'a> Index<&'a str> for &ItemComponent {
    type Output = ItemComponent;

    #[inline]
    fn index(&self, index: &'a str) -> &Self::Output {
        <ItemComponent as Index<&'a str>>::index(self, index)
    }
}
impl<'a> Index<&'a str> for ItemComponent {
    type Output = ItemComponent;

    fn index(&self, index: &'a str) -> &Self::Output {
        if let ItemComponent::Map(map) = self {
            <ItemComponentMap as Index<&'a str>>::index(map, index)
        } else {
            panic!("Cannot string index into non-map item component objects")
        }
    }
}

impl Index<usize> for &ItemComponent {
    type Output = ItemComponent;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        <ItemComponent as Index<usize>>::index(self, index)
    }
}
impl Index<usize> for ItemComponent {
    type Output = ItemComponent;

    fn index(&self, index: usize) -> &Self::Output {
        if let ItemComponent::List(list) = self {
            list.get(index).expect("Item component list index out of bounds")
        } else {
            panic!("Cannot usize index into non-list item component objects")
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A map of block definition objects.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ItemComponentMap(pub &'static [(&'static str, ItemComponent)]);

impl ItemComponentMap {
    /// Use the string key and get the corresponding value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&ItemComponent> {
        self.iter().find_map(|(k, v)| if k == &key { Some(v) } else { None })
    }
}

impl Deref for ItemComponentMap {
    type Target = [(&'static str, ItemComponent)];

    #[inline]
    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'a> Index<&'a str> for &ItemComponentMap {
    type Output = ItemComponent;

    #[inline]
    fn index(&self, index: &'a str) -> &Self::Output {
        <ItemComponentMap as Index<&'a str>>::index(self, index)
    }
}
impl<'a> Index<&'a str> for ItemComponentMap {
    type Output = ItemComponent;

    fn index(&self, index: &'a str) -> &Self::Output {
        if let Some((_, result)) = self.0.iter().find(|(key, _)| *key == index) {
            result
        } else {
            panic!("No component value found for key: \"{index}\"")
        }
    }
}
