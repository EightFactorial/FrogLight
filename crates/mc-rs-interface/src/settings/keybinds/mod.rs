use serde::{Deserialize, Serialize};

pub mod movement;
use movement::MovementKeybinds;

pub mod gameplay;
use gameplay::GameplayKeybinds;

pub mod inventory;
use inventory::InventoryKeybinds;

// TODO: Add the ability to set keybinds to keyboard or mouse buttons
// Maybe https://github.com/leafwing-studios/leafwing-input-manager ?
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Keybinds {
    #[serde(default)]
    pub movement: MovementKeybinds,
    #[serde(default)]
    pub gameplay: GameplayKeybinds,
    #[serde(default)]
    pub inventory: InventoryKeybinds,
}
