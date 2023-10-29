use bevy::{prelude::App, reflect::Reflect};
use leafwing_input_manager::{
    prelude::{InputManagerPlugin, InputMap},
    scan_codes::QwertyScanCode,
    user_input::UserInput,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryKeybinds {
    drop_item: Option<UserInput>,
    hotbar_slot_1: Option<UserInput>,
    hotbar_slot_2: Option<UserInput>,
    hotbar_slot_3: Option<UserInput>,
    hotbar_slot_4: Option<UserInput>,
    hotbar_slot_5: Option<UserInput>,
    hotbar_slot_6: Option<UserInput>,
    hotbar_slot_7: Option<UserInput>,
    hotbar_slot_8: Option<UserInput>,
    hotbar_slot_9: Option<UserInput>,
    toggle_inventory: Option<UserInput>,
    swap_item_in_hand: Option<UserInput>,
}

impl Default for InventoryKeybinds {
    fn default() -> Self {
        Self {
            drop_item: Some(QwertyScanCode::Q.into()),
            hotbar_slot_1: Some(QwertyScanCode::Key1.into()),
            hotbar_slot_2: Some(QwertyScanCode::Key2.into()),
            hotbar_slot_3: Some(QwertyScanCode::Key3.into()),
            hotbar_slot_4: Some(QwertyScanCode::Key4.into()),
            hotbar_slot_5: Some(QwertyScanCode::Key5.into()),
            hotbar_slot_6: Some(QwertyScanCode::Key6.into()),
            hotbar_slot_7: Some(QwertyScanCode::Key7.into()),
            hotbar_slot_8: Some(QwertyScanCode::Key8.into()),
            hotbar_slot_9: Some(QwertyScanCode::Key9.into()),
            toggle_inventory: Some(QwertyScanCode::E.into()),
            swap_item_in_hand: Some(QwertyScanCode::F.into()),
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
