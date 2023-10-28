use bevy::prelude::KeyCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MovementKeybinds {
    pub forward: Option<KeyCode>,
    pub backward: Option<KeyCode>,
    pub left: Option<KeyCode>,
    pub right: Option<KeyCode>,
    pub jump: Option<KeyCode>,
    pub sneak: Option<KeyCode>,
    pub sprint: Option<KeyCode>,
}

impl Default for MovementKeybinds {
    fn default() -> Self {
        Self {
            forward: Some(KeyCode::W),
            backward: Some(KeyCode::S),
            left: Some(KeyCode::A),
            right: Some(KeyCode::D),
            jump: Some(KeyCode::Space),
            sneak: Some(KeyCode::ShiftLeft),
            sprint: Some(KeyCode::ControlLeft),
        }
    }
}
