use serde::{Deserialize, Serialize};

use crate::menus::main_menu::backgrounds::BackgroundEnum;

use super::default_u32;

/// Settings for the game menus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuSettings {
    #[serde(default)]
    pub main_menu: BackgroundEnum,
    // #[serde(default)]
    // pub scale: MenuScale
    #[serde(default = "dirt_block")]
    pub block: (u32, u32),
}

impl Default for MenuSettings {
    fn default() -> Self {
        Self {
            main_menu: Default::default(),
            block: dirt_block(),
        }
    }
}

#[inline]
const fn dirt_block() -> (u32, u32) { (default_u32::<0>(), default_u32::<0>()) }
