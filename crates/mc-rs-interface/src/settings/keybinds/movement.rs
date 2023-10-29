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
    app.add_plugins(InputManagerPlugin::<MovementActions>::default());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Actionlike)]
pub enum MovementActions {
    Forward,
    Backward,
    Left,
    Right,
    Jump,
    Sneak,
    Sprint,
}

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

impl From<MovementKeybinds> for InputMap<MovementActions> {
    fn from(value: MovementKeybinds) -> Self {
        let mut map = Self::default();

        if let Some(key) = value.forward {
            map.insert(key, MovementActions::Forward);
        }
        if let Some(key) = value.backward {
            map.insert(key, MovementActions::Backward);
        }
        if let Some(key) = value.left {
            map.insert(key, MovementActions::Left);
        }
        if let Some(key) = value.right {
            map.insert(key, MovementActions::Right);
        }
        if let Some(key) = value.jump {
            map.insert(key, MovementActions::Jump);
        }
        if let Some(key) = value.sneak {
            map.insert(key, MovementActions::Sneak);
        }
        if let Some(key) = value.sprint {
            map.insert(key, MovementActions::Sprint);
        }

        map
    }
}
