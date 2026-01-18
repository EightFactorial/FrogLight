//! TODO

use alloc::boxed::Box;
use core::{
    any::{Any, TypeId},
    fmt::Debug,
};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
use foldhash::fast::RandomState;
use froglight_common::prelude::Identifier;
use froglight_item::item::Item;
use indexmap::IndexMap;

use crate::plugin::{GlobalPlugins, ReflectInventory};

/// An inventory that can hold items.
///
/// Uses internal plugins to manage slots and menus.
#[cfg_attr(feature = "bevy", derive(Component, Reflect), reflect(opaque, Clone, Component))]
pub struct Inventory {
    plugin_data: IndexMap<TypeId, Box<dyn MaybeReflect>, RandomState>,
}

impl Inventory {
    /// Create a new [`Inventory`].
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Get the [`Item`] in the specified slot.
    ///
    /// Returns `None` if the slot is empty or does not exist.
    #[must_use]
    pub fn get_slot(&self, mut slot: usize) -> Option<Item> {
        for plugin in GlobalPlugins::get_map().values() {
            match plugin.get_slot(self, slot) {
                InventoryResult::Passthrough(pass) => slot = pass,
                InventoryResult::Complete(result) => return result,
            }
        }
        None
    }

    /// Set the [`Item`] in the specified slot.
    ///
    /// Returns `true` if the item was set successfully, `false` otherwise.
    pub fn set_slot(&mut self, mut item: Option<Item>, mut slot: usize) -> bool {
        for plugin in GlobalPlugins::get_map().values() {
            match plugin.set_slot(self, item, slot) {
                InventoryResult::Passthrough((pass_item, pass_slot)) => {
                    item = pass_item;
                    slot = pass_slot;
                }
                InventoryResult::Complete(()) => return true,
            }
        }
        false
    }

    /// Enable a menu within the [`Inventory`].
    ///
    /// Returns `true` if the menu was enabled successfully, `false` otherwise.
    pub fn enable_menu(&mut self, mut menu: Identifier<'static>) -> bool {
        for plugin in GlobalPlugins::get_map().values() {
            match plugin.enable_menu(self, menu) {
                InventoryResult::Passthrough(pass) => menu = pass,
                InventoryResult::Complete(()) => return true,
            }
        }
        false
    }

    /// Disable a menu within the [`Inventory`].
    ///
    /// Returns `true` if the menu was disabled successfully, `false` otherwise.
    pub fn disable_menu(&mut self, mut menu: Identifier<'static>) -> bool {
        for plugin in GlobalPlugins::get_map().values() {
            match plugin.disable_menu(self, menu) {
                InventoryResult::Passthrough(pass) => menu = pass,
                InventoryResult::Complete(()) => return true,
            }
        }
        false
    }

    /// Query the status of a menu within the [`Inventory`].
    ///
    /// Returns `Some(true)` if the menu is enabled, `Some(false)` if disabled,
    /// or `None` if the menu does not exist.
    #[must_use]
    pub fn query_menu_status(&self, mut menu: Identifier<'static>) -> Option<bool> {
        for plugin in GlobalPlugins::get_map().values() {
            match plugin.query_menu_status(self, menu) {
                InventoryResult::Passthrough(pass) => menu = pass,
                InventoryResult::Complete(result) => return Some(result),
            }
        }
        None
    }

    /// Query the slots of a menu within the [`Inventory`].
    ///
    /// Returns `None` if the menu does not exist.
    #[must_use]
    pub fn query_menu_slots(
        &self,
        mut menu: Identifier<'static>,
    ) -> Option<IndexMap<usize, Item, RandomState>> {
        for plugin in GlobalPlugins::get_map().values() {
            match plugin.query_menu_slots(self, menu) {
                InventoryResult::Passthrough(pass) => menu = pass,
                InventoryResult::Complete(result) => return Some(result),
            }
        }
        None
    }

    /// Get a reference to plugin data of type `T` if it exists.
    #[must_use]
    pub fn plugin_data_ref<T: MaybeReflect>(&self) -> Option<&T> {
        self.plugin_data.get(&TypeId::of::<T>()).and_then(|b| b.as_any().downcast_ref::<T>())
    }

    /// Get a mutable reference to plugin data of type `T` if it exists.
    #[must_use]
    pub fn plugin_data_mut<T: MaybeReflect>(&mut self) -> Option<&mut T> {
        self.plugin_data
            .get_mut(&TypeId::of::<T>())
            .and_then(|b| b.as_any_mut().downcast_mut::<T>())
    }

    /// Set plugin data of type `T`.
    ///
    /// Returns any previous data of type `T` if it existed.
    pub fn set_plugin_data<T: MaybeReflect>(&mut self, data: T) -> Option<T> {
        let previous = self.plugin_data.swap_remove(&TypeId::of::<T>());
        self.plugin_data.insert(TypeId::of::<T>(), Box::new(data));
        previous.and_then(|b| b.into_any().downcast::<T>().ok()).map(|b| *b)
    }
}

#[cfg(feature = "bevy")]
impl Debug for Inventory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debug = f.debug_struct("Inventory");
        for (index, value) in self.plugin_data.values().enumerate() {
            debug.field(
                &alloc::string::ToString::to_string(&index),
                &value.reflect_type_ident().unwrap_or("Unknown"),
            );
        }
        debug.finish()
    }
}

#[cfg(feature = "bevy")]
impl Clone for Inventory {
    fn clone(&self) -> Self {
        let mut plugin_data =
            IndexMap::with_capacity_and_hasher(self.plugin_data.len(), RandomState::default());

        for (key, value) in &self.plugin_data {
            match value.reflect_clone() {
                Ok(cloned) => plugin_data.insert(*key, cloned),
                Err(err) => panic!("Attempted to clone plugin data, {err}"),
            };
        }

        Self { plugin_data }
    }
}

impl Default for Inventory {
    #[inline]
    fn default() -> Self {
        let mut inventory = Self { plugin_data: IndexMap::with_hasher(RandomState::default()) };
        GlobalPlugins::get_map().values().for_each(|plugin| plugin.initialize(&mut inventory));
        inventory
    }
}

// -------------------------------------------------------------------------------------------------

/// A group of inventory plugins.
pub trait PluginGroup: Any + Send + Sync {
    /// Create an iterator over the plugins in this group.
    fn iter_plugins(&self) -> impl Iterator<Item = &ReflectInventory>;
}

/// The result of an [`PluginGroup`] operation.
#[derive(Debug, Clone)]
pub enum InventoryResult<T, U> {
    /// A query that should be passed to the next plugin.
    Passthrough(T),
    /// A query that completed successfully.
    Complete(U),
}

// -------------------------------------------------------------------------------------------------

/// A trait for types that may implement Bevy's
/// [`Reflect`](bevy_reflect::Reflect) trait.
#[cfg(feature = "bevy")]
pub use bevy_reflect::Reflect as MaybeReflect;

/// A trait for types that may implement Bevy's
/// [`Reflect`](bevy_reflect::Reflect) trait.
#[cfg(not(feature = "bevy"))]
pub trait MaybeReflect: Any {
    /// Convert a [`Box<Self>`] into a [`Box<dyn Any>`].
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
    /// Convert a [`&Self`] into a [`&dyn Any`].
    fn as_any(&self) -> &dyn Any;
    /// Convert a [`&mut Self`] into a [`&mut dyn Any`].
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[cfg(not(feature = "bevy"))]
impl<T: Any> MaybeReflect for T {
    fn into_any(self: Box<Self>) -> Box<dyn Any> { self }

    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
