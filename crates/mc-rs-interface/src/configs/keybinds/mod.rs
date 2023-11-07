use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use crate::traits::config::{ConfigFile, ResourceConfig};

pub mod button;
pub mod keybind;

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
