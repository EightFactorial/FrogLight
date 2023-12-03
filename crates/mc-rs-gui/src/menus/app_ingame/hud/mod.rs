use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

pub mod chat;
pub mod crosshair;
pub mod hotbar;
pub mod subtitles;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct HudNodeComponent;

impl MenuComponent for HudNodeComponent {
    fn setup(app: &mut App) {
        hotbar::HotbarNodeComponent::setup(app);
        chat::ChatNodeComponent::setup(app);
        subtitles::SubtitlesNodeComponent::setup(app);
        crosshair::CrosshairNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building HudNodeComponent");
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
            .spawn((HudNodeComponent, node))
            .set_parent(parent)
            .id();
        hotbar::HotbarNodeComponent::build(entity, world);
        chat::ChatNodeComponent::build(entity, world);
        subtitles::SubtitlesNodeComponent::build(entity, world);
        crosshair::CrosshairNodeComponent::build(entity, world);
    }
}
