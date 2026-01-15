//! [Plugins](crate::inventory::InventoryPluginType) used by different inventory
//! types.
//!
//! Must be registered by
//! [`InventoryPlugins`](crate::inventory::InventoryPlugins) to enable.

#[cfg(feature = "froglight-entity")]
pub mod entity_equipment;
pub mod player_inventory;
