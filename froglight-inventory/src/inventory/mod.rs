//! TODO

mod inventory;
pub use inventory::Inventory;

mod plugin;
pub use plugin::{InventoryPluginType, InventoryPlugins, InventoryResult};

mod reflect;
pub use reflect::ReflectInventory;
