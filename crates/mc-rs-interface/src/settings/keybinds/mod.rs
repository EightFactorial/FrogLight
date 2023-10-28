use serde::{Deserialize, Serialize};

pub mod movement;
use movement::MovementKeybinds;

pub mod gameplay;
use gameplay::GameplayKeybinds;

pub mod inventory;
use inventory::InventoryKeybinds;

// TODO: Add the ability to set keybinds to keyboard or mouse buttons
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Keybinds {
    pub movement: MovementKeybinds,
    pub gameplay: GameplayKeybinds,
    pub inventory: InventoryKeybinds,
}
