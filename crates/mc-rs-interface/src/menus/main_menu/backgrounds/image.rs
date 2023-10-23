use belly::prelude::*;
use bevy::{prelude::*, window::WindowResized};
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::{menus::MenuRoot, settings::Settings};

use super::{BackgroundAssets, BackgroundEnum, MainMenuBackground};

/// A marker component for the main menu background image
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct BackgroundImage;

/// Image backgrounds for the main menu
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Display, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
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
    ///
    /// TODO: Fix size of background image when first created
    pub(super) fn create(
        root: Res<MenuRoot>,
        settings: Res<Settings>,
        assets: Res<AssetServer>,
        mut elements: Elements,
        mut commands: Commands,
    ) {
        let BackgroundEnum::Image(bg) = &settings.menu.main_menu else {
            return;
        };

        commands.entity(**root).insert(BackgroundImage);
        commands.entity(**root).insert(MainMenuBackground);

        let image: Handle<Image> = if let BackgroundImageEnum::Path(path) = bg {
            assets.load(path)
        } else {
            let path = format!("textures/gui/title/background/image/{}.png", bg);
            assets.load(path)
        };

        commands.insert_resource(BackgroundAssets(vec![image.clone_untyped()]));
        elements.select(".root").add_child(eml! {
            <div class="main-background">
                <img src=image/>
            </div>
        });
    }

    /// Destroy the main menu background
    pub(super) fn destroy(root: Res<MenuRoot>, mut commands: Commands, mut elements: Elements) {
        commands.entity(**root).remove::<BackgroundImage>();
        commands.entity(**root).remove::<MainMenuBackground>();

        commands.remove_resource::<BackgroundAssets>();
        elements.select(".root div.main-background").remove();
    }

    pub(super) fn on_window_resize(
        assets: Res<BackgroundAssets>,
        images: Res<Assets<Image>>,
        mut events: EventReader<WindowResized>,
        mut query: Query<&mut Style>,
        mut elements: Elements,
    ) {
        let Some(&WindowResized { width, height, .. }) = events.iter().next() else {
            return;
        };

        let Some(handle) = assets.0.first() else {
            return;
        };
        let Some(img) = images.get(&handle.clone().typed()) else {
            return;
        };

        let mut style = Self::get_style(&mut query, &mut elements);
        Self::resize(&mut style, img.size(), width, height);
    }

    /// Resize the background image to always fit the screen
    fn resize(style: &mut Style, img_size: Vec2, width: f32, height: f32) {
        let scale = (width / img_size.x).max(height / img_size.y);
        let size = img_size * scale;

        style.width = Val::Px(size.x);
        style.height = Val::Px(size.y);
        style.left = Val::Px((width - size.x) / 2.0);
        style.top = Val::Px((height - size.y) / 2.0);
    }

    /// Get the style of the background image
    fn get_style<'a>(query: &'a mut Query<&mut Style>, elements: &mut Elements) -> Mut<'a, Style> {
        query
            .get_mut(
                elements
                    .select(".root div.main-background img")
                    .entities()
                    .first()
                    .copied()
                    .expect("No background image entity found!"),
            )
            .expect("Background image entity has no style!")
    }
}
