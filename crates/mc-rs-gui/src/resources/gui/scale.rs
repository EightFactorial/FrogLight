use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Resource, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GuiScale(Option<u32>);

impl GuiScale {
    pub const AUTO: GuiScale = GuiScale(None);

    /// Create a new `GuiScale` from the given width and height.
    pub fn new(width: u32, height: u32) -> GuiScale {
        let val = std::cmp::max(1, std::cmp::min(width / 320, height / 240));

        GuiScale(Some(val))
    }
}
