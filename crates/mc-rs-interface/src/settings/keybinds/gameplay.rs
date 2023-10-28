use bevy::prelude::MouseButton;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameplayKeybinds {
    attack_destroy: Option<MouseButton>,
    pick_block: Option<MouseButton>,
    use_place: Option<MouseButton>,
}

impl Default for GameplayKeybinds {
    fn default() -> Self {
        Self {
            attack_destroy: Some(MouseButton::Left),
            pick_block: Some(MouseButton::Middle),
            use_place: Some(MouseButton::Right),
        }
    }
}
