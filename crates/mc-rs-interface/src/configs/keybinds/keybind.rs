use serde::{Deserialize, Serialize};

use super::button::Button;

/// A keybind that can consist of one or two buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeyBind {
    Single(Button),
    Double(Button, Button),
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
