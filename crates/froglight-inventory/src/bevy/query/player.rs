use std::ops::{Index, IndexMut};

use bevy_ecs::{
    query::{QueryFilter, WorldQuery},
    world::{Mut, Ref},
};
use froglight_common::version::Version;

use super::{Dummy, InventoryMarker, InventoryRequest, Mutable, ReadOnly};
use crate::prelude::*;

impl<Filter: QueryFilter + 'static, V: Version> InventoryRequest<Filter, ReadOnly>
    for PlayerInventoryMenu<V>
{
    type Accessor = ();
    type Query = (
        Ref<'static, PlayerInventory<V>>,
        Ref<'static, PlayerInventoryMenu<V>>,
        Ref<'static, EntityEquipment<V>>,
    );
    type Resource = Dummy;
    type Result<'a> = PlayerInventoryRef<'a, V, ReadOnly>;

    fn access<'a>(
        (): (),
        (inventory, menu, equipment): <Self::Query as WorldQuery>::Item<'a>,
        _: &Dummy,
    ) -> Self::Result<'a> {
        PlayerInventoryRef::new(inventory, menu, equipment)
    }
}

impl<Filter: QueryFilter + 'static, V: Version> InventoryRequest<Filter, Mutable>
    for PlayerInventoryMenu<V>
{
    type Accessor = ();
    type Query = (
        Mut<'static, PlayerInventory<V>>,
        Mut<'static, PlayerInventoryMenu<V>>,
        Mut<'static, EntityEquipment<V>>,
    );
    type Resource = Dummy;
    type Result<'a> = PlayerInventoryRef<'a, V, Mutable>;

    fn access<'a>(
        (): (),
        (inventory, menu, equipment): <Self::Query as WorldQuery>::Item<'a>,
        _: &mut Dummy,
    ) -> Self::Result<'a> {
        PlayerInventoryRef::new(inventory, menu, equipment)
    }
}

// -------------------------------------------------------------------------------------------------

/// A reference to a player's inventory.
///
/// This is used to access the player's inventory, hotbar,
/// cursor item, crafting grid, and equipment.
pub struct PlayerInventoryRef<'a, V: Version, Marker>
where Marker: InventoryMarker<PlayerInventory<V>>
        + InventoryMarker<PlayerInventoryMenu<V>>
        + InventoryMarker<EntityEquipment<V>>
{
    inventory: <Marker as InventoryMarker<PlayerInventory<V>>>::Component<'a>,
    menu: <Marker as InventoryMarker<PlayerInventoryMenu<V>>>::Component<'a>,
    equipment: <Marker as InventoryMarker<EntityEquipment<V>>>::Component<'a>,
}

impl<'a, V: Version, Marker> PlayerInventoryRef<'a, V, Marker>
where Marker: InventoryMarker<PlayerInventory<V>>
        + InventoryMarker<PlayerInventoryMenu<V>>
        + InventoryMarker<EntityEquipment<V>>
{
    /// Create a new [`PlayerInventoryRef`].
    #[must_use]
    pub const fn new(
        inventory: <Marker as InventoryMarker<PlayerInventory<V>>>::Component<'a>,
        menu: <Marker as InventoryMarker<PlayerInventoryMenu<V>>>::Component<'a>,
        equipment: <Marker as InventoryMarker<EntityEquipment<V>>>::Component<'a>,
    ) -> Self {
        Self { inventory, menu, equipment }
    }

    /// Access the player's [`PlayerInventory`].
    #[must_use]
    pub fn inventory(&self) -> &PlayerInventory<V> { self.inventory.as_ref() }

    /// Access the player's [`PlayerInventoryMenu`].
    #[must_use]
    pub fn menu(&self) -> &PlayerInventoryMenu<V> { self.menu.as_ref() }

    /// Access the player's [`EntityEquipment`].
    #[must_use]
    pub fn equipment(&self) -> &EntityEquipment<V> { self.equipment.as_ref() }

    /// Get an [`InventorySlot`] from the player's inventory.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&InventorySlot<V>> {
        match index {
            0 => Some(self.menu().crafting_result()),
            index @ 1..=4 => Some(&self.menu().crafting()[index - 1]),
            index @ 5..=8 => Some(&self.equipment()[index - 5]),
            index @ 9..=35 => Some(&self.inventory().inventory()[index - 9]),
            index @ 36..=44 => Some(&self.inventory().hotbar()[index - 36]),
            45 => Some(self.menu().offhand()),
            _ => None,
        }
    }
}

impl<V: Version> PlayerInventoryRef<'_, V, Mutable> {
    /// Access the player's [`PlayerInventory`] mutably.
    #[must_use]
    pub fn inventory_mut(&mut self) -> &mut PlayerInventory<V> { self.inventory.as_mut() }

    /// Access the player's [`PlayerInventoryMenu`] mutably.
    #[must_use]
    pub fn menu_mut(&mut self) -> &mut PlayerInventoryMenu<V> { self.menu.as_mut() }

    /// Access the player's [`EntityEquipment`] mutably.
    #[must_use]
    pub fn equipment_mut(&mut self) -> &mut EntityEquipment<V> { self.equipment.as_mut() }

    /// Get a mutable [`InventorySlot`] from the player's inventory.
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut InventorySlot<V>> {
        match index {
            0 => Some(self.menu_mut().crafting_result_mut()),
            index @ 1..=4 => Some(&mut self.menu_mut().crafting_mut()[index - 1]),
            index @ 5..=8 => Some(&mut self.equipment_mut()[index - 5]),
            index @ 9..=35 => Some(&mut self.inventory_mut().inventory_mut()[index - 9]),
            index @ 36..=44 => Some(&mut self.inventory_mut().hotbar_mut()[index - 36]),
            45 => Some(self.menu_mut().offhand_mut()),
            _ => None,
        }
    }
}

impl<V: Version, Marker> Index<usize> for PlayerInventoryRef<'_, V, Marker>
where Marker: InventoryMarker<PlayerInventory<V>>
        + InventoryMarker<PlayerInventoryMenu<V>>
        + InventoryMarker<EntityEquipment<V>>
{
    type Output = InventorySlot<V>;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("`PlayerInventoryRef` index out of bounds.")
    }
}
impl<V: Version> IndexMut<usize> for PlayerInventoryRef<'_, V, Mutable> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("`PlayerInventoryRef` index out of bounds.")
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
#[cfg(feature = "v1_21_4")]
fn immutable() {
    use bevy_ecs::{prelude::*, system::SystemState};
    use froglight_common::version::V1_21_4;

    /// Example system that uses `PlayerInventoryMenu`.
    #[expect(dead_code)]
    fn example(_inv: Inventory<PlayerInventoryMenu<V1_21_4>>) {
        // Whatever you want to do with inventories.
    }

    /// Example system that uses a filter with `PlayerInventoryMenu`.
    #[expect(dead_code)]
    fn example_filter(_inv: Inventory<PlayerInventoryMenu<V1_21_4>, With<Observer>>) {
        // Whatever you want to do with inventories.
    }

    let mut world = World::new();
    world.init_resource::<Dummy>();

    // Spawn a player with an inventory.
    let player = world.spawn(PlayerInventory::<V1_21_4>::new()).id();

    // Get a `Inventory` for accessing player inventory menus.
    let mut state: SystemState<Inventory<PlayerInventoryMenu<V1_21_4>>> =
        SystemState::new(&mut world);
    let inventory: Inventory<PlayerInventoryMenu<V1_21_4>> = state.get(&world);

    // Test getting immutable access to the player's inventory menu.
    let immutable = inventory.get(player, ()).unwrap();
    (0..45).for_each(|i| assert!(immutable.get(i).is_some_and(|slot| slot.is_empty())));
}

#[test]
#[cfg(feature = "v1_21_4")]
fn mutable() {
    use bevy_ecs::{prelude::*, system::SystemState};
    use froglight_common::version::V1_21_4;

    /// Example system that uses `PlayerInventoryMenu`.
    #[expect(dead_code)]
    fn example(_inv: InventoryMut<PlayerInventoryMenu<V1_21_4>>) {
        // Whatever you want to do with inventories.
    }

    /// Example system that uses a filter with `PlayerInventoryMenu`.
    #[expect(dead_code)]
    fn example_filter(_inv: InventoryMut<PlayerInventoryMenu<V1_21_4>, With<Observer>>) {
        // Whatever you want to do with inventories.
    }

    let mut world = World::new();
    world.init_resource::<Dummy>();

    // Spawn a player with an inventory.
    let player = world.spawn(PlayerInventory::<V1_21_4>::new()).id();

    // Get a `InventoryMut` for accessing player inventory menus.
    let mut state: SystemState<InventoryMut<PlayerInventoryMenu<V1_21_4>>> =
        SystemState::new(&mut world);
    let mut inventory: InventoryMut<PlayerInventoryMenu<V1_21_4>> = state.get_mut(&mut world);

    // Test getting mutable and immutable access to the player's inventory menu.
    let mut mutable = inventory.get_mut(player, ()).unwrap();
    (0..45).for_each(|i| assert!(mutable.get_mut(i).is_some_and(|slot| slot.is_empty())));

    let immutable = inventory.get(player, ()).unwrap();
    (0..45).for_each(|i| assert!(immutable.get(i).is_some_and(|slot| slot.is_empty())));
}
