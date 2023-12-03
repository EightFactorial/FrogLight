use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

pub mod audio;
pub mod controls;
pub mod resourcepacks;
pub mod video;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MenusNodeComponent;

impl MenuComponent for MenusNodeComponent {
    fn setup(app: &mut App) {
        video::VideoNodeComponent::setup(app);
        controls::ControlsNodeComponent::setup(app);
        resourcepacks::ResourcepacksNodeComponent::setup(app);
        audio::AudioNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building MenusNodeComponent");
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        };
        let entity = world
            .spawn((MenusNodeComponent, node))
            .set_parent(parent)
            .id();
        video::VideoNodeComponent::build(entity, world);
        controls::ControlsNodeComponent::build(entity, world);
        resourcepacks::ResourcepacksNodeComponent::build(entity, world);
        audio::AudioNodeComponent::build(entity, world);
    }
}
