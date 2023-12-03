use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

pub mod armor;
pub mod breath;
pub mod health;
pub mod hunger;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct StatusNodeComponent;

impl MenuComponent for StatusNodeComponent {
    fn setup(app: &mut App) {
        breath::BreathNodeComponent::setup(app);
        armor::ArmorNodeComponent::setup(app);
        hunger::HungerNodeComponent::setup(app);
        health::HealthNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building StatusNodeComponent");
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
            .spawn((StatusNodeComponent, node))
            .set_parent(parent)
            .id();
        breath::BreathNodeComponent::build(entity, world);
        armor::ArmorNodeComponent::build(entity, world);
        hunger::HungerNodeComponent::build(entity, world);
        health::HealthNodeComponent::build(entity, world);
    }
}
