use serde::{Deserialize, Serialize};

use super::default_f32;

/// Settings for the game menus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    #[serde(default = "default_f32::<1>")]
    pub global_volume: f32,
    #[serde(default = "default_f32::<1>")]
    pub music_volume: f32,
    #[serde(default = "default_f32::<1>")]
    pub effect_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            global_volume: default_f32::<1>(),
            music_volume: default_f32::<1>(),
            effect_volume: default_f32::<1>(),
        }
    }
}
