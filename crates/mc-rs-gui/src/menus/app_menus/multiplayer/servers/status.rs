use bevy::prelude::*;
use compact_str::CompactString;
use mc_rs_core::events::StatusResponse;

use crate::{
    menus::states::menus::MenuComponentMenusSet,
    resources::{
        font::{shadows::TextShadow, DefaultTextStyle},
        scale::GuiScaleComponent,
        servers::{ServerItem, ServerList},
    },
};

use super::{ServerListIndex, ServersNodeComponent, ShouldRebuildServers};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct StatusNodeComponent;

impl StatusNodeComponent {
    pub(super) fn setup(app: &mut App) {
        app.add_systems(
            Update,
            Self::status_updater
                .run_if(
                    on_event::<StatusResponse>()
                        .and_then(any_with_component::<ServersNodeComponent>()),
                )
                .in_set(MenuComponentMenusSet),
        );
    }

    pub(super) fn build(server: &ServerItem, parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building StatusNodeComponent");

        let default_style: TextStyle = world.resource::<DefaultTextStyle>().clone().into();

        world
            .spawn((
                StatusNodeComponent,
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .with_children(|node| {
                // Take up the same space as the icon
                node.spawn((
                    GuiScaleComponent::new(
                        ServersNodeComponent::SERVER_HEIGHT,
                        ServersNodeComponent::SERVER_HEIGHT,
                    ),
                    NodeBundle::default(),
                ));

                let status = match server.cached_status {
                    Some(ref status) => status.clone(),
                    None => CompactString::new_inline("Loading..."),
                };

                let text = Text::from_section(status, default_style);
                let text_shadow = TextShadow::create_shadow_text_bundle(text.clone());

                node.spawn(TextBundle {
                    text,
                    z_index: ZIndex::Global(i32::MAX - 256),
                    ..Default::default()
                })
                .with_children(|node| {
                    node.spawn(text_shadow);
                });
            })
            .set_parent(parent);
    }
}

impl StatusNodeComponent {
    fn status_updater(
        query: Query<&ServerListIndex>,
        mut events: EventReader<StatusResponse>,
        mut update: ResMut<ShouldRebuildServers>,
        mut servers: ResMut<ServerList>,
    ) {
        for event in events.read() {
            if let Some(entity) = event.entity {
                let Ok(index) = query.get(entity) else {
                    error!("Failed to get ServerListIndex");
                    return Self::hostname_fallback(event, &mut servers, &mut update);
                };

                let Some(server) = servers.get_mut(index.0) else {
                    error!("Failed to get ServerItem");
                    return Self::hostname_fallback(event, &mut servers, &mut update);
                };

                Self::update_item(event, server, &mut update);
            } else {
                Self::hostname_fallback(event, &mut servers, &mut update);
            }
        }
    }

    fn hostname_fallback(
        event: &StatusResponse,
        servers: &mut ServerList,
        update: &mut ShouldRebuildServers,
    ) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        warn!("StatusResponse falling back to hostname");

        if let Some(server) = servers
            .servers
            .iter_mut()
            .find(|s| s.address == event.hostname)
        {
            Self::update_item(event, server, update);
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Failed to find ServerItem");
        }
    }

    fn update_item(
        event: &StatusResponse,
        server: &mut ServerItem,
        update: &mut ShouldRebuildServers,
    ) {
        // Double check that the hostname matches
        if server.address != event.hostname {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!(
                "Hostname mismatch: {} != {}",
                server.address, event.hostname
            );
            return;
        }

        let status = event.description.to_string().into();
        if let Some(cached) = server.cached_status.as_mut() {
            if *cached != status {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Updating Status: {}", event.hostname);

                *cached = status;
                *update = ShouldRebuildServers(true);
            } else {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Skipping Identical Status: {}", event.hostname)
            }
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Setting Status: {}", event.hostname);

            server.cached_status = Some(status);
            *update = ShouldRebuildServers(true);
        }
    }
}
