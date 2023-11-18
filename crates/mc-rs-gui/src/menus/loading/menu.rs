use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

use super::{bar::LoadingBar, logo::LoadingLogo};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct LoadingMenu;

impl MenuComponent for LoadingMenu {
    fn setup(app: &mut App) {
        // TODO: Fade in only Startup
        // TODO: Fade out when completed

        LoadingLogo::setup(app);
        LoadingBar::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building LoadingMenu");

        let node = NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..Default::default()
        };

        // Spawn LoadingMenu
        let entity = world.spawn((LoadingMenu, node)).id();
        world.entity_mut(parent).add_child(entity);

        // Build submenus
        LoadingLogo::build(entity, world);
        LoadingBar::build(entity, world);
    }
}
