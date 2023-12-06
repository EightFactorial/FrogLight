use bevy::prelude::*;
use compact_str::CompactString;
use mc_rs_core::{events::StatusRequest, versions::v1_20_0::V1_20_0};

use crate::{
    menus::{
        app_menus::states::MainMenuState,
        states::menus::{MenuComponentMenusSet, MenuComponentState},
        traits::MenuComponent,
    },
    resources::{scale::GuiScaleComponent, servers::ServerList},
};

use super::MultiplayerCenterNodeComponent;

/// TODO: Get this Version from somewhere
type DefaultVersion = V1_20_0;

pub mod connection;
pub mod icon;
pub mod ping;
pub mod players;
pub mod status;
pub mod title;
pub mod version;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Component)]
pub struct ServersNodeComponent(CompactString);

impl MenuComponent for ServersNodeComponent {
    fn setup(app: &mut App) {
        app.init_resource::<ShouldRebuildServers>();
        app.add_systems(OnEnter(MainMenuState::Multiplayer), Self::update_statuses);

        app.add_systems(
            Update,
            Self::rebuild
                .run_if(
                    in_state(MenuComponentState::Menus)
                        .and_then(resource_equals(ShouldRebuildServers(true)))
                        .and_then(any_with_component::<MultiplayerCenterNodeComponent>()),
                )
                .in_set(MenuComponentMenusSet),
        );

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

        let outer_node = world
            .spawn((
                GuiScaleComponent::new(180, 180),
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::horizontal(Val::Px(4.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .set_parent(parent)
            .id();

        let server_list = world.resource::<ServerList>().clone();
        for (index, server) in server_list.servers.into_iter().enumerate() {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Building ServerNode: {}", server.title);

            let server_entity = world
                .spawn((
                    ServerListIndex(index),
                    ServersNodeComponent(server.address.clone()),
                    GuiScaleComponent::new(180, 30),
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(40.0),
                            margin: UiRect::vertical(Val::Px(4.0)),
                            ..Default::default()
                        },
                        background_color: Color::RED.with_a(0.10).into(),
                        ..Default::default()
                    },
                ))
                .set_parent(outer_node)
                .id();

            status::StatusNodeComponent::build(&server, server_entity, world);
            icon::IconNodeComponent::build(&server, server_entity, world);
            version::VersionNodeComponent::build(&server, server_entity, world);
            players::PlayersNodeComponent::build(&server, server_entity, world);
            title::TitleNodeComponent::build(&server, server_entity, world);
            connection::ConnectionNodeComponent::build(&server, server_entity, world);
            ping::PingNodeComponent::build(&server, server_entity, world);
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct ServerListIndex(pub usize);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Resource)]
pub struct ShouldRebuildServers(pub bool);

impl ServersNodeComponent {
    /// Despawn all ServersNodeComponents and rebuild them
    fn rebuild(world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Rebuilding ServersNodeComponent");

        *world.resource_mut::<ShouldRebuildServers>() = ShouldRebuildServers(false);

        let entities = world
            .query_filtered::<Entity, With<ServersNodeComponent>>()
            .iter(world)
            .collect::<Vec<_>>();

        entities.into_iter().for_each(|entity| {
            world.entity_mut(entity).despawn_recursive();
        });

        let entity = world
            .query_filtered::<Entity, With<MultiplayerCenterNodeComponent>>()
            .single(world);
        Self::build(entity, world);
    }

    fn update_statuses(
        query: Query<(Entity, &ServersNodeComponent)>,
        mut events: EventWriter<StatusRequest<DefaultVersion>>,
    ) {
        query.for_each(|(entity, ServersNodeComponent(address))| {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Updating Status: {address}");

            events.send(StatusRequest::<DefaultVersion>::new_with(
                Some(entity),
                address.clone(),
            ));
        });
    }
}
