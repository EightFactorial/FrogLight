use serde::{Deserialize, Serialize};

/// Settings for the game menus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    #[serde(default = "f32_one")]
    pub global_volume: f32,
    #[serde(default = "f32_one")]
    pub music_volume: f32,
    #[serde(default = "f32_one")]
    pub effect_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            global_volume: f32_one(),
            music_volume: f32_one(),
            effect_volume: f32_one(),
        }
    }
}

#[inline]
fn f32_one() -> f32 { 1. }
