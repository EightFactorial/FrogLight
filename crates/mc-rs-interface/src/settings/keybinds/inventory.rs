use bevy::prelude::KeyCode;
use serde::{Deserialize, Serialize};

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
