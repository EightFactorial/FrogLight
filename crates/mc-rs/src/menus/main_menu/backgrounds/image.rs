use belly::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::menus::MenuRoot;

use super::BackgroundAssets;

/// A marker component for the main menu background image
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct BackgroundImage;

/// Image backgrounds for the main menu
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Display, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum BackgroundImageEnum {
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

    /// A path to a custom background
    Path(String),
}

impl BackgroundImageEnum {
    /// Creates the main menu background
    pub(super) fn create(
        &self,
        root: &MenuRoot,
        assets: &AssetServer,
        mut elements: Elements,
        mut commands: Commands,
    ) {
        commands.entity(**root).insert(BackgroundImage);

        let image: Handle<Image> = if let BackgroundImageEnum::Path(path) = self {
            assets.load(path)
        } else {
            let path = format!("textures/gui/title/background/image/{}.png", self);
            assets.load(path)
        };

        commands.insert_resource(BackgroundAssets(vec![image.clone_untyped()]));
        elements.select(".root").add_child(eml! {
            <div class="main-background">
                <img src=image/>
            </div>
        });
    }
}
