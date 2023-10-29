use bevy::{
    prelude::{App, KeyCode},
    reflect::Reflect,
};
use leafwing_input_manager::{
    prelude::{InputManagerPlugin, InputMap},
    Actionlike,
};
use serde::{Deserialize, Serialize};

pub(super) fn setup(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<InventoryActions>::default());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Actionlike)]
pub enum InventoryActions {
    DropItem,
    HotbarSlot1,
    HotbarSlot2,
    HotbarSlot3,
    HotbarSlot4,
    HotbarSlot5,
    HotbarSlot6,
    HotbarSlot7,
    HotbarSlot8,
    HotbarSlot9,
    ToggleInventory,
    SwapItemInHand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryKeybinds {
    drop_item: Option<KeyCode>,
    hotbar_slot_1: Option<KeyCode>,
    hotbar_slot_2: Option<KeyCode>,
    hotbar_slot_3: Option<KeyCode>,
    hotbar_slot_4: Option<KeyCode>,
    hotbar_slot_5: Option<KeyCode>,
    hotbar_slot_6: Option<KeyCode>,
    hotbar_slot_7: Option<KeyCode>,
    hotbar_slot_8: Option<KeyCode>,
    hotbar_slot_9: Option<KeyCode>,
    toggle_inventory: Option<KeyCode>,
    swap_item_in_hand: Option<KeyCode>,
}

impl Default for InventoryKeybinds {
    fn default() -> Self {
        Self {
            drop_item: Some(KeyCode::Q),
            hotbar_slot_1: Some(KeyCode::Key1),
            hotbar_slot_2: Some(KeyCode::Key2),
            hotbar_slot_3: Some(KeyCode::Key3),
            hotbar_slot_4: Some(KeyCode::Key4),
            hotbar_slot_5: Some(KeyCode::Key5),
            hotbar_slot_6: Some(KeyCode::Key6),
            hotbar_slot_7: Some(KeyCode::Key7),
            hotbar_slot_8: Some(KeyCode::Key8),
            hotbar_slot_9: Some(KeyCode::Key9),
            toggle_inventory: Some(KeyCode::E),
            swap_item_in_hand: Some(KeyCode::F),
        }
    }
}

impl From<InventoryKeybinds> for InputMap<InventoryActions> {
    fn from(value: InventoryKeybinds) -> Self {
        let mut map = Self::default();

        if let Some(key) = value.drop_item {
            map.insert(key, InventoryActions::DropItem);
        }
        if let Some(key) = value.hotbar_slot_1 {
            map.insert(key, InventoryActions::HotbarSlot1);
        }
        if let Some(key) = value.hotbar_slot_2 {
            map.insert(key, InventoryActions::HotbarSlot2);
        }
        if let Some(key) = value.hotbar_slot_3 {
            map.insert(key, InventoryActions::HotbarSlot3);
        }
        if let Some(key) = value.hotbar_slot_4 {
            map.insert(key, InventoryActions::HotbarSlot4);
        }
        if let Some(key) = value.hotbar_slot_5 {
            map.insert(key, InventoryActions::HotbarSlot5);
        }
        if let Some(key) = value.hotbar_slot_6 {
            map.insert(key, InventoryActions::HotbarSlot6);
        }
        if let Some(key) = value.hotbar_slot_7 {
            map.insert(key, InventoryActions::HotbarSlot7);
        }
        if let Some(key) = value.hotbar_slot_8 {
            map.insert(key, InventoryActions::HotbarSlot8);
        }
        if let Some(key) = value.hotbar_slot_9 {
            map.insert(key, InventoryActions::HotbarSlot9);
        }
        if let Some(key) = value.toggle_inventory {
            map.insert(key, InventoryActions::ToggleInventory);
        }
        if let Some(key) = value.swap_item_in_hand {
            map.insert(key, InventoryActions::SwapItemInHand);
        }

        map
    }
}
