use bevy::input::{keyboard::KeyCode, mouse::MouseButton, Input};
use serde::{Deserialize, Serialize};

use super::button::Button;

/// A keybind that can consist of one or two buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeyBind {
    /// A keybind that consists of one button.
    Single(Button),
    /// A keybind that consists of two buttons.
    ///
    /// The first button is the primary button, and the second is the modifier button.
    Double(Button, Button),
}

impl KeyBind {
    pub fn just_pressed(
        &self,
        input_keyboard: &Input<KeyCode>,
        input_mouse: &Input<MouseButton>,
    ) -> bool {
        match self {
            Self::Single(button) => button.just_pressed(input_keyboard, input_mouse),
            Self::Double(button, modifier) => {
                button.just_pressed(input_keyboard, input_mouse)
                    && modifier.pressed(input_keyboard, input_mouse)
            }
        }
    }

    pub fn pressed(
        &self,
        input_keyboard: &Input<KeyCode>,
        input_mouse: &Input<MouseButton>,
    ) -> bool {
        match self {
            Self::Single(button) => button.pressed(input_keyboard, input_mouse),
            Self::Double(button, modifier) => {
                button.pressed(input_keyboard, input_mouse)
                    && modifier.pressed(input_keyboard, input_mouse)
            }
        }
    }
}

impl From<Button> for KeyBind {
    fn from(button: Button) -> Self { Self::Single(button) }
}

impl From<(Button, Button)> for KeyBind {
    fn from(buttons: (Button, Button)) -> Self {
        if buttons.0 == buttons.1 {
            Self::Single(buttons.0)
        } else {
            Self::Double(buttons.0, buttons.1)
        }
    }
}

impl From<[Button; 1]> for KeyBind {
    fn from(buttons: [Button; 1]) -> Self { Self::Single(buttons[0]) }
}

impl From<[Button; 2]> for KeyBind {
    fn from(buttons: [Button; 2]) -> Self {
        if buttons[0] == buttons[1] {
            Self::Single(buttons[0])
        } else {
            Self::Double(buttons[0], buttons[1])
        }
    }
}
