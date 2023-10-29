use bevy::{
    prelude::{App, MouseButton},
    reflect::Reflect,
};
use leafwing_input_manager::{
    prelude::{InputManagerPlugin, InputMap},
    Actionlike,
};
use serde::{Deserialize, Serialize};

pub(super) fn setup(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<GameplayActions>::default());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Actionlike)]
pub enum GameplayActions {
    AttackDestroy,
    PickBlock,
    UsePlace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameplayKeybinds {
    pub attack_destroy: Option<MouseButton>,
    pub pick_block: Option<MouseButton>,
    pub use_place: Option<MouseButton>,
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

impl From<GameplayKeybinds> for InputMap<GameplayActions> {
    fn from(value: GameplayKeybinds) -> Self {
        let mut map = Self::default();

        if let Some(key) = value.attack_destroy {
            map.insert(key, GameplayActions::AttackDestroy);
        }
        if let Some(key) = value.pick_block {
            map.insert(key, GameplayActions::PickBlock);
        }
        if let Some(key) = value.use_place {
            map.insert(key, GameplayActions::UsePlace);
        }

        map
    }
}
