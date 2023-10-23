use belly::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::menus::MenuRoot;

/// Cube map backgrounds for the main menu
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Display, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum BackgroundCubeMapEnum {
    #[default]
    Plains,
    Village,
    Desert,
    DesertVillage,
    Ocean,
    WarmOcean,
    Mountains,
    Cave,
    Cavern,
}

impl BackgroundCubeMapEnum {
    #[allow(dead_code)]
    pub(super) fn create(
        &self,
        _root: &MenuRoot,
        _assets: &AssetServer,
        mut _elements: Elements,
        mut _commands: Commands,
    ) {
        // TODO: Add backgrounds
    }
}
