use std::io::Cursor;

use base64::{engine::general_purpose::STANDARD, Engine};
use bevy::prelude::*;
use image::io::Reader as ImageReader;
use mc_rs_core::events::StatusResponse;
use mc_rs_resourcepack::assets::resourcepacks::AssetFromWorld;

use crate::{
    menus::states::menus::MenuComponentMenusSet,
    resources::{
        scale::GuiScaleComponent,
        servers::{ServerItem, ServerList},
    },
};

use super::{ServerListIndex, ServersNodeComponent, ShouldRebuildServers};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct IconNodeComponent;

impl IconNodeComponent {
    const ICON_SIZE: u32 = 24;

    pub(super) fn setup(app: &mut App) {
        app.add_systems(
            Update,
            Self::icon_updater
                .run_if(
                    on_event::<StatusResponse>()
                        .and_then(any_with_component::<ServersNodeComponent>()),
                )
                .in_set(MenuComponentMenusSet),
        );
    }

    pub(super) fn build(server: &ServerItem, parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building IconNodeComponent");

        let mut image_handle: Option<Handle<Image>> = None;

        // Try to decode the base64 icon
        if let Some(base64) = &server.cached_icon {
            match STANDARD.decode(base64.trim_start_matches("data:image/png;base64,")) {
                Err(err) => {
                    error!("Failed to decode base64 icon: {err}");
                }
                Ok(bytes) => {
                    let Ok(reader) = ImageReader::new(Cursor::new(bytes)).with_guessed_format()
                    else {
                        error!("Failed to guess image format");
                        return;
                    };

                    let Ok(image) = reader.decode() else {
                        error!("Failed to decode image");
                        return;
                    };

                    let image = Image::from_dynamic(image, true);
                    let handle = world.resource_mut::<Assets<Image>>().add(image);
                    image_handle = Some(handle);
                }
            }
        }

        // Fallback to the unknown server icon
        if image_handle.is_none() {
            // If there is no unknown server icon, fallback to the fallback icon
            let handle = world
                .get_texture_or_fallback("minecraft:misc/unknown_server")
                .clone();
            image_handle = Some(handle);
        }

        world
            .spawn((
                GuiScaleComponent::new(
                    ServersNodeComponent::SERVER_HEIGHT,
                    ServersNodeComponent::SERVER_HEIGHT,
                ),
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .with_children(|node| {
                node.spawn((
                    GuiScaleComponent::new(Self::ICON_SIZE, Self::ICON_SIZE),
                    IconNodeComponent,
                    ImageBundle {
                        image: image_handle.unwrap().into(),
                        ..Default::default()
                    },
                ));
            })
            .set_parent(parent);
    }

    fn icon_updater(
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
        updater: &mut ShouldRebuildServers,
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

        // If the icon is not valid, skip it
        if let Some(base64) = &event.favicon {
            if let Err(err) = STANDARD.decode(base64.trim_start_matches("data:image/png;base64,")) {
                #[cfg(any(debug_assertions, feature = "debug"))]
                error!("Skipping Icon {}: {err}", event.hostname);
                return;
            }
        }

        if server.cached_icon != event.favicon {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Updating Icon: {}", event.hostname);

            server.cached_icon = event.favicon.clone();
            *updater = ShouldRebuildServers(true);
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Skipping Identical Icon: {}", event.hostname);
        }
    }
}
