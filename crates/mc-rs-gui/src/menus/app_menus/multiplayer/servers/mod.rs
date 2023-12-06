use bevy::prelude::*;
use compact_str::CompactString;
use mc_rs_core::{
    events::StatusRequest,
    sounds::{SoundEvent, SoundEventKind},
    versions::v1_20_0::V1_20_0,
    ResourceLocation,
};
use mc_rs_resourcepack::assets::resourcepacks::AssetFromWorld;

use crate::{
    menus::{
        app_menus::{multiplayer::MultiplayerNodeComponent, states::MainMenuState},
        states::menus::MenuComponentMenusSet,
        traits::{AddMenuResource, MenuComponent},
    },
    resources::{scale::GuiScaleComponent, servers::ServerList},
};

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
            (
                Self::rebuild.run_if(
                    resource_equals(ShouldRebuildServers(true))
                        .and_then(any_with_component::<ServersOuterNodeComponent>()),
                ),
                (
                    ServersNodeButtonComponent::click_sound,
                    ServersNodeButtonComponent::join_server,
                )
                    .run_if(any_with_component::<ServersNodeButtonComponent>()),
            )
                .run_if(in_state(MainMenuState::Multiplayer))
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

        // Add unknown server icon
        let unk_icon = world.get_texture_or_fallback("minecraft:misc/unknown_server");
        world.add_menu_resource(unk_icon.clone().untyped());

        let outer_node = world
            .spawn((
                GuiScaleComponent::new(
                    MultiplayerNodeComponent::MENU_WIDTH,
                    MultiplayerNodeComponent::MENU_WIDTH,
                ),
                ServersOuterNodeComponent,
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

        Self::build_list(outer_node, world);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct ServersOuterNodeComponent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Component)]
pub struct ServerListIndex(pub usize);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct ShouldRebuildServers(pub bool);

impl ServersNodeComponent {
    const SERVER_HEIGHT: u32 = 30;

    /// Despawn all ServersNodeComponents and rebuild them
    fn rebuild(world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Rebuilding ServersNodeComponent");

        *world.resource_mut::<ShouldRebuildServers>() = ShouldRebuildServers(false);

        let entity = world
            .query_filtered::<Entity, With<ServersOuterNodeComponent>>()
            .single(world);

        world.entity_mut(entity).despawn_descendants();
        Self::build_list(entity, world);
    }

    /// Iterate over all servers and build a ServerNodeComponent for each
    fn build_list(parent: Entity, world: &mut World) {
        for (index, server) in world
            .resource::<ServerList>()
            .clone()
            .servers
            .into_iter()
            .enumerate()
        {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Building ServerNode: {}", server.title);

            let server_entity = world
                .spawn((
                    ServerListIndex(index),
                    ServersNodeComponent(server.address.clone()),
                    GuiScaleComponent::new(
                        MultiplayerNodeComponent::MENU_WIDTH,
                        Self::SERVER_HEIGHT,
                    ),
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            margin: UiRect::vertical(Val::Px(4.0)),
                            ..Default::default()
                        },
                        background_color: Color::RED.with_a(0.10).into(),
                        ..Default::default()
                    },
                ))
                .with_children(|node| {
                    node.spawn((
                        ServersNodeButtonComponent,
                        ButtonBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..Default::default()
                            },
                            background_color: Color::NONE.into(),
                            ..Default::default()
                        },
                    ));
                })
                .set_parent(parent)
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

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Component)]
pub struct ServersNodeButtonComponent;

impl ServersNodeButtonComponent {
    fn click_sound(
        query: Query<&Interaction, (Changed<Interaction>, With<Self>)>,
        mut events: EventWriter<SoundEvent>,
    ) {
        if query.iter().any(|int| matches!(int, Interaction::Pressed)) {
            events.send(SoundEvent {
                name: ResourceLocation::new("minecraft:random/click"),
                kind: SoundEventKind::Global,
                position: None,
            })
        }
    }

    #[allow(clippy::type_complexity)]
    fn join_server(
        query: Query<
            (&Parent, &Interaction),
            (
                Changed<Interaction>,
                Without<ServersNodeComponent>,
                With<Self>,
            ),
        >,
        servers: Query<&ServersNodeComponent>,
    ) {
        for (parent, int) in query.iter() {
            if !matches!(int, Interaction::Pressed) {
                continue;
            }

            if let Ok(ServersNodeComponent(address)) = servers.get(**parent) {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Clicked ServerNode: {}", address);
            }
        }
    }
}
