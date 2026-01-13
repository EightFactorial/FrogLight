use bevy_reflect::FromType;

use crate::inventory::{InventoryPluginType, ReflectInventory};

impl<T: InventoryPluginType> FromType<T> for ReflectInventory {
    fn from_type() -> Self { ReflectInventory::from_plugin::<T>() }
}
