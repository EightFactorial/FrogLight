use serde::{Deserialize, Serialize};

use crate::menus::main_menu::backgrounds::BackgroundEnum;

/// Settings for the game menus.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MenuSettings {
    #[serde(default)]
    pub main_menu: BackgroundEnum,
}
