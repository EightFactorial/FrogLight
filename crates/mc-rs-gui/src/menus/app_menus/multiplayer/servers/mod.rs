use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

pub mod connection;
pub mod icon;
pub mod ping;
pub mod players;
pub mod status;
pub mod title;
pub mod version;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct ServersNodeComponent;

impl MenuComponent for ServersNodeComponent {
    fn setup(app: &mut App) {
        status::StatusNodeComponent::setup(app);
        icon::IconNodeComponent::setup(app);
        version::VersionNodeComponent::setup(app);
        players::PlayersNodeComponent::setup(app);
        title::TitleNodeComponent::setup(app);
        connection::ConnectionNodeComponent::setup(app);
        ping::PingNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building ServersNodeComponent");
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
            .spawn((ServersNodeComponent, node))
            .set_parent(parent)
            .id();
        status::StatusNodeComponent::build(entity, world);
        icon::IconNodeComponent::build(entity, world);
        version::VersionNodeComponent::build(entity, world);
        players::PlayersNodeComponent::build(entity, world);
        title::TitleNodeComponent::build(entity, world);
        connection::ConnectionNodeComponent::build(entity, world);
        ping::PingNodeComponent::build(entity, world);
    }
}
