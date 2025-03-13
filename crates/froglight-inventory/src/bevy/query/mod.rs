//! TODO

use bevy_ecs::query::{QueryData, ReadOnlyQueryData};

pub mod player;

/// A trait for querying an inventory.
pub trait InventoryQuery: 'static {
    /// An identifier for an inventory.
    ///
    /// May be `()` if not required.
    type InventoryId;

    /// The result of accessing an inventory.
    ///
    /// Provides access to the queried inventories.
    type InventoryResult<'query>;

    /// The query data required to access the inventory.
    type WorldQuery: QueryData;

    /// Access the queried inventory.
    fn access(
        query: &mut Self::WorldQuery,
        inventory: Self::InventoryId,
    ) -> Self::InventoryResult<'_>;
}

// -------------------------------------------------------------------------------------------------

/// A read-only [`Query`](bevy_ecs::system::Query) for an entity's inventory.
#[derive(QueryData)]
pub struct Inventory<I: InventoryQuery>
where I::WorldQuery: ReadOnlyQueryData
{
    data: I::WorldQuery,
}

impl<I: InventoryQuery> Inventory<I>
where I::WorldQuery: ReadOnlyQueryData
{
    /// Access the queried inventory.
    pub fn access(&mut self, inventory: I::InventoryId) -> I::InventoryResult<'_> {
        I::access(&mut self.data, inventory)
    }
}

// -------------------------------------------------------------------------------------------------

/// A mutable [`Query`](bevy_ecs::system::Query) for an entity's inventory.
#[derive(QueryData)]
#[query_data(mutable)]
pub struct InventoryMut<I: InventoryQuery> {
    data: I::WorldQuery,
}

impl<I: InventoryQuery> InventoryMut<I> {
    /// Access the queried inventory.
    pub fn access(&mut self, inventory: I::InventoryId) -> I::InventoryResult<'_> {
        I::access(&mut self.data, inventory)
    }
}
