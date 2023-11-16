use std::fmt::Debug;

use bevy::prelude::*;

pub mod game;

pub mod loading;

pub mod main_menu;
use main_menu::MainMenuRoot;

pub mod settings;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MenuRoot;

#[allow(dead_code)]
impl MenuRoot {
    pub(super) fn add_systems(app: &mut App) {
        // TODO: Add systems

        // Add submenu systems
        MainMenuRoot::add_systems(app);
    }

    pub(super) fn build(world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building {}", std::any::type_name::<Self>());

        // Spawn MenuComponent
        let entity = world.spawn(Self).id();
        world.entity_mut(entity).insert(MenuRoot);

        // Build submenus
        MainMenuRoot::build(entity, world);
    }
}

trait MenuComponent: Debug + Component {
    fn add_systems(app: &mut App);
    fn build(parent: Entity, world: &mut World);
}
