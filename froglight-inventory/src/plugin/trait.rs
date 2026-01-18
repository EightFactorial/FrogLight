use foldhash::fast::RandomState;
use froglight_common::prelude::Identifier;
use froglight_item::item::Item;
use indexmap::IndexMap;

use crate::inventory::{InventoryMut, InventoryRef, InventoryResult};

/// A trait implemented by inventory plugins.
pub trait PluginType: 'static {
    /// The identifier of this inventory plugin.
    const IDENTIFIER: Identifier<'static>;

    /// Initialize this plugin within the given [`Inventory`].
    ///
    /// ## Note
    ///
    /// This method will be called once when an inventory is created,
    /// regardless of whether the plugin is enabled or not.
    fn initialize(_: &mut InventoryMut) {}

    /// Get a specific item slot in the [`Inventory`].
    fn get_slot(inventory: &InventoryRef, slot: usize) -> InventoryResult<usize, Option<Item>>;

    /// Set a specific item slot in the [`Inventory`].
    fn set_slot(
        inventory: &mut InventoryMut,
        item: Option<Item>,
        slot: usize,
    ) -> InventoryResult<(Option<Item>, usize), ()>;

    /// Enable a menu in the [`Inventory`].
    fn enable_menu(
        inventory: &mut InventoryMut,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, ()>;

    /// Disable a menu in the [`Inventory`].
    fn disable_menu(
        inventory: &mut InventoryMut,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, ()>;

    /// Query whether a menu is enabled in the [`Inventory`].
    fn query_menu_status(
        inventory: &InventoryRef,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, bool>;

    /// Get all item slots of a menu in the [`Inventory`].
    ///
    /// Returns an empty map if the menu is disabled.
    fn query_menu_slots(
        inventory: &InventoryRef,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, IndexMap<usize, Item, RandomState>>;
}

// -------------------------------------------------------------------------------------------------

/// A collection of function pointers from a [`PluginType`].
#[derive(Debug, Clone)]
#[expect(clippy::type_complexity, reason = "Function definitions")]
pub struct ReflectInventory {
    identifier: Identifier<'static>,
    initialize: fn(&mut InventoryMut),
    get_slot: fn(&InventoryRef, usize) -> InventoryResult<usize, Option<Item>>,
    set_slot:
        fn(&mut InventoryMut, Option<Item>, usize) -> InventoryResult<(Option<Item>, usize), ()>,
    enable_menu:
        fn(&mut InventoryMut, Identifier<'static>) -> InventoryResult<Identifier<'static>, ()>,
    disable_menu:
        fn(&mut InventoryMut, Identifier<'static>) -> InventoryResult<Identifier<'static>, ()>,
    query_menu_status:
        fn(&InventoryRef, Identifier<'static>) -> InventoryResult<Identifier<'static>, bool>,
    query_menu_slots:
        fn(
            &InventoryRef,
            Identifier<'static>,
        ) -> InventoryResult<Identifier<'static>, IndexMap<usize, Item, RandomState>>,
}

impl ReflectInventory {
    /// Create a new [`ReflectInventory`] from the given [`PluginType`].
    #[must_use]
    pub fn from_plugin<P: PluginType>() -> Self {
        Self {
            identifier: P::IDENTIFIER,
            initialize: P::initialize,
            get_slot: P::get_slot,
            set_slot: P::set_slot,
            enable_menu: P::enable_menu,
            disable_menu: P::disable_menu,
            query_menu_status: P::query_menu_status,
            query_menu_slots: P::query_menu_slots,
        }
    }

    /// Get the [`Identifier`] of this inventory plugin.
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { &self.identifier }

    /// Initialize the given [`Inventory`] with this plugin's data.
    #[inline]
    pub fn initialize(&self, inventory: &mut InventoryMut) { (self.initialize)(inventory); }

    /// Get a specific item slot in the [`Inventory`].
    #[inline]
    #[must_use]
    pub fn get_slot(
        &self,
        inventory: &InventoryRef,
        slot: usize,
    ) -> InventoryResult<usize, Option<Item>> {
        (self.get_slot)(inventory, slot)
    }

    /// Set a specific item slot in the [`Inventory`].
    #[inline]
    pub fn set_slot(
        &self,
        inventory: &mut InventoryMut,
        item: Option<Item>,
        slot: usize,
    ) -> InventoryResult<(Option<Item>, usize), ()> {
        (self.set_slot)(inventory, item, slot)
    }

    /// Enable a menu in the [`Inventory`].
    #[inline]
    pub fn enable_menu(
        &self,
        inventory: &mut InventoryMut,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, ()> {
        (self.enable_menu)(inventory, menu)
    }

    /// Disable a menu in the [`Inventory`].
    #[inline]
    pub fn disable_menu(
        &self,
        inventory: &mut InventoryMut,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, ()> {
        (self.disable_menu)(inventory, menu)
    }

    /// Query the status of a menu in the [`Inventory`].
    #[inline]
    #[must_use]
    pub fn query_menu_status(
        &self,
        inventory: &InventoryRef,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, bool> {
        (self.query_menu_status)(inventory, menu)
    }

    /// Query the slots of a menu in the [`Inventory`].
    ///
    /// Returns an empty map if the menu is disabled.
    #[inline]
    #[must_use]
    pub fn query_menu_slots(
        &self,
        inventory: &InventoryRef,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, IndexMap<usize, Item, RandomState>> {
        (self.query_menu_slots)(inventory, menu)
    }
}
