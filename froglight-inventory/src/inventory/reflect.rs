use foldhash::fast::RandomState;
use froglight_common::prelude::Identifier;
use froglight_item::item::Item;
use indexmap::IndexMap;

use crate::inventory::{Inventory, InventoryPluginType, plugin::InventoryResult};

/// A collection of function pointers for interacting with an
/// [`Inventory`](crate::inventory::Inventory).
#[derive(Debug, Clone)]
#[expect(clippy::type_complexity, reason = "Function definitions")]
pub struct ReflectInventory {
    identifier: Identifier<'static>,
    initialize: fn(&mut Inventory),
    get_slot: fn(&Inventory, usize) -> InventoryResult<usize, Option<Item>>,
    get_slot_all: fn(&Inventory) -> InventoryResult<(), IndexMap<usize, Item, RandomState>>,
    set_slot: fn(&mut Inventory, Option<Item>, usize) -> InventoryResult<(Option<Item>, usize), ()>,
    enable_menu:
        fn(&mut Inventory, Identifier<'static>) -> InventoryResult<Identifier<'static>, ()>,
    disable_menu:
        fn(&mut Inventory, Identifier<'static>) -> InventoryResult<Identifier<'static>, ()>,
    query_menu: fn(&Inventory, Identifier<'static>) -> InventoryResult<Identifier<'static>, bool>,
}

impl ReflectInventory {
    /// Creates a new [`ReflectInventory`] from the given plugin type.
    #[must_use]
    pub fn from_plugin<P: InventoryPluginType>() -> Self {
        Self {
            identifier: P::IDENTIFIER,
            initialize: P::initialize,
            get_slot: P::get_slot,
            get_slot_all: P::get_slot_all,
            set_slot: P::set_slot,
            enable_menu: P::enable_menu,
            disable_menu: P::disable_menu,
            query_menu: P::query_menu,
        }
    }

    /// Get the [`Identifier`] of this inventory plugin.
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { &self.identifier }

    /// Initialize the given [`Inventory`] with this plugin's data.
    #[inline]
    pub fn initialize(&self, inventory: &mut Inventory) { (self.initialize)(inventory); }

    /// Get a specific item slot in the [`Inventory`].
    #[inline]
    #[must_use]
    pub fn get_slot(
        &self,
        inventory: &Inventory,
        slot: usize,
    ) -> InventoryResult<usize, Option<Item>> {
        (self.get_slot)(inventory, slot)
    }

    /// Get all item slots in the [`Inventory`].
    #[inline]
    #[must_use]
    pub fn get_slot_all(
        &self,
        inventory: &Inventory,
    ) -> InventoryResult<(), IndexMap<usize, Item, RandomState>> {
        (self.get_slot_all)(inventory)
    }

    /// Set a specific item slot in the [`Inventory`].
    #[inline]
    pub fn set_slot(
        &self,
        inventory: &mut Inventory,
        item: Option<Item>,
        slot: usize,
    ) -> InventoryResult<(Option<Item>, usize), ()> {
        (self.set_slot)(inventory, item, slot)
    }

    /// Enable a menu in the [`Inventory`].
    #[inline]
    pub fn enable_menu(
        &self,
        inventory: &mut Inventory,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, ()> {
        (self.enable_menu)(inventory, menu)
    }

    /// Disable a menu in the [`Inventory`].
    #[inline]
    pub fn disable_menu(
        &self,
        inventory: &mut Inventory,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, ()> {
        (self.disable_menu)(inventory, menu)
    }

    /// Query the status of a menu in the [`Inventory`].
    #[inline]
    #[must_use]
    pub fn query_menu(
        &self,
        inventory: &Inventory,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, bool> {
        (self.query_menu)(inventory, menu)
    }
}
