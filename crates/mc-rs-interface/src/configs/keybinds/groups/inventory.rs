use serde::{Deserialize, Serialize};

use crate::configs::keybinds::{Button, KeyBind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryKeybinds {
    pub drop_item: Option<KeyBind>,
    pub hotbar_slot_1: Option<KeyBind>,
    pub hotbar_slot_2: Option<KeyBind>,
    pub hotbar_slot_3: Option<KeyBind>,
    pub hotbar_slot_4: Option<KeyBind>,
    pub hotbar_slot_5: Option<KeyBind>,
    pub hotbar_slot_6: Option<KeyBind>,
    pub hotbar_slot_7: Option<KeyBind>,
    pub hotbar_slot_8: Option<KeyBind>,
    pub hotbar_slot_9: Option<KeyBind>,
    pub toggle_inventory: Option<KeyBind>,
    pub swap_item_in_hand: Option<KeyBind>,
}

impl Default for InventoryKeybinds {
    fn default() -> Self {
        Self {
            drop_item: Some(Button::Q.into()),
            hotbar_slot_1: Some(Button::Key1.into()),
            hotbar_slot_2: Some(Button::Key2.into()),
            hotbar_slot_3: Some(Button::Key3.into()),
            hotbar_slot_4: Some(Button::Key4.into()),
            hotbar_slot_5: Some(Button::Key5.into()),
            hotbar_slot_6: Some(Button::Key6.into()),
            hotbar_slot_7: Some(Button::Key7.into()),
            hotbar_slot_8: Some(Button::Key8.into()),
            hotbar_slot_9: Some(Button::Key9.into()),
            toggle_inventory: Some(Button::E.into()),
            swap_item_in_hand: Some(Button::F.into()),
        }
    }
}
