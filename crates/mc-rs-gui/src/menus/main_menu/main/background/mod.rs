use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct CubemapBackground;

impl MenuComponent for CubemapBackground {
    fn add_systems(_app: &mut App) {
        // TODO: Add systems
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building CubemapBackground");

        // Spawn MenuComponent
        let entity = world.spawn(Self).id();
        world.entity_mut(parent).add_child(entity);
    }
}
