use serde::{Deserialize, Serialize};

use crate::plugins::configs::keybinds::{Button, KeyBind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MovementKeybinds {
    pub forward: Option<KeyBind>,
    pub backward: Option<KeyBind>,
    pub left: Option<KeyBind>,
    pub right: Option<KeyBind>,
    pub jump: Option<KeyBind>,
    pub sneak: Option<KeyBind>,
    pub sprint: Option<KeyBind>,
}

impl Default for MovementKeybinds {
    fn default() -> Self {
        Self {
            forward: Some(Button::W.into()),
            backward: Some(Button::S.into()),
            left: Some(Button::A.into()),
            right: Some(Button::D.into()),
            jump: Some(Button::Space.into()),
            sneak: Some(Button::ShiftLeft.into()),
            sprint: Some(Button::ControlLeft.into()),
        }
    }
}
