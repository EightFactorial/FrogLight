use bevy::{prelude::App, reflect::Reflect};
use leafwing_input_manager::{
    prelude::{InputManagerPlugin, InputMap, UserInput},
    scan_codes::QwertyScanCode,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MovementKeybinds {
    pub forward: Option<UserInput>,
    pub backward: Option<UserInput>,
    pub left: Option<UserInput>,
    pub right: Option<UserInput>,
    pub jump: Option<UserInput>,
    pub sneak: Option<UserInput>,
    pub sprint: Option<UserInput>,
}

impl Default for MovementKeybinds {
    fn default() -> Self {
        Self {
            forward: Some(QwertyScanCode::W.into()),
            backward: Some(QwertyScanCode::S.into()),
            left: Some(QwertyScanCode::A.into()),
            right: Some(QwertyScanCode::D.into()),
            jump: Some(QwertyScanCode::Space.into()),
            sneak: Some(QwertyScanCode::ShiftLeft.into()),
            sprint: Some(QwertyScanCode::ControlLeft.into()),
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
