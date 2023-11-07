use serde::{Deserialize, Serialize};

use crate::configs::{button::Button, keybind::KeyBind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameplayKeybinds {
    pub attack_destroy: Option<KeyBind>,
    pub pick_block: Option<KeyBind>,
    pub use_place: Option<KeyBind>,
}

impl Default for GameplayKeybinds {
    fn default() -> Self {
        Self {
            attack_destroy: Some(Button::MouseLeft.into()),
            pick_block: Some(Button::MouseMiddle.into()),
            use_place: Some(Button::MouseRight.into()),
        }
    }
}
