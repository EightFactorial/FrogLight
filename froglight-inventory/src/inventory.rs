//! TODO

use alloc::boxed::Box;
#[cfg(not(feature = "bevy"))]
use core::any::Any;
use core::{any::TypeId, fmt::Debug};

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
pub struct Inventory<G: PluginGroup = GlobalPlugins> {
    plugin_group: G,
    plugin_data: IndexMap<TypeId, Box<dyn MaybeReflect>, RandomState>,
}

impl<G: PluginGroup> Inventory<G> {
    /// Create a new [`Inventory`].
    #[must_use]
    pub fn new() -> Self
    where
        G: Default,
    {
        Self::new_from(G::default())
    }

    /// Create a new [`Inventory`] from the given [`PluginGroup`].
    #[must_use]
    pub fn new_from(plugin_group: G) -> Self {
        let mut inventory =
            Self { plugin_group, plugin_data: IndexMap::with_hasher(RandomState::default()) };

        {
            let plugin_group = &inventory.plugin_group;
            let mut inv_mut = InventoryMut {
                plugin_group: IterFn::Owned(Box::new(|| {
                    Box::new(inventory.plugin_group.iter_plugins())
                })),
                plugin_data: &mut inventory.plugin_data,
            };
            plugin_group.iter_plugins().for_each(|plugin| plugin.initialize(&mut inv_mut));
        }

        inventory
    }

    /// Get the [`Item`] in the specified slot.
    ///
    /// Returns `None` if the slot is empty or does not exist.
    #[must_use]
    pub fn get_slot(&self, slot: usize) -> Option<Item> { InventoryRef::new(self).get_slot(slot) }

    /// Set the [`Item`] in the specified slot.
    ///
    /// Returns `true` if the item was set successfully, `false` otherwise.
    pub fn set_slot(&mut self, item: Option<Item>, slot: usize) -> bool {
        InventoryMut::new(self).set_slot(item, slot)
    }

    /// Enable a menu within the [`Inventory`].
    ///
    /// Returns `true` if the menu was enabled successfully, `false` otherwise.
    pub fn enable_menu(&mut self, menu: Identifier<'static>) -> bool {
        InventoryMut::new(self).enable_menu(menu)
    }

    /// Disable a menu within the [`Inventory`].
    ///
    /// Returns `true` if the menu was disabled successfully, `false` otherwise.
    pub fn disable_menu(&mut self, menu: Identifier<'static>) -> bool {
        InventoryMut::new(self).disable_menu(menu)
    }

    /// Query the status of a menu within the [`Inventory`].
    ///
    /// Returns `Some(true)` if the menu is enabled, `Some(false)` if disabled,
    /// or `None` if the menu does not exist.
    #[must_use]
    pub fn query_menu_status(&self, menu: Identifier<'static>) -> Option<bool> {
        InventoryRef::new(self).query_menu_status(menu)
    }

    /// Query the slots of a menu within the [`Inventory`].
    ///
    /// Returns `None` if the menu does not exist.
    #[must_use]
    pub fn query_menu_slots(
        &self,
        menu: Identifier<'static>,
    ) -> Option<IndexMap<usize, Item, RandomState>> {
        InventoryRef::new(self).query_menu_slots(menu)
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
impl<G: PluginGroup + Clone> Clone for Inventory<G> {
    fn clone(&self) -> Self {
        let mut plugin_data =
            IndexMap::with_capacity_and_hasher(self.plugin_data.len(), RandomState::default());

        for (key, value) in &self.plugin_data {
            match value.reflect_clone() {
                Ok(cloned) => plugin_data.insert(*key, cloned),
                Err(err) => panic!("Attempted to clone plugin data, {err}"),
            };
        }

        Self { plugin_group: self.plugin_group.clone(), plugin_data }
    }
}

impl<G: PluginGroup + Default> Default for Inventory<G> {
    #[inline]
    fn default() -> Self { Self::new() }
}

// -------------------------------------------------------------------------------------------------

/// A reference to an [`Inventory`].
pub struct InventoryRef<'inv, 'iter> {
    plugin_group: IterFn<'inv, 'iter>,
    plugin_data: &'inv IndexMap<TypeId, Box<dyn MaybeReflect>, RandomState>,
}

/// A mutable reference to an [`Inventory`].
pub struct InventoryMut<'inv, 'iter> {
    plugin_group: IterFn<'inv, 'iter>,
    plugin_data: &'inv mut IndexMap<TypeId, Box<dyn MaybeReflect>, RandomState>,
}

/// An enum representing either a reference to or an owned [`Iterator`]
/// function.
enum IterFn<'inv, 'iter> {
    Ref(&'inv (dyn Fn() -> Box<dyn Iterator<Item = &'iter ReflectInventory> + 'iter> + 'inv)),
    Owned(Box<dyn Fn() -> Box<dyn Iterator<Item = &'iter ReflectInventory> + 'iter> + 'inv>),
}

impl<'iter> IterFn<'_, 'iter> {
    /// Reborrow the [`IterFn`] for a shorter lifetime.
    #[must_use]
    const fn reborrow<'c>(&'c self) -> IterFn<'c, 'iter> {
        match self {
            IterFn::Ref(r) => IterFn::Ref(r),
            IterFn::Owned(o) => IterFn::Ref(o),
        }
    }
}

impl<'inv, 'iter> InventoryRef<'inv, 'iter> {
    /// Create a new [`InventoryRef`] from the given [`Inventory`].
    #[must_use]
    pub fn new<G: PluginGroup>(inventory: &'inv Inventory<G>) -> Self
    where
        'inv: 'iter,
    {
        Self {
            plugin_group: IterFn::Owned(Box::new(|| {
                Box::new(inventory.plugin_group.iter_plugins())
            })),
            plugin_data: &inventory.plugin_data,
        }
    }

    /// Reborrow the [`InventoryRef`] for a shorter lifetime.
    #[must_use]
    pub const fn reborrow<'c>(&'c self) -> InventoryRef<'c, 'iter> {
        InventoryRef { plugin_group: self.plugin_group.reborrow(), plugin_data: self.plugin_data }
    }

    /// Get an iterator over the plugins used by this [`Inventory`].
    pub fn plugins(&self) -> impl Iterator<Item = &'iter ReflectInventory> { (self.plugin_group)() }

    /// Get the [`Item`] in the specified slot.
    ///
    /// Returns `None` if the slot is empty or does not exist.
    #[must_use]
    pub fn get_slot(&self, mut slot: usize) -> Option<Item> {
        for plugin in (self.plugin_group)() {
            match plugin.get_slot(self, slot) {
                InventoryResult::Passthrough(pass) => slot = pass,
                InventoryResult::Complete(result) => return result,
            }
        }
        None
    }

    /// Query the status of a menu within the [`Inventory`].
    ///
    /// Returns `Some(true)` if the menu is enabled, `Some(false)` if disabled,
    /// or `None` if the menu does not exist.
    #[must_use]
    pub fn query_menu_status(&self, mut menu: Identifier<'static>) -> Option<bool> {
        for plugin in (self.plugin_group)() {
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
        for plugin in (self.plugin_group)() {
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
}

impl<'inv, 'iter> InventoryMut<'inv, 'iter> {
    /// Create a new [`InventoryMut`] from the given [`Inventory`].
    #[must_use]
    pub fn new<G: PluginGroup>(inventory: &'inv mut Inventory<G>) -> Self
    where
        'inv: 'iter,
    {
        Self {
            plugin_group: IterFn::Owned(Box::new(|| {
                Box::new(inventory.plugin_group.iter_plugins())
            })),
            plugin_data: &mut inventory.plugin_data,
        }
    }

    /// Reborrow the [`InventoryMut`] for a shorter lifetime.
    #[must_use]
    pub const fn reborrow<'c>(&'c mut self) -> InventoryMut<'c, 'iter> {
        InventoryMut { plugin_group: self.plugin_group.reborrow(), plugin_data: self.plugin_data }
    }

    /// Reborrow the [`InventoryMut`] as an [`InventoryRef`].
    #[must_use]
    pub const fn reborrow_ref<'c>(&'c self) -> InventoryRef<'c, 'iter> {
        InventoryRef { plugin_group: self.plugin_group.reborrow(), plugin_data: self.plugin_data }
    }

    /// Get an iterator over the plugins used by this [`Inventory`].
    pub fn plugins(&self) -> impl Iterator<Item = &'iter ReflectInventory> { (self.plugin_group)() }

    /// Get the [`Item`] in the specified slot.
    ///
    /// Returns `None` if the slot is empty or does not exist.
    #[must_use]
    pub fn get_slot(&self, slot: usize) -> Option<Item> { self.reborrow_ref().get_slot(slot) }

    /// Set the [`Item`] in the specified slot.
    ///
    /// Returns `true` if the item was set successfully, `false` otherwise.
    pub fn set_slot(&mut self, mut item: Option<Item>, mut slot: usize) -> bool {
        for plugin in (self.plugin_group)() {
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
        for plugin in (self.plugin_group)() {
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
        for plugin in (self.plugin_group)() {
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
    pub fn query_menu_status(&self, menu: Identifier<'static>) -> Option<bool> {
        self.reborrow_ref().query_menu_status(menu)
    }

    /// Query the slots of a menu within the [`Inventory`].
    ///
    /// Returns `None` if the menu does not exist.
    #[must_use]
    pub fn query_menu_slots(
        &self,
        menu: Identifier<'static>,
    ) -> Option<IndexMap<usize, Item, RandomState>> {
        self.reborrow_ref().query_menu_slots(menu)
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

impl<'inv, 'iter> core::ops::Deref for IterFn<'inv, 'iter> {
    type Target = dyn Fn() -> Box<dyn Iterator<Item = &'iter ReflectInventory> + 'iter> + 'inv;

    fn deref(&self) -> &Self::Target {
        match self {
            IterFn::Ref(r) => r,
            IterFn::Owned(o) => o,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A group of inventory plugins.
pub trait PluginGroup: MaybeReflect + Clone + Send + Sync {
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
