use core::{
    any::TypeId,
    fmt::{self, Debug},
};
#[cfg(feature = "std")]
use std::sync::OnceLock;

use foldhash::fast::RandomState;
use froglight_common::prelude::Identifier;
use froglight_item::item::Item;
use indexmap::IndexMap;
#[cfg(all(feature = "once_cell", not(feature = "std")))]
use once_cell::sync::OnceCell as OnceLock;

use crate::inventory::{Inventory, ReflectInventory};

/// A global registry of inventory plugins.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InventoryPlugins;

static INSTANCE: OnceLock<IndexMap<TypeId, ReflectInventory, RandomState>> = OnceLock::new();

impl InventoryPlugins {
    /// Get access to a specific inventory plugin by its type.
    ///
    /// Returns `None` if the plugin has not been registered.
    #[must_use]
    pub fn get<T: InventoryPluginType>() -> Option<&'static ReflectInventory> {
        Self::try_get_map().and_then(|map| map.get(&TypeId::of::<T>()))
    }

    /// Get access to the global inventory plugins registry.
    ///
    /// # Panics
    ///
    /// Panics if the inventory plugins have not been initialized.
    #[must_use]
    pub fn get_map() -> &'static IndexMap<TypeId, ReflectInventory, RandomState> {
        Self::try_get_map().expect("InventoryPlugins have not been initialized!")
    }

    /// Try to get access to the global inventory plugins registry.
    ///
    /// Returns `None` if the inventory plugins have not been initialized.
    #[must_use]
    pub fn try_get_map() -> Option<&'static IndexMap<TypeId, ReflectInventory, RandomState>> {
        INSTANCE.get()
    }

    /// Initialize the inventory plugins registry with the given plugins.
    ///
    /// # Panics
    ///
    /// Panics if the inventory plugins have already been initialized.
    pub fn initialize_iter(plugins: impl Iterator<Item = (TypeId, ReflectInventory)>) {
        Self::initialize(plugins.collect());
    }

    /// Initialize the inventory plugins registry with the given plugins.
    ///
    /// # Panics
    ///
    /// Panics if the inventory plugins have already been initialized.
    pub fn initialize(mut plugins: IndexMap<TypeId, ReflectInventory, RandomState>) {
        plugins.sort_unstable_by_key(|_, r| r.identifier().reborrow().into_owned());

        #[cfg(feature = "tracing")]
        for plugin in plugins.values() {
            tracing::debug!(target: "froglight_inventory", "Initializing the \"{}\" InventoryPlugin", plugin.identifier());
        }

        INSTANCE.set(plugins).unwrap_or_else(|input| {
            panic!("InventoryPlugins have already been initialized:\n  Current: {:?}\n->\n  Attempted: {input:?}", Self::get_map());
        });
    }
}

impl Debug for InventoryPlugins {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("InventoryPlugins");
        if let Some(plugins) = Self::try_get_map() {
            for (index, plugin) in plugins.values().enumerate() {
                debug.field(&alloc::string::ToString::to_string(&index), plugin.identifier());
            }
        }
        debug.finish()
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait implemented by inventory plugins.
pub trait InventoryPluginType: 'static {
    /// The identifier of this inventory plugin.
    const IDENTIFIER: Identifier<'static>;

    /// Initialize this plugin within the given [`Inventory`].
    fn initialize(inventory: &mut Inventory);

    /// Get a specific item slot in the [`Inventory`].
    fn get_slot(inventory: &Inventory, slot: usize) -> InventoryResult<usize, Option<Item>>;

    /// Set a specific item slot in the [`Inventory`].
    fn set_slot(
        inventory: &mut Inventory,
        item: Option<Item>,
        slot: usize,
    ) -> InventoryResult<(Option<Item>, usize), ()>;

    /// Enable a menu in the [`Inventory`].
    fn enable_menu(
        inventory: &mut Inventory,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, ()>;

    /// Disable a menu in the [`Inventory`].
    fn disable_menu(
        inventory: &mut Inventory,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, ()>;

    /// Query whether a menu is enabled in the [`Inventory`].
    fn query_menu_status(
        inventory: &Inventory,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, bool>;

    /// Get all item slots of a menu in the [`Inventory`].
    ///
    /// Returns an empty map if the menu is disabled.
    fn query_menu_slots(
        inventory: &Inventory,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, IndexMap<usize, Item, RandomState>>;
}

/// The result of an [`InventoryPluginType`] operation.
#[derive(Debug, Clone)]
pub enum InventoryResult<T, U> {
    /// A query that should be passed to the next plugin.
    Passthrough(T),
    /// A query that completed successfully.
    Complete(U),
}
