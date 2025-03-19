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

    fn access<'request: 'access, 'access>(
        (): (),
        query: <Self::Query as WorldQuery>::Item<'request>,
        _: &Dummy,
    ) -> Self::Result<'access> {
        let (inventory, inventory_menu, equipment) = query;
        PlayerInventoryRef { inventory, inventory_menu, equipment }
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

    fn access<'request: 'access, 'access>(
        (): (),
        query: <Self::Query as WorldQuery>::Item<'request>,
        _: &mut Dummy,
    ) -> Self::Result<'access> {
        let (inventory, inventory_menu, equipment) = query;
        PlayerInventoryRef { inventory, inventory_menu, equipment }
    }
}

// -------------------------------------------------------------------------------------------------

/// A reference to a player's inventory.
///
/// This is used to access the player's inventory, hotbar,
/// cursor item, crafting grid, and equipment.
#[expect(dead_code)]
pub struct PlayerInventoryRef<'request, V: Version, Marker>
where Marker: InventoryMarker<PlayerInventory<V>>
        + InventoryMarker<PlayerInventoryMenu<V>>
        + InventoryMarker<EntityEquipment<V>>
{
    inventory: <Marker as InventoryMarker<PlayerInventory<V>>>::Component<'request>,
    inventory_menu: <Marker as InventoryMarker<PlayerInventoryMenu<V>>>::Component<'request>,
    equipment: <Marker as InventoryMarker<EntityEquipment<V>>>::Component<'request>,
}

// -------------------------------------------------------------------------------------------------

#[test]
#[cfg(feature = "v1_21_4")]
fn player_inventory_menu() {
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

    // Test getting read-only access to the player's inventory menu.
    let _immutable = inventory.get(player, ()).unwrap();

    // Get a `InventoryMut` for accessing player inventory menus.
    let mut state: SystemState<InventoryMut<PlayerInventoryMenu<V1_21_4>>> =
        SystemState::new(&mut world);
    let mut inventory: InventoryMut<PlayerInventoryMenu<V1_21_4>> = state.get_mut(&mut world);

    // Test getting read-only and mutable access to the player's inventory menu.
    let _immutable = inventory.get(player, ()).unwrap();
    let _mutable = inventory.get_mut(player, ()).unwrap();
}
