use bevy::prelude::*;

use super::traits::MenuComponent;

mod bar;

mod logo;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct LoadingMenuRoot;

impl MenuComponent for LoadingMenuRoot {
    fn add_systems(_app: &mut App) {
        // TODO: Add LoadingBar and Logo systems
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building LoadingMenuRoot");

        // Spawn LoadingMenuRoot
        let entity = world.spawn(Self).id();
        world.entity_mut(parent).add_child(entity);

        // TODO: Build LoadingBar and Logo
    }
}
