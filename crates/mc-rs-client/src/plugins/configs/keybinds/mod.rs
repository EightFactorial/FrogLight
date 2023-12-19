use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use super::traits::{ConfigFile, ResourceConfig};

mod button;
pub use button::Button;

mod keybind;
pub use keybind::KeyBind;

pub mod groups;
use groups::{GameplayKeybinds, InventoryKeybinds, MovementKeybinds};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Resource, Serialize, Deserialize)]
pub struct Keybinds {
    #[serde(default)]
    pub movement: MovementKeybinds,
    #[serde(default)]
    pub gameplay: GameplayKeybinds,
    #[serde(default)]
    pub inventory: InventoryKeybinds,
}

impl ResourceConfig for Keybinds {}
impl ConfigFile for Keybinds {
    const FILE_PATH: &'static str = "keybinds.toml";
}
